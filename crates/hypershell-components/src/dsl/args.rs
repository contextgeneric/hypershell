use core::marker::PhantomData;

use cgp::prelude::{Cons, Nil};

pub struct WithArgs<Args>(pub PhantomData<Args>);

pub struct FieldArgs<Tag>(pub PhantomData<Tag>);

pub struct StaticArg<Arg>(pub PhantomData<Arg>);

pub struct FieldArg<Tag>(pub PhantomData<Tag>);

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
