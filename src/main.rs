mod card;

use leptos::*;
use leptos_router::*;

use card::{get_card_result, get_rulings, Card, CardResult, Images, Ruling};

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|| {
        view! {
            <main>
                <div id="sel-call-container">
                    <h1>Select your card</h1>
                </div>
                <Router>
                    <Routes>
                        <Route path="/" view=App/>
                    </Routes>
                </Router>
            </main>
        }
    })
}

#[derive(Params, PartialEq, Clone)]
struct NameSearch {
    q: Option<String>,
}

#[component]
fn App() -> impl IntoView {
    let query = use_query::<NameSearch>();

    let resource = create_resource(query, |q| async move {
        let namesearch = q.clone().unwrap();

        let Some(q) = namesearch.q else {
            return (CardResult::Unloaded, Vec::new());
        };

        get_card_result_and_rulings(&q).await
    });

    view! {
        <div id="input-container">
            <Form method="GET" action="">
                <input type="search" name="q" id="card-input" placeholder="Black Lotus"/>
            </Form>
        </div>

        {move || {
            if let Some((card_result, rulings)) = resource.get() {
                view! { <CardAndRulingsContainer card_result rulings/> }
            } else {
                view! {
                    <CardAndRulingsContainer card_result=CardResult::Unloaded rulings=Vec::new()/>
                }
            }
        }}
    }
}

#[component]
fn CardAndRulingsContainer(card_result: CardResult, rulings: Vec<Ruling>) -> impl IntoView {
    view! {
        <div id="card-and-rulings-container">
            <CardContainer card_result/>
            <hr/>
            <Rulings rulings/>
        </div>
    }
}

#[component]
fn CardContainer(card_result: CardResult) -> impl IntoView {
    view! {
        <div id="card-container">
            {move || {
                match card_result.clone() {
                    CardResult::Success(
                        Card { name, image_uris: Images { normal }, type_line, oracle_text, .. },
                    ) => {
                        let name = &name;
                        view! {
                            <img id="card-img" src=normal alt=name width="265px" height="370px"/>
                            <ul id="card-information">
                                <li>{name}</li>
                                <li>{type_line}</li>
                                <li id="ci-preloaded">{oracle_text}</li>
                            </ul>
                        }
                    }
                    CardResult::NoMatch => {
                        view! {
                            <img
                                id="card-img"
                                src="/princess.jpg"
                                alt="Your princess is in another castle"
                                width="265px"
                                height="370px"
                            />
                            <ul id="card-information">
                                <li>"No matches found"</li>
                            </ul>
                        }
                    }
                    CardResult::TooManyMatches => {
                        view! {
                            <img
                                id="card-img"
                                src="/masterskywalker.jpg"
                                alt="Master Skywalker"
                                width="265px"
                                height="370px"
                            />
                            <ul id="card-information">
                                <li>"Too many matches"</li>
                            </ul>
                        }
                    }
                    CardResult::Unloaded => {
                        view! {
                            <img
                                id="card-img"
                                src="https://cards.scryfall.io/normal/front/b/d/bd8fa327-dd41-4737-8f19-2cf5eb1f7cdd.jpg?1614638838"
                                alt="Liliana"
                                width="265px"
                                height="370px"
                            />
                            <ul id="card-information">
                                <li>Black Lotus</li>
                                <li>Artifact</li>
                                <li id="ci-preloaded">
                                    "{T}" , Sacrifice Black Lotus: Add three mana of any
                                    one color.
                                </li>
                            </ul>
                        }
                    }
                }
            }}

        </div>
    }
}

#[component]
fn Rulings(rulings: Vec<Ruling>) -> impl IntoView {
    leptos::view! {
        <div id="rulings-container">
            <ul id="rulings">
                {rulings.into_iter().map(|ruling| view! { <li>{ruling.comment}</li> }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

async fn get_card_result_and_rulings(name: &str) -> (CardResult, Vec<Ruling>) {
    let card_result = get_card_result(name).await;

    let rulings = match &card_result {
        CardResult::Success(card) => get_rulings(card).await,
        _ => Vec::new(),
    };

    (card_result, rulings)
}
