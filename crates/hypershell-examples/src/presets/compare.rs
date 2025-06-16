#[cgp::re_export_imports]
mod preset {
    use cgp::extra::handler::HandlerComponent;
    use hypershell::prelude::*;
    use hypershell_components::providers::BoxHandler;

    use crate::dsl::{Compare, If};
    use crate::presets::ChecksumHandlerPreset;
    use crate::providers::{HandleCompare, HandleIf};

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
                BoxHandler<HandleCompare>,
            <CodeCond, CodeThen, CodeElse> If<CodeCond, CodeThen, CodeElse>:
                HandleIf,
        }
    }
}
