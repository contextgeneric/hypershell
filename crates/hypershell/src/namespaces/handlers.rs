use cgp::prelude::{HandlerComponent, cgp_namespace};
use hypershell_components::namespaces::{BaseHandlerImpls, HypershellNamespace};
use hypershell_tokio_components::namespaces::TokioHandlerImpls;

cgp_namespace! {
    new HypershellDefautHandlers: HypershellNamespace {
        for <Key, Value> in BaseHandlerImpls {
            @hypershell.dsl.handler.core.HandlerComponent.Key:
                Value,
        }

        for <Key, Value> in TokioHandlerImpls {
            @hypershell.dsl.handler.[
                cli,
                file,
                stream,
                tokio,
            ].HandlerComponent.Key:
                Value,
        }
    }
}
