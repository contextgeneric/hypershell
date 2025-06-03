use cgp::prelude::*;
use reqwest::Client;

use crate::presets::HypershellPreset;

#[cgp_context(HypershellHttpComponents: HypershellPreset)]
#[derive(HasField)]
pub struct HypershellHttp {
    pub http_client: Client,
}
