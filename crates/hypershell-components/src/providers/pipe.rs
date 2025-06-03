use cgp::extra::handler::{HandlerComponent, PipeHandlers};
use cgp::prelude::*;

use crate::dsl::Pipe;
use crate::traits::WrapCall;

delegate_components! {
    new HandlePipe {
        HandlerComponent: UseDelegate<new RunPipeHandler {
            <Handlers: WrapCall> Pipe<Handlers>:
                PipeHandlers<Handlers::Wrapped>,
        }>,
    }
}
