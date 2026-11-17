use cgp::prelude::*;
use reqwest::Client;

use crate::namespaces::HypershellNamespace;

#[derive(HasField)]
pub struct HypershellHttp {
    pub http_client: Client,
}

delegate_components! {
    HypershellHttp {
        namespace HypershellNamespace;
    }
}
