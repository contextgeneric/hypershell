use core::marker::PhantomData;

use cgp::prelude::{Cons, Nil};

use crate::dsl::StaticArg;

pub struct WithArgs<Args>(pub PhantomData<Args>);

pub struct FieldArgs<Tag>(pub PhantomData<Tag>);

pub struct JoinArgs<Args>(pub PhantomData<Args>);

pub type WithStaticArgs<Args> = WithArgs<<Args as ToStaticArgs>::StaticArgs>;

pub trait ToStaticArgs {
    type StaticArgs;
}

impl ToStaticArgs for Nil {
    type StaticArgs = Nil;
}

impl<Arg, Args> ToStaticArgs for Cons<Arg, Args>
where
    Args: ToStaticArgs,
{
    type StaticArgs = Cons<StaticArg<Arg>, Args::StaticArgs>;
}
