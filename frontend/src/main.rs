use std::collections::HashSet;
use reqwasm::http::Request;

use yew::{function_component, html, Html};
use yewdux::prelude::{Dispatch, use_store};

use common::starter_dto::StarterDto;

use crate::left_section::LeftSection;
use crate::right_section::RightSection;
use crate::stores::StartersState;

mod left_section;
mod right_section;
mod stores;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

fn get_test_starters() -> StartersState {
    let starter1 = StarterDto {
        name: "1".to_string(),
        crates: vec!["tokio".to_string(), "dotenv".to_string()],
        description: "Some description".to_string(),
    };

    let all_starters = (0..=15)
        .enumerate()
        .map(|x| StarterDto {
            name: x.0.to_string(),
            ..starter1.clone()
        })
        .collect::<Vec<StarterDto>>();

    StartersState {
        all_starters: HashSet::from_iter(all_starters.clone().into_iter()),
        selected_starters: HashSet::new(),
        unselected_starters: HashSet::from_iter(all_starters.into_iter()),
    }
}

fn get_real_starters(dispatch: Dispatch<StartersState>) {
    wasm_bindgen_futures::spawn_local(async move {
        let all_starters: Vec<StarterDto> = Request::get("http://localhost:8080/starters")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        log::debug!("{:#?}", &all_starters);

        let state = StartersState {
            all_starters: HashSet::from_iter(all_starters.clone().into_iter()),
            selected_starters: HashSet::new(),
            unselected_starters: HashSet::from_iter(all_starters.into_iter()),
        };

        dispatch.set(state);
    });
}

#[function_component(App)]
pub fn app() -> Html {
    let (_, dispatch) = use_store::<StartersState>();

    if dispatch.get().all_starters.is_empty() {
        match dotenv::var("MODE"). { }
        // let state = get_test_starters();
        // dispatch.set(state);

        get_real_starters(dispatch.clone());

        log::debug!("Initial values was successfully set.");
    }

    html! {
        <body>
            {header()}
                <div class="section-container">
                    <LeftSection />
                    <RightSection />
                </div>
            {footer()}
        </body>
    }
}

pub fn header() -> Html {
    html! {
        <header>
            <h1 class="paragraph">
                <img src="icons/cargo_init_icon_v4.png" alt="icon" class="image" />
                    <span class="text">{"cargo-initializr"}</span>
            </h1>
        </header>
    }
}

pub fn footer() -> Html {
    html! {
        <footer>
            <p style="text-align: center;">{"This is the footer of the page."}</p>
        </footer>
    }
}
