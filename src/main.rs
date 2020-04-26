pub mod app;
pub mod r#trait;

use app::{Application, ProductionExternalApiClient};
use r#trait::{ExternalApiClient, ProvideExternalApiClient};

use anyhow::Result;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ProductionExternalApiClient::new();
    let app = Application::new(config);

    let mut runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async move {
        let body = match get_iss_now(&app).await {
            Ok(val) => val,
            Err(_) => {
                exit(1);
            }
        };

        println!("body = {:?}", body);

        loop {
            tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
        }
    })
}

async fn get_iss_now<T>(ctx: &T) -> Result<String>
where
    T: ProvideExternalApiClient,
{
    let url = ctx.provide().url();
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}
