use cgp::prelude::*;

use crate::dsl::StaticArg;

pub trait WrapStaticArg {
    type Wrapped;
}

impl WrapStaticArg for Nil {
    type Wrapped = Nil;
}

impl<Arg, Args> WrapStaticArg for Cons<Arg, Args>
where
    Args: WrapStaticArg,
{
    type Wrapped = Cons<StaticArg<Arg>, Args::Wrapped>;
}
