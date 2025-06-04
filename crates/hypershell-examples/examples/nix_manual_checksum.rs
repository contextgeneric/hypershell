use hypershell::prelude::*;
use hypershell_hash_components::dsl::Checksum;
use hypershell_macro::hypershell;
use reqwest::Client;
use sha2::Sha256;

pub type Program = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        StaticArg<"https://nixos.org/manual/nixpkgs/unstable/">,
        WithHeaders<Nil>,
    >
    | Checksum<Sha256>
    | ConvertTo<[u8; 32]>
};

#[cgp_context(MyAppComponents: MyAppPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
}

#[cgp::re_export_imports]
mod preset {
    use cgp::extra::handler::PipeHandlers;
    use hypershell::prelude::*;
    use hypershell::presets::HypershellHandlerPreset;
    use hypershell_hash_components::dsl::Checksum;
    use hypershell_hash_components::providers::HandleStreamChecksum;
    use hypershell_tokio_components::providers::{AsyncReadToStream, FuturesToTokioAsyncRead};

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
                PipeHandlers<Product![
                    FuturesToTokioAsyncRead,
                    AsyncReadToStream,
                    HandleStreamChecksum,
                ]>,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        http_client: Client::new(),
    };

    let checksum = app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("{}", hex::encode(checksum));

    Ok(())
}
