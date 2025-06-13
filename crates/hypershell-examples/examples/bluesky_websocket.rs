// This example demonstrates how to extend Hypershell with a native WebSocket
// handler to connect to the Bluesky firehose and process the stream.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. A native `WebSocket` handler is used to connect to the Bluesky
//    WebSocket stream. This handler is provided by the
//    `hypershell-tungstenite-components` extension crate.
//
// 2. The output stream from the WebSocket connection is piped to a
//    `StreamingExec` handler, which runs `grep` to filter the stream
//    for a dynamic `keyword`.
//
// 3. The final filtered output is piped to `StreamToStdout` to be printed
//    on the console.
//
// Since the `WebSocket` handler is not part of the default `HypershellPreset`,
// a custom `MyAppPreset` is defined to extend Hypershell's functionality.
//
// The `MyAppPreset` inherits from `HypershellPreset` and overrides the
// `HandlerComponent` to include handlers from `TungsteniteHandlerPreset`,
// which provides the `WebSocket` handler implementation. It also overrides
// `ErrorRaiserComponent` to handle potential `TungsteniteError`s.
//
// The `MyApp` context is configured to use this custom `MyAppPreset`,
// enabling it to execute the program with the extended capabilities.
//
// The `main` function initializes `MyApp` with a keyword and runs the program.
// An empty input stream is provided to `app.handle` as required by the
// handler signature, although it is not used by the `WebSocket` handler.

use hypershell::prelude::*;
use hypershell_tokio_components::types::TokioAsyncReadStream;
use tokio::io::simplex;

pub type Program = hypershell! {
        WebSocket<
            StaticArg<"wss://jetstream1.us-west.bsky.network/subscribe">,
            (),
        >
    |   StreamingExec<
            StaticArg<"grep">,
            WithArgs [ FieldArg<"keyword"> ],
        >
    |   StreamToStdout
};

#[cgp_context(MyAppComponents: MyAppPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub keyword: String,
}

#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use cgp_error_anyhow::RaiseAnyhowError;
    use hypershell::presets::{HypershellErrorHandlers, HypershellHandlerPreset, HypershellPreset};
    use hypershell_tungstenite_components::presets::TungsteniteHandlerPreset;
    use tokio_tungstenite::tungstenite::Error as TungsteniteError;

    cgp_preset! {
        MyAppPreset: HypershellPreset {
            override ErrorRaiserComponent:
                MyErrorHandlers::Provider,
            override HandlerComponent:
                MyHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        MyErrorHandlers: HypershellErrorHandlers {
            TungsteniteError: RaiseAnyhowError,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        MyHandlerPreset:
            HypershellHandlerPreset
            + TungsteniteHandlerPreset
        {
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        keyword: "love".to_owned(),
    };

    // Create an input stream that is never written, as otherwise the Websocket stream
    // will be closed once the input stream is closed.
    let (read, _write) = simplex(102400);
    let input = TokioAsyncReadStream::from(read);

    app.handle(PhantomData::<Program>, input).await?;

    Ok(())
}
