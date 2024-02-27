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

#[component]
fn App() -> impl IntoView {
    let (card_result, set_card_result) = create_signal::<CardResult>(CardResult::Unloaded);
    let (rulings, set_rulings) = create_signal::<Vec<Ruling>>(Vec::new());

    let input_element: NodeRef<html::Input> = create_node_ref();

    let action = create_action(
        move |input: &(WriteSignal<CardResult>, String, WriteSignal<Vec<Ruling>>)| {
            let input = input.clone();
            async move {
                let card_result = get_card_result(&input.1).await;

                let rulings = match &card_result {
                    CardResult::Success(card) => get_rulings(card).await,
                    _ => Vec::new(),
                };

                input.0.set(card_result);
                input.2.set(rulings);
            }
        },
    );

    view! {
        <div id="input-container">
            <input
                on:keypress=move |e| {
                    if e.key_code() == 13 {
                        action
                            .dispatch((
                                set_card_result,
                                input_element.get().unwrap().value(),
                                set_rulings,
                            ))
                    }
                }

                node_ref=input_element
                type="search"
                name="search"
                id="card-input"
                placeholder="Black Lotus"
            />
        </div>
        <CardAndRulingsContainer card_result rulings/>
    }
}

#[component]
fn CardAndRulingsContainer(
    card_result: ReadSignal<CardResult>,
    rulings: ReadSignal<Vec<Ruling>>,
) -> impl IntoView {
    view! {
        <div id="card-and-rulings-container">
            <CardContainer card_result/>
            <hr/>
            <Rulings rulings/>
        </div>
    }
}

#[component]
fn CardContainer(card_result: ReadSignal<CardResult>) -> impl IntoView {
    view! {
        <div id="card-container">
            {move || {
                match card_result.get() {
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
fn Rulings(rulings: ReadSignal<Vec<Ruling>>) -> impl IntoView {
    leptos::view! {
        <div id="rulings-container">
            <ul id="rulings">
                <For
                    each=rulings
                    key=|ruling| ruling.comment.clone()
                    children=move |ruling| {
                        view! { <li>{ruling.comment}</li> }
                    }
                />
            </ul>
        </div>
    }
}
