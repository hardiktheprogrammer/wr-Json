use error_chain::error_chain;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]

struct Article {
    article: String,
    author: String,
    paragraph: vec<Paragraph>, //slices , paragraph are separated by spaces
}

fn main() {
    let article: Article = Article {
        article: String::from("why Rust is so stupid"),
        author: String::from("rust foundation "),
        paragraph: vec![ // arrays of paragraphs
            Paragraph {  // each vector is a paragraph element
                name:String::from("Rust is must!")

            },
            paragraph {
                    name:String::from("Rust is not must Do Javascript or either TypeScript!")

            paragraph {
                name:String::from("Be ")
            }
            
        ],
    };
}
