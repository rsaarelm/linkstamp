use std::io::{prelude::*, stdin};

use askama::Template;
use clap::Parser;
use derive_deref::Deref;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

const EPOCH: &str = "1970-01-01T00:00:00Z";

fn normalize_date(partial_date: &str) -> String {
    let mut ret = partial_date.to_string();
    let skip = ret.chars().count();
    for c in EPOCH.chars().skip(skip) {
        ret.push(c);
    }
    ret
}

const FEED_LINK_COUNT: usize = 30;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(default)]
struct LinkData {
    title: String,
    added: String,
    date: String,
    /// If it's available, `added`, otherwise `date`.
    feed_date: String,
    tags: Vec<String>,
    notes: String,
    id: String,

    /// Further parts if it's a series of posts.
    sequence: Vec<String>,
}

#[derive(Clone, Default, Template)]
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

    let mut links: IndexMap<String, LinkData> = if args.toml {
        toml::from_str(&buffer).expect("Invalid TOML")
    } else {
        idm::from_str(&buffer).expect("Invalid IDM")
    };

    for (url, data) in links.iter_mut() {
        // Compute IDs from URLs.
        let digest = md5::compute(url);
        let id = base64_url::encode(&digest.0);
        data.id = id;

        // Generate dummy titles from URLs if needed
        if data.title.trim().is_empty() {
            data.title = url.clone();
        }

        // Generate ATOM feed sorting dates
        data.feed_date = if !data.added.is_empty() {
            normalize_date(&data.added)
        } else if !data.date.is_empty() {
            normalize_date(&data.date)
        } else {
            EPOCH.to_owned()
        };

        // Mark PDF links
        if url.ends_with(".pdf")
            && (!data.title.ends_with(".pdf") && !data.title.ends_with(" (pdf)"))
        {
            data.title.push_str(" (pdf)");
        }
    }

    // Assume date values can be sorted lexically so the last in order is newest.
    let updated = links
        .iter()
        .map(|(_, data)| &data.feed_date)
        .max()
        .cloned()
        .unwrap_or(EPOCH.to_owned());

    let mut links = Links {
        links,
        updated,
        title: args.title,
        site_url: args.site_url.unwrap_or_default(),
    };

    // Reverse the list, make the newest link show up on top on the links
    // page.
    links.links.reverse();

    if args.dump_json {
        print!(
            "{}",
            serde_json::to_string(&links.links).expect("Failed to output JSON")
        );
        return;
    }

    if args.feed {
        // Creating an ATOM feed. Sort links by when they were added, only
        // show the last 30 links.
        links
            .links
            .sort_by(|k1, v1, k2, v2| (&v1.feed_date, k1).cmp(&(&v2.feed_date, k2)));
        if links.links.len() > FEED_LINK_COUNT {
            links.links = links.links.split_off(links.links.len() - FEED_LINK_COUNT);
        }
        println!("{}", Feed(links).render().unwrap());
    } else {
        println!("{}", links.render().unwrap());
    }
}
