use std::io::{prelude::*, stdin};

use askama::Template;
use clap::Parser;
use derive_deref::Deref;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
struct LinkData {
    title: String,
    added: String,
    date: String,
    tags: Vec<String>,
    description: String,
}

#[derive(Default, Template)]
#[template(path = "links.html")]
struct Links {
    site_url: String,
    title: String,
    updated: String,
    links: IndexMap<String, LinkData>,
}

#[derive(Default, Template, Deref)]
#[template(path = "feed.xml")]
struct Feed(Links);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Parse input as TOML.
    #[arg(long)]
    toml: bool,

    /// Emit ATOM feed.
    #[arg(long)]
    feed: bool,

    /// Dump bookmarks in JSON
    #[arg(long)]
    dump_json: bool,

    /// Title of your page.
    #[arg(long, default_value = "Links page")]
    title: String,

    /// URL of your links website.
    #[arg(long)]
    site_url: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut buffer = String::new();
    stdin().lock().read_to_string(&mut buffer).unwrap();

    let links: IndexMap<String, LinkData> = if args.toml {
        toml::from_str(&buffer).expect("Invalid TOML")
    } else {
        idm::from_str(&buffer).expect("Invalid IDM")
    };

    // Assume date values can be sorted lexically so the last in order is newest.
    let updated = links
        .iter()
        .map(|(_, data)| &data.added)
        .max()
        .cloned()
        .unwrap_or("1970-01-01T00:00:00Z".to_owned());

    let links = Links {
        links,
        updated,
        title: args.title,
        site_url: args.site_url.unwrap_or_default(),
    };

    if args.dump_json {
        print!("{}", serde_json::to_string(&links.links).expect("Failed to output JSON"));
        return;
    }

    if args.feed {
        println!("{}", Feed(links).render().unwrap());
    } else {
        println!("{}", links.render().unwrap());
    }
}
