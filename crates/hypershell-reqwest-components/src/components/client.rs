use cgp::prelude::*;
use reqwest::Client;

#[cgp_getter]
pub trait HasReqwestClient {
    fn reqwuest_client(&self) -> &Client;
}
