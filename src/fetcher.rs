use reqwest::blocking::Client as HttpClient;
use serde_json::{json, Value};
use std::error::Error;

fn fetch_last_depth_history() -> (Option<i64>, Option<i64>) {
    let url = "http://localhost:8080/get-earnings-history?sort_by=histId&order=desc&limit=1";
    let client = HttpClient::new();

    if let Ok(response) = client.get(url).send() {
        if let Ok(json) = response.json::<Value>() {
            if let Some(data) = json.get(0) {
                let hist_id = data.get("histId").and_then(|v| v.as_i64());
                let end_time = data.get("endTime").and_then(|v| v.as_i64());
                return (hist_id, end_time);
            }
        }
    }

    (None, None) // Return None for both if the request or parsing fails
}

fn fetch_swaps(url: String) -> Result<Value, reqwest::Error> {
    let client = HttpClient::new();
    let response = client.get(url).send()?;
    let json = response.json::<Value>()?;
    Ok(json)
}

fn insert_interval(
    url: &str,
    client: &HttpClient,
    interval: Vec<Value>,
) -> Result<(), Box<dyn Error>> {
    let response = client.post(url).json(&interval).send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response
            .text()
            .unwrap_or_else(|_| "Failed to retrieve error message".to_string());
        eprintln!("Error inserting interval at {}: {}", url, error_message);
        Err(format!("HTTP error at {}: {}", url, error_message).into())
    }
}

fn main() {
    let http_client = HttpClient::new();
    let (hist_id_c, end_time_c) = fetch_last_depth_history();
    println!(
        "extracted {} and {}",
        hist_id_c.unwrap(),
        end_time_c.unwrap()
    );
    // Define your fetch URLs
    let fetch_urls = vec![
        "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count=400&from=",
        "https://midgard.ninerealms.com/v2/history/earnings?interval=hour&count=400&from=",
        "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&count=400&from=",
        "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count=400&from=",
    ];

    // Define your insertion URLs
    let insertion_urls = vec![
        "http://127.0.0.1:8080/add-depth-batch-history",
        "http://127.0.0.1:8080/add-earnings-batch-history",
        "http://127.0.0.1:8080/add-swaps-batch-history",
        "http://127.0.0.1:8080/add-runepool-batch-history",
    ];

    let mut intervals_vec: Vec<Vec<Value>> = vec![Vec::new(); fetch_urls.len()];

    // Fetch data from each URL
    for (i, fetch_url) in fetch_urls.iter().enumerate() {
        let mut hist_id: i64 = 1;
        let mut from: i64 = 1;
        if let (Some(h), Some(f)) = (hist_id_c, end_time_c) {
            hist_id = h + 1;
            from = f;
            println!("histId: {}, endTime: {}", hist_id, from);
        } else {
            eprintln!("Failed to retrieve histId or endTime.");
        }
        let fetch_url_with_from = format!("{}{}", fetch_url, from);
        match fetch_swaps(fetch_url_with_from) {
            Ok(response) => {
                if let Some(intervals) = response.get("intervals").and_then(|d| d.as_array()) {
                    for interval in intervals {
                        let mut new_interval = interval.clone();
                        new_interval["histId"] = json!(hist_id);
                        intervals_vec[i].push(new_interval);
                        hist_id += 1;
                    }
                } else {
                    eprintln!("No intervals found in the response from {}.", fetch_url);
                }
            }
            Err(err) => {
                eprintln!("Error fetching data from {}: {}", fetch_url, err);
            }
        }
    }

    // Insert the fetched intervals into their corresponding URLs
    for (i, insertion_url) in insertion_urls.iter().enumerate() {
        let pools_to_post = if i == 1 {
            if let Some(obj) = intervals_vec[i][0].as_object_mut() {
                // Extract the pools data
                obj.remove("pools")
            } else {
                None
            }
        } else {
            None
        };
        // Insert the interval data (without pools)
        if let Err(err) = insert_interval(insertion_url, &http_client, intervals_vec[i].clone()) {
            eprintln!("Error inserting intervals into {}: {}", insertion_url, err);
        } else {
            println!("Inserted intervals into {} successfully.", insertion_url);
        }

        // Post pools to the secondary endpoint if pools were removed
        if let Some(pools) = pools_to_post {
            if let Some(pools_array) = pools.as_array() {
                // Define the endpoint for posting pools
                let pools_endpoint = "http://localhost:8080/add-pool-batch";

                let mut processed_pools: Vec<Value> = Vec::new();
                let hist_id = hist_id_c.unwrap(); // Or another source for hist_id, depending on your setup
                let mut pool_id = 1;

                // Iterate through each pool to add pool_id and hist_id
                for pool in pools_array {
                    println!("pool => {:#?}\n", pool);
                    let mut pool_with_ids = pool.clone();
                    pool_with_ids["poolId"] = json!(pool_id);
                    pool_with_ids["histId"] = json!(hist_id);
                    pool_id += 1;
                    processed_pools.push(pool_with_ids);
                }
                // Post the modified pools array to the endpoint
                if processed_pools.len() == 0 {
                    println!("No Pools To post");
                } else {
                    if let Err(err) = insert_interval(pools_endpoint, &http_client, processed_pools)
                    {
                        eprintln!("Error inserting pools into {}: {}", pools_endpoint, err);
                    } else {
                        println!("Inserted pools into {} successfully.", pools_endpoint);
                    }
                }
            }
        }
    }
}
