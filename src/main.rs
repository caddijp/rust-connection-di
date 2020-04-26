use anyhow::Result;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iss_api_url = "http://api.open-notify.org/iss-now.json".to_string();

    let mut runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async move {
        let body = match get_iss_now(iss_api_url).await {
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

async fn get_iss_now(path: String) -> Result<String> {
    let body = reqwest::get(&path).await?.text().await?;
    Ok(body)
}
