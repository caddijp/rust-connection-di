pub trait ExternalApiClient {
    fn url(&self) -> String;
}

pub trait ProvideExternalApiClient {
    type Config: ExternalApiClient;

    fn provide(&self) -> &Self::Config;
}
