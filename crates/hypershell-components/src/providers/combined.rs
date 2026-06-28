use alloc::boxed::Box;

use cgp::extra::handler::Promote;
use cgp::prelude::{HandlerComponent, delegate_components};

use crate::components::{
    CommandArgExtractorComponent, StringArgExtractorComponent, UrlArgExtractorComponent,
};
use crate::dsl::{BytesToString, ConvertTo, FieldArg, JoinArgs, Pipe, StaticArg, Use};
use crate::providers::{
    BoxHandler, Call, DecodeUtf8Bytes, ExtractFieldArg, ExtractStaticArg, ExtractStringCommandArg,
    ExtractStringUrlArg, HandleConvert, HandlePipe, HandleUseProvider, JoinStringArgs,
};

delegate_components! {
    new HypershellBaseProvider {
        open {
            HandlerComponent,
            StringArgExtractorComponent,
            CommandArgExtractorComponent,
            UrlArgExtractorComponent,
        };

        @HandlerComponent.BytesToString:
            DecodeUtf8Bytes,
        @HandlerComponent.<T> ConvertTo<T>:
            Promote<HandleConvert>,
        @HandlerComponent.<Handlers> Pipe<Handlers>:
            HandlePipe,
        @HandlerComponent.<Provider, Code> Use<Provider, Code>:
            HandleUseProvider,
        @HandlerComponent.<Code> Box<Code>:
            BoxHandler<Call<Code>>,

        @StringArgExtractorComponent.<Arg> StaticArg<Arg>:
            ExtractStaticArg,
        @StringArgExtractorComponent.<Tag> FieldArg<Tag>:
            ExtractFieldArg,
        @StringArgExtractorComponent.<Args> JoinArgs<Args>:
            JoinStringArgs,

        @CommandArgExtractorComponent.[
            <Arg> StaticArg<Arg>,
            <Tag> FieldArg<Tag>,
        ]: ExtractStringCommandArg,

        @UrlArgExtractorComponent.[
            <Arg> StaticArg<Arg>,
            <Args> JoinArgs<Args>,
            <Tag> FieldArg<Tag>,
        ]: ExtractStringUrlArg,
    }
}
