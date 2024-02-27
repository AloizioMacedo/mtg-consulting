mod card;

use leptos::*;
use leptos_router::*;

use card::{get_card_result, Card, CardResult, Images};

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
    let (card_result, set_card_result) = create_signal::<CardResult>(CardResult::NoMatch);

    let input_element: NodeRef<html::Input> = create_node_ref();

    let action = create_action(|input: &(WriteSignal<CardResult>, String)| {
        let input = input.clone();
        async move { assign_card(&input).await }
    });

    view! {
        <div id="input-container">
            <input
                on:keypress=move |e| {
                    if e.key_code() == 13 {
                        action.dispatch((set_card_result, input_element.get().unwrap().value()))
                    }
                }

                node_ref=input_element
                type="search"
                name="search"
                id="card-input"
                placeholder="Black Lotus"
            />
        </div>
        <CardAndRulingsContainer card_result/>
    }
}

#[component]
fn CardAndRulingsContainer(card_result: ReadSignal<CardResult>) -> impl IntoView {
    view! {
        <div id="card-and-rulings-container">
            <CardContainer card_result/>
            <hr/>
            <Rulings/>
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
                    _ => {
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
fn Rulings() -> impl IntoView {
    view! {
        <div id="rulings-container">
            <ul id="rulings"></ul>
        </div>
    }
}

async fn assign_card(input: &(WriteSignal<CardResult>, String)) {
    let result = get_card_result(&input.1).await;

    input.0.set(result);
}
