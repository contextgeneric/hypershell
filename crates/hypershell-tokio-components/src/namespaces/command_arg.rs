use cgp::prelude::cgp_namespace;
use hypershell_components::dsl::JoinArgs;
use hypershell_components::namespaces::BaseCommandArgExtractorImpls;

use crate::providers::JoinExtractArgs;

cgp_namespace! {
    new TokioCommandArgExtractorImpls: BaseCommandArgExtractorImpls {
        <Args> JoinArgs<Args>:
            JoinExtractArgs,
    }
}
