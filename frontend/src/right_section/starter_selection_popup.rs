use yew::virtual_dom::VNode;
use yew::{function_component, html, Html};
use yewdux::prelude::use_store;

use common::starter::starter_dto::StarterDto;

use crate::stores::StartersState;

#[function_component(FindStarter)]
fn find_starter() -> Html {
    html! {
        <input class="find-input"
                type="search"
                placeholder="Search for starter"
        />
    }
}

#[function_component(StarterSelectionPopup)]
pub fn starter_selection_popup() -> Html {
    let (_, dispatch) = use_store::<StartersState>();

    let all_unselected_starters = dispatch
        .get()
        .unselected_starters
        .clone()
        .into_iter()
        .collect::<Vec<StarterDto>>();

    // // TODO: change comparator to alphabetic
    // all_unselected_starters.sort_by(|first, second| {
    //     let first: usize = first.name.parse().unwrap();
    //     let second: usize = second.name.parse().unwrap();
    //     first.cmp(&second)
    // });

    let all_unselected_starters = all_unselected_starters
        .into_iter()
        .map(|unselected_starter| {
            let unselected_starter_clone = unselected_starter.clone();

            let onclick = dispatch.reduce_mut_callback(move |state| {
                state
                    .selected_starters
                    .insert(unselected_starter_clone.clone());
                state.unselected_starters.remove(&unselected_starter_clone);
                log::debug!("{:#?}", &state.unselected_starters);
                log::debug!("{:#?}", &state.selected_starters);
            });

            html! {
                <div onclick={onclick} class="starter">
                    <div class="name">{ unselected_starter.name.clone() }</div>
                    <div class="crates">{ unselected_starter.crates.join(" ") }</div>
                    <div class="description">{ unselected_starter.description.clone() }</div>
                </div>
            }
        })
        .collect::<Vec<VNode>>();

    html! {
        <div id="starter_selection" class="overlay">
            <div class="popup">
                <div style="height: 50px; padding-bottom: 10px; display: flex">
                    <FindStarter />
                    <div style="justify-content: center; padding-bottom: 10px;">
                        <a class="close" href="#">{"X"}</a>
                    </div>
                </div>
                <div class="list-overflow">
                    <ul>
                        {all_unselected_starters}
                    </ul>
                </div>
            </div>
        </div>
    }
}
