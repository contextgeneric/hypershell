use cgp::prelude::cgp_namespace;

use crate::dsl::{FieldArg, JoinArgs, StaticArg};
use crate::providers::ExtractStringUrlArg;


cgp_namespace! {
    new BaseUrlArgExtractorImpls {
        [
            <Arg> StaticArg<Arg>,
            <Args> JoinArgs<Args>,
            <Tag> FieldArg<Tag>,
        ]: ExtractStringUrlArg,
    }
}
