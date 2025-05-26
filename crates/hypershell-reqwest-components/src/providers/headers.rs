use core::marker::PhantomData;

use cgp::prelude::*;
use hypershell_components::dsl::Header;
use reqwest::RequestBuilder;

use crate::components::{RequestBuilderUpdater, RequestBuilderUpdaterComponent};

#[cgp_new_provider]
impl<Context, Key, Value> RequestBuilderUpdater<Context, Header<Key, Value>> for UpdateRequestHeader
where
    Context: HasErrorType,
{
    fn update_request_builder(
        _context: &Context,
        _phantom: PhantomData<Header<Key, Value>>,
        _builder: &mut RequestBuilder,
    ) -> Result<(), Context::Error> {
        Ok(())
    }
}
