use hypershell::prelude::*;
use hypershell_hash_components::dsl::Checksum;
use hypershell_hash_components::providers::HandleStreamChecksum;
use hypershell_macro::hypershell;
use hypershell_tokio_components::providers::{AsyncReadToStream, FuturesToTokioAsyncRead};
use reqwest::Client;
use sha2::Sha256;

pub type Program = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        StaticArg<"https://nixos.org/manual/nixpkgs/unstable/">,
        WithHeaders<Nil>,
    >
    | Use<FuturesToTokioAsyncRead>
    | Use<AsyncReadToStream>
    | Use<HandleStreamChecksum, Checksum<Sha256>>
    | ConvertTo<[u8; 32]>
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = HypershellHttp {
        http_client: Client::new(),
    };

    let checksum = app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("{}", hex::encode(checksum));

    Ok(())
}
