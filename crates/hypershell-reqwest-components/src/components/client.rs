use cgp::prelude::*;
use reqwest::Client;

/// Gets the `reqwest` client an HTTP handler sends its requests with.
#[cgp_getter]
#[prefix(@hypershell.reqwest in DefaultNamespace)]
pub trait HasReqwestClient {
    fn request_client(&self) -> &Client;
}
