use yew::{function_component, html, Html};
use yewdux::prelude::use_store;

use crate::config::init_app_config;

use crate::left_section::LeftSection;
use crate::right_section::RightSection;
use crate::starters::{get_mock_starters, get_starters};
use crate::stores::{AppConfig, IntegrationMode, StartersState};

mod config;
mod left_section;
mod right_section;
mod starters;
mod stores;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    let (_, dispatch_starters) = use_store::<StartersState>();
    let (_, dispatch_config) = use_store::<AppConfig>();
    init_app_config(&dispatch_config);

    if dispatch_starters.get().all_starters.is_empty() {
        match dispatch_config.get().integration_mode {
            IntegrationMode::Test => {
                let state = get_mock_starters();
                dispatch_starters.set(state);
            }
            IntegrationMode::Production => {
                get_starters(
                    dispatch_starters.clone(),
                    dispatch_config.get().backend_url.clone().unwrap(),
                );
            }
        }

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
