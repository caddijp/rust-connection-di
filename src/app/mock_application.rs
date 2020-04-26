use crate::r#trait::{ExternalApiClient, ProvideExternalApiClient};

pub struct MockExternalApiClient {
    url: String,
}

pub struct MockApplication {
    external_api_client: MockExternalApiClient,
    // 以下に、pg_poolなど他に必要な設定が出たタイミングで設定します
}

impl MockExternalApiClient {
    pub fn new() -> Self {
        Self {
            url: "http://localhost:3000/iss-now".to_string(),
        }
    }
}

impl MockApplication {
    pub fn new(external_api_client: MockExternalApiClient) -> Self {
        Self {
            external_api_client,
        }
    }
}

impl ExternalApiClient for MockExternalApiClient {
    fn url(&self) -> String {
        self.url.clone()
    }
}

impl ProvideExternalApiClient for MockApplication {
    type Config = MockExternalApiClient;

    fn provide(&self) -> &Self::Config {
        &self.external_api_client
    }
}
