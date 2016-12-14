extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate env_logger;
extern crate l20n;
extern crate mount;
extern crate hoedown;
#[cfg(feature = "serves_static")]
extern crate staticfile;
extern crate urlencoded;

pub mod config;
pub mod data;
pub mod handlers;
pub mod middlewares;
pub mod tests;
pub mod private;

use iron::prelude::*;

use hbs::HandlebarsEngine;
use hbs::DirectorySource;

#[cfg(feature = "serves_static")]
fn serves_static(mount: &mut mount::Mount) {
    use staticfile::Static;
    use std::path::Path;

    mount.mount("/static", Static::new(Path::new("./static/")));
}

#[cfg(not(feature = "serves_static"))]
fn serves_static(_mount: &mut mount::Mount) {}

fn main() {
    env_logger::init().unwrap();

    let mut hbse = HandlebarsEngine::new();

    // add a directory source, all files with .hbs suffix will be loaded as template
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    // load templates from all registered sources
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    // create the router
    let mut router = router::Router::new();
    config::route::configure_routes(&mut router);

    // create the chain
    let mut chain = Chain::new(router);
    chain.link_after(middlewares::Custom404);
    chain.link_after(hbse);

    // create mounting middleware.
    let mut mount = mount::Mount::new();
    mount.mount("/", chain);

    // serves static file (optional feature)
    serves_static(&mut mount);

    // create and start the server
    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}

