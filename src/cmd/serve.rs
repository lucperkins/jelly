use super::build;
use crate::error::Error;
use notify::{Event, Watcher};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    path::PathBuf,
    process::exit,
    str::FromStr,
    sync::mpsc::channel,
    thread,
};
use tempfile::TempDir;
use tiny_http::Request;
use tracing::{debug, error, info};

#[derive(Clone)]
struct Site {
    source: PathBuf,
    out: PathBuf,
}

impl Site {
    fn new(source: PathBuf, out: PathBuf) -> Self {
        Self { source, out }
    }

    fn build(&self) {
        debug!("building site");

        if let Err(e) = build(&self.source, &self.out) {
            error!("error building site: {e}");
        }
    }
}

struct FileServer {
    root: PathBuf,
    address: SocketAddr,
}

impl FileServer {
    fn new(root: PathBuf, address: SocketAddr) -> Self {
        Self { root, address }
    }

    fn serve(&self) -> Result<(), Error> {
        let server = tiny_http::Server::http(self.address).expect("couldn't start server"); // TODO: don't use expect here

        for req in server.incoming_requests() {
            self.handle_files(req)?;
        }

        Ok(())
    }

    fn handle_files(&self, req: Request) -> Result<(), Error> {
        // Borrowed from Cobalt
        let mut req_path = req.url().to_string();
        if let Some(position) = req_path.rfind('?') {
            req_path.truncate(position);
        }

        let path = self.root.to_path_buf().join(&req_path[1..]);
        let serve_path = if path.is_file() {
            path
        } else {
            path.join("index.html")
        };
        if serve_path.exists() {
            let file = std::fs::File::open(&serve_path).expect("failed to find file");
            let mut response = tiny_http::Response::from_file(file);
            if let Some(mime) = mime_guess::MimeGuess::from_path(&serve_path).first_raw() {
                let content_type = format!("Content-Type:{}", mime);

                let content_type =
                    tiny_http::Header::from_str(&content_type).expect("formatted correctly");
                response.add_header(content_type);
            }
            req.respond(response).expect("can't respond");
        } else {
            req.respond(
                tiny_http::Response::from_string("<h1><center>404: Page not found</center></h1>")
                    .with_status_code(404)
                    .with_header(
                        tiny_http::Header::from_str("Content-Type: text/html")
                            .expect("formatted correctly"),
                    ),
            )
            .expect("couldn't respond with 404");
        }

        Ok(())
    }
}

pub fn serve(source: PathBuf, open: bool, port: u16) -> Result<(), Error> {
    let tmp_dir = TempDir::new()?; // TODO: make this a temporary directory
    let out_path = tmp_dir.as_ref().to_owned();

    let bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port); // TODO: make this configurable
    if TcpListener::bind(bind_address).is_err() {
        return Err(Error::PortNotFree(bind_address.to_string())); // TODO: improve this error
    }

    info!("listening on port {port}");

    debug!("adding Ctrl-C handler");

    ctrlc::set_handler(move || {
        debug!("detected Ctrl-C; exiting");
        exit(0);
    })
    .expect("something went wrong while quitting");

    debug!("added Ctrl-C handler");

    debug!("serving docs; writing output to {:?}", out_path);

    let site = Site::new(source.clone(), out_path.clone());

    // Initial site build
    site.build();

    debug!("creating file server");

    let file_server = FileServer::new(out_path, bind_address);

    debug!("starting file server");

    thread::spawn(move || {
        file_server.serve().expect("http server error");
    });

    if open {
        open::that("http://localhost:8080")?;
    }

    debug!("successfully built site");

    debug!("successfully bound to {}", bind_address);

    debug!("setting up watcher on {:?}", source);

    let (_tx, rx) = channel::<Event>();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(Event { kind, .. }) => {
            use notify::EventKind::*;

            match kind {
                Create(_) | Modify(_) | Remove(_) => {
                    debug!("got a {kind:?} event");

                    site.build();
                }
                _ => {
                    debug!("got some other kind of event: {:?}", kind);
                }
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    debug!("set up watcher on {:?}", source);

    #[cfg(not(feature = "dev-handlebar-templates"))]
    let watch_paths = vec![source, "src/template".into()];

    #[cfg(feature = "dev-handlebar-templates")]
    let watch_paths = vec![source];

    for path in watch_paths {
        watcher.watch(path.as_path(), notify::RecursiveMode::Recursive)?;
    }

    if let Err(e) = rx.recv() {
        tmp_dir.close()?;
        tracing::debug!("error encountered from listener: {}", e);
        return Err(Error::Recv(e));
    }

    debug!("quitting");

    Ok(())
}
