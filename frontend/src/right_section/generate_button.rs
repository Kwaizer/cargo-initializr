use yew::{function_component, html, Callback, Html};
use yewdux::prelude::use_store;

use crate::stores::ProjectDescriptionState;

#[function_component(GenerateButton)]
pub fn generate_button() -> Html {
    let (item, _) = use_store::<ProjectDescriptionState>();
    let onclick = Callback::from(move |_| {
        log::debug!("generating {:?}", &item);
    });

    html! {
        <button {onclick} class="generate-btn">
            {"Generate"}
        </button>
    }
}
