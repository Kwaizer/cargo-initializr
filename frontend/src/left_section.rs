use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_input::InputDispatch;

use common::project_description_dto::target_kind::TargetKind;

use crate::stores::ProjectDescriptionState;

fn is_name_valid(name: &str) -> bool {
    name.chars()
        .filter(|char| char.ne(&'-') && char.ne(&'_'))
        .all(char::is_alphanumeric)
}

#[function_component(LeftSection)]
pub fn left_section() -> Html {
    let (state, dispatch) = use_store::<ProjectDescriptionState>();

    let onselect_for_target = dispatch.input_mut(|desc, input: TargetKind| {
        desc.project_description.target_kind = input;
        log::debug!("{:?}", desc);
    });
    let session_value_for_target = match state.project_description.target_kind {
        TargetKind::Bin => (true, false),
        TargetKind::Lib => (false, true),
    };

    let onchange_for_name = dispatch.input_mut(|desc, input: String| {
        if !input.is_empty() && is_name_valid(&input) {
            desc.project_description.package_description.name = input.into();
            desc.is_description_valid = true;
            log::debug!("{:?}", desc);
            log::debug!("name is valid");
        } else {
            desc.is_description_valid = false;
            log::debug!("name is NOT valid");
        }
    });
    let session_value_for_name = state.project_description.package_description.name.0.clone();

    let onchange_for_author = dispatch.input_mut(|desc, input: String| {
        desc.project_description.package_description.author = (!input.is_empty()).then_some(input);
        log::debug!("{:?}", desc);
    });
    let session_value_for_author = state
        .project_description
        .package_description
        .author
        .clone()
        .unwrap_or_default();

    let onchange_for_description = dispatch.input_mut(|item, input: String| {
        item.project_description.package_description.description =
            (!input.is_empty()).then_some(input);
        log::debug!("{:?}", item);
    });
    let session_value_for_description = state
        .project_description
        .package_description
        .description
        .clone()
        .unwrap_or_default();

    html! {
        <div class="section">
            <form>
                <h2 style="text-align: center;">{"Project description"}</h2>

                <label for="selector">{"Target kind:"}</label>
                    <div style="display: flex; width: 400px; justify-content: center;">
                        <div style="display: flex; font-family: Monospace, sans-serif;">
                            <input onchange={&onselect_for_target}
                                    type="radio"
                                    id="bin"
                                    name="target"
                                    value="bin"
                                    checked={session_value_for_target.0} />
                                <label for="bin">{"Bin"}</label><br />
                        </div>
                        <div style="width: 50px;"></div>
                        <div style="display: flex; font-family: Monospace, sans-serif;">
                            <input onchange={onselect_for_target}
                                    type="radio"
                                    id="lib"
                                    name="target"
                                    value="lib"
                                    checked={session_value_for_target.1} />
                                <label for="lib">{"Lib"}</label><br />
                        </div>
                    </div>

                <div>
                    <label for="name">{"Name:"}</label>
                        <input
                            onchange={onchange_for_name}
                            type="text"
                            id="name"
                            name="name"
                            value={session_value_for_name}
                            required=true
                        />
                </div>

                <label for="author">{"Author:"}</label>
                    <input
                        onchange={onchange_for_author}
                        type="text"
                        id="author"
                        name="author"
                        value={session_value_for_author}
                        required=false
                    />

                <label for="description">{"Description:"}</label>
                    <input
                        onchange={onchange_for_description}
                        type="text"
                        id="description"
                        name="description"
                        value={session_value_for_description}
                        required=false
                    />
            </form>
        </div>
    }
}
