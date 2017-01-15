use std::io;

use iron::prelude::*;
use iron::status;

use hbs::Template;

use data;
use router;

// the handlers

pub fn index(_: &mut Request) -> IronResult<Response> {
    use std::collections::BTreeMap;

    let data: BTreeMap<String, String> = BTreeMap::new();
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

pub fn get_root_page(req: &mut Request) -> IronResult<Response> {
    let ref page_name = req.extensions.get::<router::Router>().unwrap().find("page").unwrap_or("no query");

    match data::make_data_from_markdown(page_name) {
        Ok(data) => Ok(Response::with((status::Ok, Template::new("page", data)))),
        Err(e) => {
            let status = match e.kind() {
                io::ErrorKind::NotFound => status::NotFound,
                io::ErrorKind::PermissionDenied => status::Forbidden,
                _ => status::InternalServerError,
            };

            Err(IronError::new(e, status))
        }
    }
}

pub fn get_category_page(req: &mut Request) -> IronResult<Response> {
    use rustc_serialize::json::ToJson;

    let ref category = req.extensions.get::<router::Router>().unwrap().find("category").unwrap_or("no query");
    let ref page_name = req.extensions.get::<router::Router>().unwrap().find("page").unwrap_or("no query");

    match data::make_data_from_markdown(&format!("{}/{}", category, page_name)[..]) {
        Ok(mut data) => {
            data.insert("category".to_owned(), category.to_json()); // this is injected as the article css class.
            Ok(Response::with((status::Ok, Template::new("page", data))))
        }
        Err(e) => {
            let status = match e.kind() {
                io::ErrorKind::NotFound => status::NotFound,
                io::ErrorKind::PermissionDenied => status::Forbidden,
                _ => status::InternalServerError,
            };

            Err(IronError::new(e, status))
        }
    }
}

