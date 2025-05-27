#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::prelude::*;

    use crate::components::StringArgExtractorComponent;
    use crate::dsl::{FieldArg, StaticArg};
    use crate::providers::{ExtractFieldArg, ExtractStaticArg};

    cgp_preset! {
        HypershellBasePreset {
            StringArgExtractorComponent:
                UseDelegate<StringArgExtractorPreset::Provider>,
        }
    }

    cgp_preset! {
        StringArgExtractorPreset {
            <Arg> StaticArg<Arg>: ExtractStaticArg,
            <Tag> FieldArg<Tag>: ExtractFieldArg,
        }
    }
}
