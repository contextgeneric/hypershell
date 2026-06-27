use cgp::prelude::{cgp_namespace, delegate_components};
use hypershell_components::dsl::{FieldArgs, WithArgs};
use hypershell_components::namespaces::HypershellHandlers;

use crate::dsl::{CoreExec, ToTokioAsyncRead};

cgp_namespace! {
    HypershellNamespace {

    }
}

delegate_components! {
    HypershellHandlers {
        [
            ToTokioAsyncRead,
            <Path, Args> CoreExec<Path, Args>,
        ] =>
            @hypershell.dsl.handler.tokio,
    }
}

delegate_components! {
    new HypershellCommandUpdaters {
        [
            <Args> WithArgs<Args>,
            <Tag> FieldArgs<Tag>,
        ] =>
            @hypershell,
    }
}
