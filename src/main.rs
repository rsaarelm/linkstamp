use std::io::{prelude::*, stdin};

use askama::Template;
use clap::Parser;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

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
    links: IndexMap<String, LinkData>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Parse input as TOML.
    #[arg(long)]
    toml: bool,
}

fn main() {
    let args = Args::parse();

    let mut buffer = String::new();
    stdin().lock().read_to_string(&mut buffer).unwrap();

    if args.toml {
        println!(
            "{}",
            Links {
                links: toml::from_str(&buffer).expect("Invalid TOML")
            }
            .render()
            .unwrap()
        );
    } else {
        println!(
            "{}",
            Links {
                links: idm::from_str(&buffer).expect("Invalid IDM")
            }
            .render()
            .unwrap()
        );
    }
}
