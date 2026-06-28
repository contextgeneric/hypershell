use cgp::prelude::{HandlerComponent, cgp_namespace};
use hypershell_components::providers::BoxHandler;

use crate::dsl::{Compare, If};
use crate::namespaces::HypershellChecksumNamespace;
use crate::providers::{HandleCompare, HandleIf};

cgp_namespace! {
    new HypershellCompareNamespace: HypershellChecksumNamespace {
        // Note: The compare handler is somehow much slower when the future is not boxed
        @cgp.extra.handler.HandlerComponent.<CodeA, CodeB> Compare<CodeA, CodeB>:
            BoxHandler<HandleCompare>,

        @cgp.extra.handler.HandlerComponent.<CodeCond, CodeThen, CodeElse> If<CodeCond, CodeThen, CodeElse>:
            HandleIf,
    }
}
