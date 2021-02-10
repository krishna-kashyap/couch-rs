use serde::{Deserialize, Serialize};

/// Couch vendor abstraction
#[derive(Serialize, Deserialize, Debug)]
pub struct CouchVendor {
    pub name: String,
    pub version: Option<String>,
}

/// Couch status abstraction
#[derive(Serialize, Deserialize, Debug)]
pub struct CouchStatus {
    pub couchdb: String,
    pub git_sha: Option<String>,
    pub uuid: Option<String>,
    pub version: String,
    pub vendor: CouchVendor,
}

/// Couch response abstraction
#[derive(Serialize, Deserialize, Debug)]
pub struct CouchResponse {
    pub ok: Option<bool>,
    pub error: Option<String>,
    pub reason: Option<String>,
}

/// Cluster information
#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterInfo {
    pub n: u32,
    pub q: u32,
    pub r: u32,
    pub w: u32,
}

/// Size information
#[derive(Serialize, Deserialize, Debug)]
pub struct SizeInfo {
    pub active: u64,
    pub external: u64,
    pub file: Option<u64>, // Set as optional so as to reuse with partition info whichc doesn't contain file property
}

/// Database information
#[derive(Serialize, Deserialize, Debug)]
pub struct DbProperties {
    pub partitioned: Option<bool>,
}

/// Database information
#[derive(Serialize, Deserialize, Debug)]
pub struct DbInfo {
    pub cluster: ClusterInfo,
    pub compact_running: bool,
    pub db_name: String,
    pub disk_format_version: u32,
    pub doc_count: u64,
    pub doc_del_count: u64,
    pub instance_start_time: String,
    pub purge_seq: String,
    pub sizes: SizeInfo,
    pub update_seq: String,
    pub props: DbProperties,
}

/// Partition information
#[derive(Serialize, Deserialize, Debug)]
pub struct PartitionInfo {
    pub db_name: String,
    pub partition: String,
    pub doc_count: u64,
    pub doc_del_count: u64,
    pub sizes: SizeInfo,
}
