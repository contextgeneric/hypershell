#[cgp::re_export_imports]
mod preset {
    use cgp::extra::handler::PipeHandlers;
    use hypershell::prelude::*;
    use hypershell::presets::HypershellHandlerPreset;
    use hypershell_hash_components::dsl::{BytesToHex, Checksum};
    use hypershell_hash_components::providers::{HandleBytesToHex, HandleStreamChecksum};
    use hypershell_tokio_components::presets::ToFuturesStreamHandlers;

    cgp_preset! {
        HypershellChecksumPreset: HypershellPreset {
            override HandlerComponent:
                ChecksumHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        ChecksumHandlerPreset: HypershellHandlerPreset {
            <Hasher> Checksum<Hasher>:
                PipeHandlers<Product![
                    ToFuturesStreamHandlers::Provider,
                    HandleStreamChecksum,
                ]>,
            BytesToHex:
                HandleBytesToHex,
        }
    }
}
