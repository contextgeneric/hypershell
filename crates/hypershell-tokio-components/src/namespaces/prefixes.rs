use cgp::prelude::{UseDelegate, cgp_namespace, delegate_components};
use hypershell_components::dsl::{FieldArgs, WithArgs};
use hypershell_components::namespaces::{HypershellHandlers, HypershellNamespace};

use crate::components::CommandUpdaterComponent;
use crate::dsl::{CoreExec, ToTokioAsyncRead};

cgp_namespace! {
    new TokioNamespace: HypershellNamespace {
        @hypershell.tokio.CommandUpdaterComponent:
            UseDelegate<HypershellCommandUpdaters>,
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
            @hypershell.dsl.tokio.command.core,
    }
}
