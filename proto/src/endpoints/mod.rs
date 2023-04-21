use edu_subxt::chain::DatahighwayOnlineClient;

pub mod university;

pub struct Backend {
    node_client: DatahighwayOnlineClient,
}

impl Backend {
    pub fn new(node_client: DatahighwayOnlineClient) -> Self {
        Self { node_client }
    }
}
