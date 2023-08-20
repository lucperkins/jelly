use notify::{Event, Watcher};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    path::PathBuf,
    process::exit,
    sync::mpsc::channel,
};
use tempfile::TempDir;

use crate::error::Error;

use super::build;

pub fn serve(source: PathBuf) -> Result<(), Error> {
    let out = TempDir::new()?; // TODO: make this a temporary directory

    tracing::debug!("adding Ctrl-C handler");

    ctrlc::set_handler(move || {
        tracing::debug!("detected Ctrl-C; exiting");
        exit(0);
    })
    .expect("something went wrong while quitting");

    tracing::debug!("added Ctrl-C handler");

    tracing::debug!("serving docs; writing output to {:?}", out.as_ref());

    // Initial site build
    build(source.clone(), out.as_ref().to_owned())?;

    tracing::debug!("successfully built site");

    let bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080); // TODO: make this configurable
    if TcpListener::bind(bind_address).is_err() {
        return Err(Error::PortNotFree(bind_address.to_string())); // TODO: improve this error
    }

    tracing::debug!("successfully bound to {}", bind_address);

    tracing::debug!("setting up watcher on {:?}", source);

    let (_tx, rx) = channel::<Event>();
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(Event { kind, .. }) => {
            use notify::EventKind::*;

            match kind {
                Create(_) | Modify(_) | Remove(_) => {
                    tracing::debug!("got a {:?} event", kind);
                }
                _ => {
                    tracing::debug!("got some other kind of event: {:?}", kind);
                }
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    tracing::debug!("set up watcher on {:?}", source);

    watcher.watch(source.as_path(), notify::RecursiveMode::Recursive)?; // Why doesn't this watch?

    if let Err(e) = rx.recv() {
        out.close()?;
        tracing::debug!("error encountered from listener: {}", e);
        return Err(Error::Recv(e));
    }

    tracing::debug!("quitting");

    Ok(())
}
