use core::marker::PhantomData;

pub struct Compare<CodeA, CodeB>(pub PhantomData<(CodeA, CodeB)>);

pub struct If<CodeCond, CodeThen, CodeElse>(pub PhantomData<(CodeCond, CodeThen, CodeElse)>);
