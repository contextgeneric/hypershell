use cgp::prelude::{HandlerComponent, cgp_namespace};
use hypershell::namespaces::HypershellNamespace;
use hypershell_hash_components::dsl::{BytesToHex, Checksum};

use crate::providers::HypershellChecksumProvider;

cgp_namespace! {
    new HypershellChecksumNamespace: HypershellNamespace {
        @cgp.extra.handler.HandlerComponent.[
            <Hasher> Checksum<Hasher>,
            BytesToHex,
        ]:
            HypershellChecksumProvider,
    }
}
