use cgp::extra::handler::HandlerComponent;
use cgp::prelude::delegate_components;
use hypershell_components::dsl::{DecodeJson, EncodeJson};

use crate::providers::{HandleDecodeJson, HandleEncodeJson};

delegate_components! {
    new HypershellJsonProvider {
        open {HandlerComponent};

        @HandlerComponent.EncodeJson:
            HandleEncodeJson,

        @HandlerComponent.<Value> DecodeJson<Value>:
            HandleDecodeJson,
    }
}
