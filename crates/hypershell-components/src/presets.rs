#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::{HandlerComponent, Promote};
    use cgp::prelude::*;

    use crate::components::{StringArgExtractorComponent, UrlArgExtractorComponent};
    use crate::dsl::{BytesToString, ConvertTo, FieldArg, JoinArgs, Pipe, StaticArg, Use};
    use crate::providers::{
        DecodeUtf8Bytes, ExtractFieldArg, ExtractStaticArg, ExtractStringUrlArg, HandleConvert,
        HandlePipe, HandleUseProvider, JoinStringArgs,
    };

    cgp_preset! {
        HypershellBasePreset {
            HandlerComponent:
                BaseHandlerPreset::Provider,
            StringArgExtractorComponent:
                BaseStringArgExtractorPreset::Provider,
            UrlArgExtractorComponent:
                UrlArgExtractorPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        BaseStringArgExtractorPreset {
            <Arg> StaticArg<Arg>:
                ExtractStaticArg,
            <Tag> FieldArg<Tag>:
                ExtractFieldArg,
            <Args> JoinArgs<Args>:
                JoinStringArgs,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        UrlArgExtractorPreset {
            [
                <Arg> StaticArg<Arg>,
                <Args> JoinArgs<Args>,
                <Tag> FieldArg<Tag>,
            ]: ExtractStringUrlArg,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        BaseHandlerPreset {
            BytesToString:
                DecodeUtf8Bytes,
            <T> ConvertTo<T>:
                Promote<HandleConvert>,
            <Handlers> Pipe<Handlers>:
                HandlePipe,
            <Provider, Code> Use<Provider, Code>:
                HandleUseProvider,
        }
    }
}
