use serde::{Deserialize, Serialize};

const CORE_FUZZY_URL: &str = "https://api.scryfall.com/cards/named?fuzzy=";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub name: String,
    pub image_uris: Images,
    pub type_line: String,
    pub oracle_text: String,
    pub rulings_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardError {
    details: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rulings {
    data: Vec<Ruling>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ruling {
    comment: String,
}

#[derive(Serialize, Deserialize)]
struct HtmlBuilder {
    name: String,
    image_uri: String,
    type_line: String,
    oracle_text: String,
    rulings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Images {
    pub normal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CardResult {
    Success(Card),
    NoMatch,
    TooManyMatches,
}

#[derive(Deserialize)]
struct CardQuery {
    search: String,
}

pub async fn get_card_result(name: &str) -> CardResult {
    let response = reqwest::get(CORE_FUZZY_URL.to_string() + name)
        .await
        .expect("Should return response.");

    match response.status() {
        reqwest::StatusCode::OK => CardResult::Success(
            response
                .json::<Card>()
                .await
                .expect("Response should be parseable to Card."),
        ),
        _ => {
            if response
                .json::<CardError>()
                .await
                .expect("Response error should have details field.")
                .details
                .starts_with("Too many")
            {
                CardResult::TooManyMatches
            } else {
                CardResult::NoMatch
            }
        }
    }
}
