use crate::stores::StartersState;
use common::starter::starter_dto::StarterDto;
use reqwasm::http::Request;
use std::collections::HashSet;
use url::Url;
use yewdux::prelude::Dispatch;

pub fn get_mock_starters() -> StartersState {
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

pub fn get_starters(dispatch: Dispatch<StartersState>, mut url: Url) {
    wasm_bindgen_futures::spawn_local(async move {
        url.set_path("starters");
        let all_starters: Vec<StarterDto> = Request::get(url.as_str())
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
