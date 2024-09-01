use reqwest::header::{ACCEPT, AUTHORIZATION};

#[tokio::main]
async fn main() {
    let url = "https://ribb.api.golem.cloud/v8/connect/afsal";
    let token = "mytoken";

    // Create a client
    let client = reqwest::Client::new();

    // Send GET request
    let response = client
        .post(url)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await.unwrap();

    // Check if the response status is successful
    if response.status().is_success() {
        // Parse the response body as JSON (or handle it as needed)
        let json: serde_json::Value = response.json().await.unwrap();
        println!("Response JSON: {:?}", json);
    } else {
        println!("Request failed with status: {}", response.status());
    }
}
