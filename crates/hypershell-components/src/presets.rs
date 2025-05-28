#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::*;

    use crate::components::{StringArgExtractorComponent, UrlArgExtractorComponent};
    use crate::dsl::{BytesToString, FieldArg, JoinArgs, Pipe, StaticArg};
    use crate::providers::{
        DecodeUtf8Bytes, ExtractFieldArg, ExtractMethodFieldArg, ExtractStaticArg,
        ExtractStringUrlArg, JoinStringArgs, RunPipe,
    };

    cgp_preset! {
        HypershellBasePreset {
            HandlerComponent:
                BaseHandlerPreset::Provider,
            StringArgExtractorComponent:
                StringArgExtractorPreset::Provider,
            UrlArgExtractorComponent:
                UrlArgExtractorPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        StringArgExtractorPreset {
            <Arg> StaticArg<Arg>: ExtractStaticArg,
            <Tag> FieldArg<Tag>: ExtractFieldArg,
            <Args> JoinArgs<Args>: JoinStringArgs,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        UrlArgExtractorPreset {
            [
                <Arg> StaticArg<Arg>,
                <Args> JoinArgs<Args>,
            ]: ExtractStringUrlArg,
            <Tag> FieldArg<Tag>: ExtractMethodFieldArg,
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
