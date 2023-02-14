use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    let my_article: Article = Article {
        article: String::from("hwo to work with json in rust"),
        author: String::from("alice"),
        paragraphs: vec![
            Paragraph {
                name: String::from("first sentence"),
            },
            Paragraph {
                name: String::from("body of paragraph"),
            },
            Paragraph {
                name: String::from("end of the paragraph"),
            },
        ],
    };

    let my_json = serde_json::to_string_pretty(&my_article).unwrap();
    println!("The JSON is: \n{}", my_json);
}
