use cgp::prelude::delegate_components;
use hypershell_components::namespaces::HypershellHandlers;

use crate::dsl::ToTokioAsyncRead;

delegate_components! {
    HypershellHandlers {
        [
            ToTokioAsyncRead,
        ] =>
            @hypershell.dsl.handler.tokio,
    }
}
