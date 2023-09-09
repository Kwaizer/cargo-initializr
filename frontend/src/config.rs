use crate::stores::{AppConfig, IntegrationMode};
use url::Url;
use yewdux::prelude::Dispatch;

pub fn init_app_config(dispatch: Dispatch<AppConfig>) {
    let app_config = get_app_config();
    dispatch.set(app_config.clone());
    log::debug!("AppConfig was initialized: {:#?}", app_config)
}

pub fn get_app_config() -> AppConfig {
    let integration_mode = match option_env!("YEW_INTEGRATION_MODE") {
        Some(mode) => IntegrationMode::from(mode),
        None => IntegrationMode::default(),
    };

    let backend_url = if integration_mode == IntegrationMode::Test {
        None
    } else {
        let url = option_env!("YEW_BACKEND_URL").expect("Missing 'YEW_BACKEND_URL' env variable.");
        let url = Url::parse(&*url).expect("Invalid 'YEW_BACKEND_URL' env variable.");
        Some(url)
    };

    AppConfig {
        integration_mode,
        backend_url,
    }
}
