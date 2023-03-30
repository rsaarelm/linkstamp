use std::io::{prelude::*, stdin};

use serde::{Deserialize, Serialize};
use askama::Template;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
struct LinkData {
    url: String,
    added: String,
    tags: Vec<String>,
    description: String,
}

#[derive(Template)]
#[template(path = "links.html")]
struct Links {
    links: Vec<(String, LinkData)>
}

fn main() {
    let mut buffer = String::new();
    stdin().lock().read_to_string(&mut buffer).unwrap();

    println!("{}",
        Links { links: idm::from_str(&buffer).expect("Invalid input") }.render().unwrap());
}
