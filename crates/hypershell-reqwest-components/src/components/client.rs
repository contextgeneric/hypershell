use cgp::prelude::*;
use reqwest::Client;

#[cgp_getter]
pub trait HasReqwestClient {
    fn request_client(&self) -> &Client;
}
