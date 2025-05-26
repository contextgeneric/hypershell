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
    use hypershell_tokio_components::presets::HypershellTokioPreset;
    use hypershell_tokio_components::providers::ExecOutputError;

    cgp_preset! {
        HypershellAppPreset: HypershellTokioPreset {
            ErrorTypeProviderComponent:
                UseAnyhowError,
            ErrorRaiserComponent:
                UseDelegate<HypershellErrorHandlers::Provider>,
            ErrorWrapperComponent:
                DebugAnyhowError,
        }
    }

    cgp_preset! {
        HypershellErrorHandlers {
            [
                StdIoError,
                Utf8Error,
            ]:
                RaiseAnyhowError,
            ExecOutputError:
                DebugAnyhowError,
        }
    }
}
