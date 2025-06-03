use cgp::prelude::*;
use reqwest::Client;

use crate::presets::HypershellAppPreset;

#[cgp_context(HypershellHttpComponents: HypershellAppPreset)]
#[derive(HasField)]
pub struct HypershellHttp {
    pub http_client: Client,
}
