#[cgp::re_export_imports]
mod preset {
    use core::str::Utf8Error;
    use std::io::Error as StdIoError;

    use cgp::core::component::UseDelegate;
    use cgp::core::error::{
        ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
    };
    use cgp::prelude::*;
    use cgp_error_anyhow::{DebugAnyhowError, RaiseAnyhowError, UseAnyhowError};
    use hypershell_components::presets::{BaseHandlerPreset, HypershellBasePreset};
    use hypershell_reqwest_components::presets::{HypershellReqwestPreset, ReqwestHandlerPreset};
    use hypershell_tokio_components::presets::{HypershellTokioPreset, TokioHandlerPreset};
    use hypershell_tokio_components::providers::ExecOutputError;
    use reqwest::Error as ReqwestError;

    cgp_preset! {
        HypershellAppPreset: HypershellBasePreset + HypershellTokioPreset + HypershellReqwestPreset {
            ErrorTypeProviderComponent:
                UseAnyhowError,
            ErrorRaiserComponent:
                AppErrorHandlers::Provider,
            ErrorWrapperComponent:
                DebugAnyhowError,
            override HandlerComponent:
                AppHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        AppErrorHandlers {
            [
                StdIoError,
                Utf8Error,
                ReqwestError,
            ]:
                RaiseAnyhowError,
            ExecOutputError:
                DebugAnyhowError,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        AppHandlerPreset: BaseHandlerPreset + TokioHandlerPreset + ReqwestHandlerPreset {

        }
    }
}
