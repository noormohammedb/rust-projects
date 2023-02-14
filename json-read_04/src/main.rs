use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let my_json = r#"
    {
        "article": "how to work with json in rust",
        "author": "alice",
        "paragraph": [
            {
            "name": "starting sentence"
            },
            {
            "name": "body of the pragraph"
            },
            {
            "name": "end of the paragraph"
            }
        ]
    }"#;
    // println!("{}", my_json);
    let parse: Article = read_json_typed(my_json);
    println!(
        "\n\nThe name of the first paragraph is : {}",
        parse.paragraph[0].name
    );
}

fn read_json_typed(raw_json_str: &str) -> Article {
    // serde_json::from_str(raw_json_str).unwrap()

    match serde_json::from_str(raw_json_str) {
        Ok(decoded_json) => decoded_json,
        Err(json_decod_error) => {
            eprintln!("Error: {}", json_decod_error);
            panic!("Error: {}", json_decod_error)
        }
    }
}
