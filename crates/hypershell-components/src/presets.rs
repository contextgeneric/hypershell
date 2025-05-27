#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::*;

    use crate::components::StringArgExtractorComponent;
    use crate::dsl::{BytesToString, FieldArg, Pipe, StaticArg};
    use crate::providers::{DecodeUtf8Bytes, ExtractFieldArg, ExtractStaticArg, RunPipe};

    cgp_preset! {
        HypershellBasePreset {
            HandlerComponent:
                BaseHandlerPreset::Provider,
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

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        BaseHandlerPreset {
            BytesToString:
                DecodeUtf8Bytes,
            <Handlers> Pipe<Handlers>:
                RunPipe,
        }
    }
}
