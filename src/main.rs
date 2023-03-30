use std::io::{prelude::*, stdin};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
struct LinkData {
    url: String,
    added: String,
    tags: Vec<String>,
    description: String,
}

fn main() {
    let mut buffer = String::new();
    stdin().lock().read_to_string(&mut buffer).unwrap();

    let bookmarks: Vec<(String, LinkData)> = idm::from_str(&buffer).expect("Invalid input");

    // TODO 2023-03-30 Choose and use a templating engine for the generated HTML page.

    println!("<ul>");
    for (name, data) in bookmarks {
        println!("<li>");
        println!(
            "<a href='{}'>{}</a><br/>",
            data.url,
            html_escape::encode_safe(&name)
        );
        for tag in data.tags {
            print!("<a class='tag' href='#tags={}'>{}</a>&nbsp;", tag, html_escape::encode_safe(&tag));
                print!(" ");
        }
        println!("<br/>");
        if !data.description.is_empty() {
            println!("<p>{}</p>", data.description);
        }
        println!("</li>");
    }
    println!("</ul>");
}
