use core::convert::Infallible;
use core::str::Utf8Error;

use cgp::core::error::ErrorRaiserComponent;
use cgp::extra::error::{RaiseInfallible, ReturnError};
use cgp::prelude::delegate_components;
use cgp_error_anyhow::{DebugAnyhowError, Error, RaiseAnyhowError};
use hypershell_reqwest_components::providers::ErrorResponse;
use hypershell_tokio_components::providers::ExecOutputError;
use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use url::ParseError;

delegate_components! {
    new HypershellErrorHandler {
        open {ErrorRaiserComponent};

        Error: ReturnError,
        Infallible: RaiseInfallible,
        [
            std::io::Error,
            Utf8Error,
            reqwest::Error,
            ParseError,
            InvalidHeaderName,
            InvalidHeaderValue,
            serde_json::Error,
        ]:
            RaiseAnyhowError,
        [
            ExecOutputError,
            ErrorResponse,
        ]:
            DebugAnyhowError,
    }
}
