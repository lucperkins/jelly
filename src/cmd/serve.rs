use super::build;
use crate::error::Error;
use indoc::{formatdoc, indoc};
use notify::{Event, Watcher};
use std::{
    fmt::Debug,
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
use ws::{Message, Sender, WebSocket};

const LIVE_RELOAD_JS: &str = include_str!("../../assets/livereload.js");

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
        let server = tiny_http::Server::http(self.address)?;

        for req in server.incoming_requests() {
            self.handle_files(req)?;
        }

        Ok(())
    }

    fn handle_files(&self, req: Request) -> Result<(), Error> {
        // Borrowed from Cobalt
        let mut req_path = req.url().to_string();

        if req_path.starts_with("/livereload.js") {
            handle_error(req.respond(
                tiny_http::Response::from_string(LIVE_RELOAD_JS).with_header(
                    tiny_http::Header::from_str("Content-Type:text/javascript").unwrap(),
                ),
            ));
        } else {
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
                let file = std::fs::File::open(&serve_path)?;
                let mut response = tiny_http::Response::from_file(file);
                if let Some(mime) = mime_guess::MimeGuess::from_path(&serve_path).first_raw() {
                    let content_type = format!("Content-Type:{}", mime);
                    let content_type = tiny_http::Header::from_str(&content_type).unwrap();
                    response.add_header(content_type);
                }
                req.respond(response)?;
            } else {
                req.respond(
                    tiny_http::Response::from_string(
                        "<h1><center>404: Page not found</center></h1>",
                    )
                    .with_status_code(404)
                    .with_header(tiny_http::Header::from_str("Content-Type: text/html").unwrap()),
                )?;
            }
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
    })?;

    debug!("added Ctrl-C handler");

    debug!("serving docs; writing output to {:?}", out_path);

    let site = Site::new(source.clone(), out_path.clone());

    // Initial site build
    site.build();

    if open {
        open::that(format!("http://localhost:{port}"))?;
    }

    debug!("successfully built site");

    debug!("successfully bound to {}", bind_address);

    debug!("setting up broadcaster server for live reload");

    let broadcaster = {
        thread::spawn(move || {
            debug!("creating file server");

            let file_server = FileServer::new(out_path, bind_address);

            debug!("starting file server");

            handle_error(file_server.serve());
        });

        let ws_server = WebSocket::new(|output: Sender| {
            move |msg: Message| {
                if msg.into_text()?.contains("\"hello\"") {
                    return output.send(Message::text(indoc! {r#"
                        {
                          "command": "hello",
                          "protocols": [ "http://livereload.com/protocols/official-7" ],
                          "serverName": "Jelly"
                        }
                    "#}));
                }
                Ok(())
            }
        })
        .map_err(Box::new)?;

        let broadcaster = ws_server.broadcaster();

        // TODO: make WS address configurable
        let ws_server = ws_server.bind("127.0.0.1:8999").map_err(Box::new)?;

        thread::spawn(move || {
            handle_error(ws_server.run());
        });

        broadcaster
    };

    debug!("setting up watcher on {:?}", source);

    let (_tx, rx) = channel::<Event>();
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(Event { kind, paths, .. }) => {
            use notify::EventKind::*;

            match kind {
                Create(_) | Modify(_) | Remove(_) => {
                    debug!("got a {kind:?} event");

                    debug!("rebuilding site");

                    site.build();

                    debug!("successfully rebuilt site");

                    debug!("broadcaster sending message");

                    if let Some(path) = paths.first() {
                        handle_error(broadcaster.send(live_reload_message(path)));
                    }

                    debug!("broadcaster sent message");
                }
                _ => {
                    debug!("got some other kind of event: {:?}", kind);
                }
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    debug!("set up watcher on {:?}", source);

    #[cfg(not(feature = "dev-handlebars-templates"))]
    let watch_paths = vec![source];

    #[cfg(feature = "dev-handlebars-templates")]
    let watch_paths = vec![source, "assets/templates".into()];

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

fn live_reload_message(path: &PathBuf) -> String {
    formatdoc! {r#"
        {{
            "command": "reload",
            "path": {path:?},
            "originalPath": "",
            "liveCSS": true,
            "liveImg": true,
            "protocol": ["http://livereload.com/protocols/official-7"]
        }}
    "#}
}

// Handler for errors inside spawns and move blocks and such
fn handle_error<T, E: Debug>(result: Result<T, E>) {
    if let Err(e) = result {
        error!("{e:?}");
    }
}
