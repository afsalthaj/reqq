use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

#[tokio::main]
async fn main() {
    let url = "https://x.api.golem.cloud/api/v3/user";

    let client = reqwest::Client::new();

    // JSON body
    let json_body = json!({
        "id": "afsalthaj",
        "name": "AfsalThaj",
        "email": "adam@golem.cloud"
    });

    let response = client
        .post(url)
        .header(ACCEPT, "application/json")
        .json(&json_body)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let id_out_result = response.text().await;
                match id_out_result {
                    Ok(id_out) => {
                        println!("Response ID: {}", id_out);
                    }
                    Err(e) => {
                        println!("Failed to read response ID: {}", e);
                    }
                }
            } else {
                println!("Request failed with status: {}", response.status());
            }
        }
        Err(e) => {
            println!("Request failed with error: {}", e);
        }
    }
}
