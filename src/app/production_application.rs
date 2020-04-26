use crate::r#trait::{ExternalApiClient, ProvideExternalApiClient};

pub struct ProductionExternalApiClient {
    url: String,
}

pub struct Application {
    external_api_client: ProductionExternalApiClient,
    // 以下に、pg_poolなど他に必要な設定が出たタイミングで設定します
}

impl ProductionExternalApiClient {
    pub fn new() -> Self {
        Self {
            url: "http://api.open-notify.org/iss-now.json".to_string(),
        }
    }
}

impl Application {
    pub fn new(config: ProductionExternalApiClient) -> Self {
        Self {
            external_api_client: config,
        }
    }
}

impl ExternalApiClient for ProductionExternalApiClient {
    fn url(&self) -> String {
        self.url.clone()
    }
}

impl ProvideExternalApiClient for Application {
    type Config = ProductionExternalApiClient;

    fn provide(&self) -> &Self::Config {
        &self.external_api_client
    }
}
