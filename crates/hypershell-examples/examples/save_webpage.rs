use hypershell::prelude::*;
use reqwest::Client;

pub type Program = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        FieldArg<"url">,
        WithHeaders[ ],
    >
    |   WriteFile<FieldArg<"file_path">>
};

#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub url: String,
    pub file_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        http_client: Client::new(),
        url: "https://nixos.org/manual/nixpkgs/unstable/".to_owned(),
        file_path: "nix_manual.html".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("Webpage saved to nix_manual.html");

    Ok(())
}
