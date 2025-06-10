use hypershell::prelude::*;
use hypershell_hash_components::dsl::{BytesToHex, Checksum};
use hypershell_macro::hypershell;
use reqwest::Client;
use sha2::Sha256;

pub type Program = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        FieldArg<"url">,
        WithHeaders[ ],
    >
    | Checksum<Sha256>
    | BytesToHex
    | StreamToStdout
};

#[cgp_context(MyAppComponents: MyAppPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub url: String,
}

#[cgp::re_export_imports]
mod preset {
    use cgp::extra::handler::{PipeHandlers, UseInputDelegate};
    use hypershell::prelude::*;
    use hypershell::presets::HypershellHandlerPreset;
    use hypershell_hash_components::dsl::{BytesToHex, Checksum};
    use hypershell_hash_components::providers::{HandleBytesToHex, HandleStreamChecksum};
    use hypershell_tokio_components::providers::{
        AsyncReadToStream, FuturesToTokioAsyncRead, HandleBytesToStream,
    };
    use hypershell_tokio_components::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

    cgp_preset! {
        MyAppPreset: HypershellPreset {
            override HandlerComponent:
                MyHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        MyHandlerPreset: HypershellHandlerPreset {
            <Hasher> Checksum<Hasher>:
                UseInputDelegate<new ChecksumHandlers {
                    <S> FuturesAsyncReadStream<S>:
                        PipeHandlers<Product![
                            FuturesToTokioAsyncRead,
                            AsyncReadToStream,
                            HandleStreamChecksum,
                        ]>,
                    <S> TokioAsyncReadStream<S>:
                        PipeHandlers<Product![
                            AsyncReadToStream,
                            HandleStreamChecksum,
                        ]>,
                    Vec<u8>:
                        PipeHandlers<Product![
                            HandleBytesToStream,
                            HandleStreamChecksum,
                        ]>,
                }>,
            BytesToHex:
                HandleBytesToHex,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        http_client: Client::new(),
        url: "https://nixos.org/manual/nixpkgs/unstable/".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
