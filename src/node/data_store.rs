use std::fmt::Display;

use super::app_config::NodeConfig;

struct BlockId(strata_node_rs::co::topl::consensus::models::BlockId);

impl Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let block_id_string =
            std::str::from_utf8(&self.0.value.as_slice()).expect("Invalid UTF-8 Sequence");
        return write!(f, "{}", format!("b_{block_id_string}"));
    }
}

struct DataStore;

impl DataStore {
    async fn create(args: &NodeConfig, genesis_id: BlockId) {
        let _data_dir = args
            .data_dir
            .replace("{genesisBlockId}", genesis_id.to_string().as_str());

        let _db = rusty_leveldb::DB::open("strata-node", rusty_leveldb::Options::default());
    }
}
