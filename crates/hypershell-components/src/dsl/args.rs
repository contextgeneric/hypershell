use core::marker::PhantomData;

use crate::traits::WrapStaticArg;

pub struct WithArgs<Args>(pub PhantomData<Args>);

pub struct FieldArgs<Tag>(pub PhantomData<Tag>);

pub struct JoinArgs<Args>(pub PhantomData<Args>);

pub type WithStaticArgs<Args> = WithArgs<<Args as WrapStaticArg>::Wrapped>;
