use cgp::extra::handler::PipeHandlers;
use cgp::prelude::*;
use hypershell_hash_components::dsl::{BytesToHex, Checksum};
use hypershell_hash_components::providers::{HandleBytesToHex, HandleStreamChecksum};
use hypershell_tokio_components::providers::HandleToFuturesStream;

delegate_components! {
    new HypershellChecksumProvider {
        open {HandlerComponent};

        @HandlerComponent.<Hasher> Checksum<Hasher>:
            PipeHandlers<Product![
                HandleToFuturesStream,
                HandleStreamChecksum,
            ]>,

        @HandlerComponent.BytesToHex:
            HandleBytesToHex,
    }
}
