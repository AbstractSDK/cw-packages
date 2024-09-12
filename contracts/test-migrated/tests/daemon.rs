mod common;

use cw_blob::interface::CwBlob;
use cw_orch::prelude::CwOrchUpload;
use cw_orch_daemon::{networks, Daemon};

// From https://github.com/CosmosContracts/juno/blob/32568dba828ff7783aea8cb5bb4b8b5832888255/docker/test-user.env#L2
const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

#[test]
fn daemon_local() {
    let daemon = Daemon::builder(networks::LOCAL_JUNO)
        .is_test(true)
        .mnemonic(LOCAL_MNEMONIC)
        .build()
        .unwrap();

    let blob = CwBlob::new("blob", daemon.clone());
    blob.upload();

    common::test(daemon, blob)
}
