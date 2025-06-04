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
    use hypershell_components::presets::{
        BaseHandlerPreset, BaseStringArgExtractorPreset, HypershellBasePreset,
    };
    use hypershell_json_components::presets::{HypershellJsonPreset, JsonHandlerPreset};
    use hypershell_reqwest_components::components::ReqwestClientGetterComponent;
    use hypershell_reqwest_components::presets::{
        HypershellReqwestPreset, ReqwestHandlerPreset, ReqwestStringArgExtractorPreset,
    };
    use hypershell_reqwest_components::providers::ErrorResponse;
    use hypershell_tokio_components::presets::{HypershellTokioPreset, TokioHandlerPreset};
    use hypershell_tokio_components::providers::ExecOutputError;
    use hypershell_tungstenite_components::presets::TungsteniteHandlerPreset;
    use reqwest::Error as ReqwestError;
    use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
    use serde_json::Error as JsonError;
    use tokio_tungstenite::tungstenite::Error as TungsteniteError;
    use url::ParseError;

    cgp_preset! {
        HypershellPreset:
            HypershellBasePreset
            + HypershellTokioPreset
            + HypershellReqwestPreset
            + HypershellJsonPreset
        {
            ErrorTypeProviderComponent:
                UseAnyhowError,
            ErrorRaiserComponent:
                HypershellErrorHandlers::Provider,
            ErrorWrapperComponent:
                DebugAnyhowError,
            ReqwestClientGetterComponent:
                UseField<symbol!("http_client")>,
            override StringArgExtractorComponent:
                HypershellStringArgExtractorPreset::Provider,
            override HandlerComponent:
                HypershellHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        HypershellErrorHandlers {
            [
                StdIoError,
                Utf8Error,
                ReqwestError,
                ParseError,
                InvalidHeaderName,
                InvalidHeaderValue,
                JsonError,
                TungsteniteError,
            ]:
                RaiseAnyhowError,
            [
                ExecOutputError,
                ErrorResponse,
            ]:
                DebugAnyhowError,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        HypershellHandlerPreset:
            BaseHandlerPreset
            + TokioHandlerPreset
            + ReqwestHandlerPreset
            + TungsteniteHandlerPreset
            + JsonHandlerPreset
        {
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        HypershellStringArgExtractorPreset:
            BaseStringArgExtractorPreset
            + ReqwestStringArgExtractorPreset
        { }
    }
}
