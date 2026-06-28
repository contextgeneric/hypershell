use cgp::prelude::*;
use reqwest::Client;

#[cgp_getter]
#[prefix(@hypershell.reqwest in DefaultNamespace)]
pub trait HasReqwestClient {
    fn request_client(&self) -> &Client;
}
