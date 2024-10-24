use reqwest::Client;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = "https://midgard.ninerealms.com/v2/history/runepool";

    // Create a reqwest client with TLS verification disabled
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // WARNING: This disables SSL verification
        .build()?;

    // Fetch the data from the API
    let response = client.get(api_url).send().await?;

    if response.status().is_success() {
        let body: Value = response.json().await?;

        // Extract intervals from the response
        if let Some(intervals) = body["intervals"].as_array() {
            for interval in intervals {
                // Create hist_id (you can use a UUID or any other logic)

                // Create a mutable copy of the interval and add the hist_id
                let mut interval_with_id = interval.clone();
                interval_with_id["histId"] = Value::from(989008);
                // Now post the updated interval to your endpoint
                println!("{:#?}", interval_with_id);
                post_interval(&client, &interval_with_id).await?;
            }
        } else {
            eprintln!("No intervals found in the response.");
        }
    } else {
        eprintln!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}

// Function to post the interval to your API
async fn post_interval(client: &Client, interval: &Value) -> Result<(), Box<dyn Error>> {
    println!("{:#?}", interval);
    let api_url = "http://127.0.0.1:8080/add-runepool-history"; // Replace with your actual API URL

    let response = client
        .post(api_url)
        .json(&interval) // Send the interval as JSON
        .send()
        .await?;

    if response.status().is_success() {
        println!("Posted interval successfully: {:#?}", interval);
    } else {
        eprintln!("Failed to post interval: {}", response.status());
    }

    Ok(())
}
