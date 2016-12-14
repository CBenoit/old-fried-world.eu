use iron::prelude::*;
use iron::status;

use hbs::Template;

use router;

// just some tests

pub fn hello_world(_: &mut Request) -> IronResult<Response> {
    use iron::mime::Mime;

    Ok(Response::with(("text/html".parse::<Mime>().unwrap(),
                        status::Ok,
                        "<!DOCTYPE html><html><head></head><body><h1>Hello World</h1></body></html>")))
}

pub fn locales_test(req: &mut Request) -> IronResult<Response> {
    use std::collections::HashMap;
    use l20n;

    let lang = req.extensions.get::<router::Router>().unwrap()
        .find("lang").unwrap_or("en");

    let mut locale = l20n::Locale::new();
    match lang {
        "fr" => locale.add_resource(include_str!("../locales/example.fr.ftl")).unwrap(),
        _ => locale.add_resource(include_str!("../locales/example.en-US.ftl")).unwrap(),
    }

    let data: HashMap<String, String> = locale.localize().unwrap();

    Ok(Response::with((status::Ok, Template::new("tests/locales_test", data))))
}

