#[cgp::re_export_imports]
mod preset {
    use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
    use cgp::prelude::*;
    use cgp_error_anyhow::{DebugAnyhowError, UseAnyhowError};
    use hypershell_tokio_components::presets::HypershellTokioPreset;

    cgp_preset! {
        HypershellAppPreset: HypershellTokioPreset {
            ErrorTypeProviderComponent:
                UseAnyhowError,
            ErrorRaiserComponent:
                DebugAnyhowError,
        }
    }
}
