use strata_node_rs::co::topl::{consensus::models::BlockId, node::models::Block};

use super::app_config::NodeConfig;

trait Show {
    type T;

    fn to_string(&self) -> String;
}

impl Show for BlockId {
    type T = BlockId;

    fn to_string(&self) -> String {
        let block_id_string =
            std::str::from_utf8(&self.value.as_slice()).expect("Invalid UTF-8 Sequence");

        return format!("b_{block_id_string}");
    }
}

struct DataStore;

impl DataStore {
    async fn create(args: &NodeConfig, genesis_id: BlockId) {
        let data_dir = args
            .data_dir
            .replace("{genesisBlockId}", genesis_id.to_string().as_str());

        let mut db = rusty_leveldb::DB::open("strata-node", rusty_leveldb::Options::default());
    }
}
