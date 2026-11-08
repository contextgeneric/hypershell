use alloc::borrow::Cow;
use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::components::{
    CanExtractStringArg, CommandArgExtractor, CommandArgExtractorComponent, HasCommandArgType,
    StringArgExtractor, StringArgExtractorComponent,
};
use crate::dsl::{FieldArg, JoinArgs, StaticArg};

/// Extracts a command argument by converting the argument's string form with `From<String>`. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
#[cgp_impl(new ExtractStringCommandArg)]
impl<Context, Arg> CommandArgExtractor<Arg> for Context
where
    Context: HasCommandArgType + CanExtractStringArg<Arg>,
    Context::CommandArg: From<String>,
{
    fn extract_command_arg(context: &Context, phantom: PhantomData<Arg>) -> Context::CommandArg {
        context.extract_string_arg(phantom).into_owned().into()
    }
}

/// Extracts a literal argument by formatting its type's default value with `Display`. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
#[cgp_impl(new ExtractStaticArg)]
impl<Context, Arg> StringArgExtractor<StaticArg<Arg>> for Context
where
    Arg: Default + Display,
{
    fn extract_string_arg(
        _context: &Context,
        _phantom: PhantomData<StaticArg<Arg>>,
    ) -> Cow<'_, str> {
        Arg::default().to_string().into()
    }
}

/// Extracts the context field named `Tag`, formatted with `Display`. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
#[cgp_impl(new ExtractFieldArg)]
impl<Context, Tag> StringArgExtractor<FieldArg<Tag>> for Context
where
    Context: HasField<Tag, Value: Display>,
{
    fn extract_string_arg(context: &Context, _phantom: PhantomData<FieldArg<Tag>>) -> Cow<'_, str> {
        context.get_field(PhantomData).to_string().into()
    }
}

/// Concatenates a `JoinArgs` list into one string, with no separator. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
pub struct JoinStringArgs;

/// Extracts the first element of a `JoinArgs` list and appends the rest.
#[cgp_impl(JoinStringArgs)]
impl<Context, Arg, Args> StringArgExtractor<JoinArgs<Cons<Arg, Args>>> for Context
where
    Context: CanExtractStringArg<Arg>,
    JoinStringArgs: StringArgExtractor<Context, JoinArgs<Args>>,
{
    fn extract_string_arg(
        context: &Context,
        _phantom: PhantomData<JoinArgs<Cons<Arg, Args>>>,
    ) -> Cow<'_, str> {
        let arg = context.extract_string_arg(PhantomData);
        let args =
            <JoinStringArgs as StringArgExtractor<Context, JoinArgs<Args>>>::extract_string_arg(
                context,
                PhantomData::<JoinArgs<Args>>,
            );

        format!("{arg}{args}").into()
    }
}

/// Extracts the empty tail of a `JoinArgs` list as an empty string.
#[cgp_impl(JoinStringArgs)]
impl<Context> StringArgExtractor<JoinArgs<Nil>> for Context {
    fn extract_string_arg(
        _context: &Context,
        _phantom: PhantomData<JoinArgs<Nil>>,
    ) -> Cow<'_, str> {
        "".into()
    }
}
