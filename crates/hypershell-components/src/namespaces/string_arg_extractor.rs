use cgp::prelude::cgp_namespace;

use crate::dsl::{FieldArg, JoinArgs, StaticArg};
use crate::providers::{ExtractFieldArg, ExtractStaticArg, JoinStringArgs};

cgp_namespace! {
    new BaseStringArgExtractorImpls {
        <Arg> StaticArg<Arg>:
            ExtractStaticArg,
        <Tag> FieldArg<Tag>:
            ExtractFieldArg,
        <Args> JoinArgs<Args>:
            JoinStringArgs,
    }
}
