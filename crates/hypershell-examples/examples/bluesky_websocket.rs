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

    let (read, _write) = simplex(102400);
    let input = TokioAsyncReadStream::from(read);

    app.handle(PhantomData::<Program>, input).await?;

    Ok(())
}
