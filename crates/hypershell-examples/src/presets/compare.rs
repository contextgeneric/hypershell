#[cgp::re_export_imports]
mod preset {
    use cgp::extra::handler::HandlerComponent;
    use hypershell::prelude::*;

    use crate::dsl::Compare;
    use crate::presets::ChecksumHandlerPreset;
    use crate::providers::HandleCompare;

    cgp_preset! {
        HypershellComparePreset: HypershellPreset {
            override HandlerComponent:
                CompareHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        CompareHandlerPreset: ChecksumHandlerPreset {
            <CodeA, CodeB> Compare<CodeA, CodeB>:
                HandleCompare,
        }
    }
}
