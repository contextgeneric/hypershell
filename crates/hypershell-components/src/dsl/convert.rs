use core::marker::PhantomData;

pub struct StreamToBytes;

pub struct StreamToString;

pub struct BytesToString;

pub struct BytesToStream;

pub struct StreamToLines;

pub struct ConvertTo<T>(pub PhantomData<T>);
