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
}
