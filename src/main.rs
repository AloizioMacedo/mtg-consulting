use leptos::*;

fn main() {
    leptos::mount_to_body(|| {
        view! {
            <main>
                <div id="sel-call-container">
                    <h1>Select your card</h1>
                </div>
                <App/>
            </main>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div id="input-container">
            <input
                class="form-control"
                type="search"
                name="search"
                id="card-input"
                hx-post="/get-card"
                hx-trigger="keyup[keyCode==13]"
                hx-target="#card-and-rulings-container"
                placeholder="Black Lotus"
                hx-swap="innerHTML"
                hx-push-url="style.css"
            />
        </div>
        <div id="card-and-rulings-container">
            <CardContainer/>
            <hr/>
            <Rulings/>
        </div>
    }
}

#[component]
fn CardContainer() -> impl IntoView {
    view! {
        <div id="card-container">
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
