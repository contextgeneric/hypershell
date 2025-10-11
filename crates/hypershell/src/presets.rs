#[cgp::re_export_imports]
mod preset {
    use core::convert::Infallible;
    use core::str::Utf8Error;
    use std::io::Error as StdIoError;

    use cgp::core::component::UseDelegate;
    use cgp::core::error::{
        ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
    };
    use cgp::extra::error::{RaiseInfallible, ReturnError};
    use cgp::prelude::*;
    use cgp_error_anyhow::{DebugAnyhowError, Error, RaiseAnyhowError, UseAnyhowError};
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
    use reqwest::Error as ReqwestError;
    use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
    use serde_json::Error as JsonError;
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
                UseField<Symbol!("http_client")>,
            override StringArgExtractorComponent:
                HypershellStringArgExtractorPreset::Provider,
            override HandlerComponent:
                HypershellHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        HypershellErrorHandlers {
            Error: ReturnError,
            Infallible: RaiseInfallible,
            [
                StdIoError,
                Utf8Error,
                ReqwestError,
                ParseError,
                InvalidHeaderName,
                InvalidHeaderValue,
                JsonError,
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
