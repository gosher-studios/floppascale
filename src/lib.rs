pub struct StorageNode {
    pub url: hyper::Uri,
    pub allows_dedup: bool,
    pub allows_rebalance: bool,
    pub size_left: usize,
}
