use yew::virtual_dom::VNode;
use yew::{function_component, html, Html};
use yewdux::prelude::use_store;

use generate_button::GenerateButton;
use starter_selection_popup::StarterSelectionPopup;

use crate::stores::StartersState;

mod generate_button;
mod starter_selection_popup;

#[function_component(RightSection)]
pub fn right_section() -> Html {
    let (_, dispatch) = use_store::<StartersState>();

    let selected_starters = dispatch
        .get()
        .selected_starters
        .clone()
        .into_iter()
        .map(|selected_starter| {
            let selected_starter_clone = selected_starter.clone();
            let onclick = dispatch.reduce_mut_callback(move |state| {
                state.selected_starters.remove(&selected_starter_clone);
                state
                    .unselected_starters
                    .insert(selected_starter_clone.clone());
                log::debug!("{:#?}", &state.unselected_starters);
                log::debug!("{:#?}", &state.selected_starters);
            });

            html! {
                <div class="item-with-delete-button">
                    <div>
                        <button class="remove_button" onclick={onclick}> {"Remove"} </button>
                    </div>
                    <div class="item">
                        <div class="name">{ selected_starter.name }</div>
                        <div class="crates">{ selected_starter.crates.join(" ") }</div>
                        <div class="description">{ selected_starter.description }</div>
                    </div>
                </div>
            }
        })
        .collect::<Vec<VNode>>();

    html! {
        <div class="section">
            <StarterSelectionPopup />
            <div style="justify-content: center; display: flex;">
                <a style="text-decoration: none;" href={"#starter_selection"}>
                    <button class="add-starters-btn">{"Add starters"}</button>
                </a>
                <GenerateButton />
            </div>
            <ul>
                {selected_starters}
            </ul>
        </div>
    }
}
