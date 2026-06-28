use cgp::prelude::{HandlerComponent, delegate_components};
use hypershell_components::dsl::{DecodeJson, EncodeJson};

use crate::providers::{HandleDecodeJson, HandleEncodeJson};

delegate_components! {
    new HypershellJsonProvider {
        open {HandlerComponent};

        @HandlerComponent.<Value> DecodeJson<Value>:
            HandleDecodeJson,

        @HandlerComponent.EncodeJson:
            HandleEncodeJson,
    }
}
