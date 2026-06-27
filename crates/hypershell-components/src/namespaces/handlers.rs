use alloc::boxed::Box;

use cgp::extra::handler::Promote;
use cgp::prelude::cgp_namespace;

use crate::dsl::{BytesToString, ConvertTo, Pipe, Use};
use crate::providers::{
    BoxHandler, Call, DecodeUtf8Bytes, HandleConvert, HandlePipe, HandleUseProvider,
};

cgp_namespace! {
    new BaseHandlerImpls {
        BytesToString:
            DecodeUtf8Bytes,
        <T> ConvertTo<T>:
            Promote<HandleConvert>,
        <Handlers> Pipe<Handlers>:
            HandlePipe,
        <Provider, Code> Use<Provider, Code>:
            HandleUseProvider,
        <Code> Box<Code>:
            BoxHandler<Call<Code>>,
    }
}
