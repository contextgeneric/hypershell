use cgp::prelude::*;

use crate::namespaces::HypershellNamespace;

pub struct HypershellCli;

delegate_components! {
    HypershellCli {
        namespace HypershellNamespace;
    }
}
