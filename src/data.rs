use std::collections::BTreeMap;

use std::io;
use std::fs;

use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;

use hoedown::Html;
use hoedown::renderer::html::Flags;
use hoedown::Markdown;
use hoedown::Render;

static MARKDOWN_PATH: &'static str = "markdown/";

// TODO: add a data caching system for rendered markdown.

pub fn make_data_from_markdown(page_name: &str) -> io::Result<BTreeMap<String, Json>> {
    let mut page_path = MARKDOWN_PATH.to_owned();
    page_path += page_name;
    page_path += ".md";

    let mut data = BTreeMap::new();
    match fs::File::open(page_path) {
        Ok(file) => {
            let markdown = Markdown::read_from(file);
            let mut html = Html::new(Flags::empty(), 0);
            data.insert("content".to_owned(), html.render(&markdown).to_str().unwrap().to_json());
            Ok(data)
        }
        Err(e) => {
            Err(e)
        }
    }
}

