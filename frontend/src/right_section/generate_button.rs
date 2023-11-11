use url_builder::URLBuilder;
use yew::{function_component, html, Html};
use yewdux::prelude::use_store;

use crate::stores::{AppConfig, IntegrationMode, ProjectDescriptionState, StartersState};


#[function_component(GenerateButton)]
pub fn generate_button() -> Html {
    let (project_description_store, _) = use_store::<ProjectDescriptionState>();
    let (selected_starter_store, _) = use_store::<StartersState>();
    let (app_config_store, _) = use_store::<AppConfig>();

    let download_url = match app_config_store.integration_mode {
        IntegrationMode::Test => {
            log::debug!("generating {:#?}", &project_description_store.project_description);
            "".to_string()
        },
        IntegrationMode::Production => {
            log::debug!("generating {:#?}", &project_description_store.project_description);
            let desc = project_description_store.project_description.clone();
            let mut url_builder = URLBuilder::new();
            url_builder
                .set_protocol("http")
                .set_host("localhost")
                .set_port(8080)
                .add_route("download")
                .add_param("target_kind", &desc.target_kind.to_string())
                .add_param("name", &desc.package_description.name.0);

            if let Some(author) = desc.package_description.author {
                url_builder.add_param("author", &author);
            }

            if let Some(description) = desc.package_description.description {
                url_builder.add_param("description", &description);
            }

            if !selected_starter_store.selected_starters.is_empty() {
                let starters = selected_starter_store.selected_starters
                        .clone()
                        .into_iter()
                        .map(|s| s.name)
                        .collect::<Vec<String>>()
                        .join(";");
                let starters = format!("[{starters}]");
                url_builder.add_param("starters", &starters);
            }

            url_builder.build()
        }
    };

    log::debug!("{download_url}");

    html! {
        <a href={download_url}><button class="generate-btn">
            {"Generate"}
        </button></a>
    }
}

