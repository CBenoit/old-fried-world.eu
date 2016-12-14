use iron::prelude::*;
use iron::AfterMiddleware;
use iron::status;

use hbs::Template;

// custom 404 not found.
pub struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        use std::collections::BTreeMap;

        // It is also possible to us that (handles only NoRoute errors from router):
        //use router::NoRoute;
        //if err.error.is::<NoRoute>() {
        if let Some(status::NotFound) = err.response.status {
            let data: BTreeMap<String, String> = BTreeMap::new();
            Ok(Response::with((status::NotFound, Template::new("404", data))))
        } else {
            Err(err)
        }
    }
}

