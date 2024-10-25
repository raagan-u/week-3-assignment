use reqwest::blocking::Client as HttpClient;
use reqwest::blocking::Response;
use serde_json::{json, Value};
use std::error::Error;
use std::time::Duration;

fn fetch_swaps(from: i64) -> Result<Value, reqwest::Error> {
    let url = format!(
        "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count=400&from={}",
        from
    );
    let client = HttpClient::new();
    let response = client.get(&url).send()?;
    let json = response.json::<Value>()?;
    Ok(json)
}

fn insert_interval(client: &HttpClient, interval: &Value) -> Result<(), Box<dyn Error>> {
    let url = "http://127.0.0.1:8080/add-depth-history"; // Replace with your actual endpoint URL

    let response = client
        .post(url)
        .json(interval) // Send the interval as JSON
        .send()?;

    if response.status().is_success() {
        println!("Inserted interval successfully.");
        Ok(())
    } else {
        let error_message = response
            .text()
            .unwrap_or_else(|_| "Failed to retrieve error message".to_string());
        eprintln!("Error inserting interval: {}", error_message);
        Err(format!("HTTP error: ").into())
    }
}

fn main() {
    let http_client = HttpClient::new(); // Create an HTTP client instance

    let mut from: i64 = 1647129600; // March 13, 2022, in epoch time
    let mut internal_vector: Vec<Value> = Vec::new();
    let today_time: i64 = 1729821362;
    let mut histid: i64 = 1;
    loop {
        match fetch_swaps(from) {
            Ok(response) => {
                if let Some(meta) = response.get("meta") {
                    if let Some(end_time) = meta.get("endTime").and_then(|v| v.as_i64()) {
                        if end_time >= today_time {
                            break; // Stop if endTime is greater than or equal to the current epoch
                        }
                        from = end_time; // Update from
                    }
                }

                if let Some(intervals) = response.get("intervals").and_then(|v| v.as_array()) {
                    for interval1 in intervals {
                        internal_vector.push(interval1.clone());
                    }
                }

                // Insert each interval into your endpoint

                for interval in &mut internal_vector {
                    // Use &mut to mutate the interval
                    // Add or update the histId field
                    interval["histId"] = json!(histid); // Directly assign the new value

                    // Print the updated interval object
                    println!("{:#?}", interval);

                    // Attempt to insert the interval
                    if let Err(err) = insert_interval(&http_client, interval) {
                        // Ensure you await the function
                        eprintln!("Error inserting interval: {}", err);
                    }

                    histid += 1; // Increment histid for the next interval
                }

                std::thread::sleep(Duration::from_secs(1)); // Delay to avoid overwhelming the server
            }
            Err(err) => {
                eprintln!("Error fetching data: {}", err);
                break;
            }
        }
    }

    println!("Total intervals fetched: {}", internal_vector.len());
}
