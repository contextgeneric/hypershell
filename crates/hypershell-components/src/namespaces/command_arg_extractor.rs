use cgp::prelude::cgp_namespace;

use crate::dsl::{FieldArg, StaticArg};
use crate::providers::ExtractStringCommandArg;

cgp_namespace! {
    new BaseCommandArgExtractorImpls {
        [
            <Arg> StaticArg<Arg>,
            <Tag> FieldArg<Tag>,
        ]: ExtractStringCommandArg,
    }
}
