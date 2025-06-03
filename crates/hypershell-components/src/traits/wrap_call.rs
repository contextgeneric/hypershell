use cgp::prelude::*;

use crate::providers::Call;

pub trait WrapCall {
    type Wrapped;
}

impl<Head, Tail> WrapCall for Cons<Head, Tail>
where
    Tail: WrapCall,
{
    type Wrapped = Cons<Call<Head>, Tail::Wrapped>;
}

impl WrapCall for Nil {
    type Wrapped = Nil;
}
