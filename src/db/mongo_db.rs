use super::config::DbConfig;
use crate::db::crud_trait::HistoryCRUD;
use crate::models::allowed_model::AllowedModel;

use async_trait::async_trait; // Ensure you import async_trait
use futures::StreamExt;
use mongodb::{
    bson::{self, doc, to_document, Bson, Document},
    error::{Error, ErrorKind, WriteFailure},
    Client, Collection, Database,
}; // Import Document
use std::error::Error as StdError;
use std::io;

pub struct MongoDbStore {
    database: Database,
}

impl MongoDbStore {
    pub async fn new(config: &DbConfig) -> Self {
        println!("Connecting to {} Database", &config.db_type);
        // Create a MongoDB client
        let client = Client::with_uri_str(&config.connection_string)
            .await
            .expect("Failed to initialize MongoDB client");

        // Get the specified database
        let database = client.database(&config.database_name);

        MongoDbStore { database }
    }
}

#[async_trait]
impl HistoryCRUD for MongoDbStore {
    async fn read_history(
        &self,
        collection_name: &str,
        start_epoch: Option<i64>,
        end_epoch: Option<i64>,
        liquidity_gt: Option<i64>,
        sort_by: String,
        order: String,
        page: u32,
        limit: u32,
        count: u32,
        interval: Option<String>,
    ) -> Result<Vec<AllowedModel>, Box<dyn StdError>> {
        let mut pipeline = Vec::new();

        if count != 30 && interval.is_none() {
            return Err("Count parameter can only be used with interval".into());
        }
        println!("{} {}", count, limit);

        // Build initial match stage combining all filters
        let mut match_conditions = doc! {};

        // Add date range conditions if provided
        if let (Some(start), Some(end)) = (start_epoch, end_epoch) {
            match_conditions.insert(
                "$and",
                vec![
                    doc! {
                        "startTime": { "$gte": start }
                    },
                    doc! {
                        "endTime": { "$lte": end }
                    },
                ],
            );
        }

        // Add liquidity filter if provided
        if let Some(liquidity_gt_value) = liquidity_gt {
            match_conditions.insert(
                "liquidityUnits",
                doc! {
                    "$gt": Bson::Int64(liquidity_gt_value)
                },
            );
        }

        // Only add match stage if we have conditions
        if !match_conditions.is_empty() {
            pipeline.push(doc! {
                "$match": match_conditions
            });
        }

        // Handle interval grouping if specified
        if let Some(interval_value) = interval {
            let time_unit = match interval_value.as_str() {
                "5min" => 5 * 60,
                "hour" => 60 * 60,
                "day" => 24 * 60 * 60,
                "week" => 7 * 24 * 60 * 60,
                "month" => 30 * 24 * 60 * 60,
                "quarter" => 3 * 30 * 24 * 60 * 60,
                "year" => 365 * 24 * 60 * 60,
                _ => 60 * 60, // Default to 1 hour
            };

            // Group by time intervals
            pipeline.push(doc! {
                "$group": {
                    "_id": {
                        "$toInt": { "$divide": ["$startTime", time_unit] }
                    },
                    "documents": { "$push": "$$ROOT" }
                }
            });

            // Add sorting before limiting the buckets
            let sort_order = if order.to_lowercase() == "asc" { 1 } else { -1 };
            pipeline.push(doc! {
                "$sort": {
                    "_id": sort_order
                }
            });

            // Now unwind the buckets to get individual documents
            pipeline.extend(vec![
                doc! {
                    "$unwind": "$documents"
                },
                doc! {
                    "$replaceRoot": { "newRoot": "$documents" }
                },
            ]);

            // Sort the individual documents
            pipeline.push(doc! {
                "$sort": {
                    sort_by: sort_order
                }
            });
        } else {
            // If no interval, just sort normally
            let sort_order = if order.to_lowercase() == "asc" { 1 } else { -1 };
            pipeline.push(doc! {
                "$sort": {
                    sort_by: sort_order
                }
            });
        }

        // count is has higher order precedence

        let limit_val = if count != 30 { count } else { limit };
        let skip_value = (page - 1) * limit_val;
        pipeline.extend(vec![
            doc! { "$skip": skip_value as i64 },
            doc! { "$limit": limit_val as i64  },
        ]);

        // Execute aggregation and collect results
        let collection = self.database.collection::<Document>(collection_name);
        let mut cursor = collection.aggregate(pipeline, None).await?;
        let mut histories = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => match bson::from_document::<AllowedModel>(document) {
                    Ok(history) => histories.push(history),
                    Err(e) => eprintln!("Deserialization error: {:?}", e),
                },
                Err(e) => eprintln!("Cursor error: {:?}", e),
            }
        }

        Ok(histories)
    }

    async fn create_history(
        &self,
        collection_name: &str,
        history: AllowedModel,
    ) -> Result<AllowedModel, Box<dyn StdError>> {
        println!("entered create_history");
        println!("{:#?}", history);
        let collection = self.database.collection(collection_name);

        // Attempt to insert the document
        let result = collection.insert_one(history.clone(), None).await;

        // Handle the result with match
        match result {
            Ok(insert_result) => {
                let _inserted_id = insert_result.inserted_id.as_object_id().ok_or_else(|| {
                    Error::from(io::Error::new(
                        io::ErrorKind::Other,
                        "Failed to get inserted ObjectId",
                    ))
                })?;
                Ok(history)
            }
            Err(ref e) => {
                // Borrow the error and check if it's a Write error
                if let ErrorKind::Write(ref write_failure) = *e.kind {
                    if let WriteFailure::WriteError(ref write_error) = write_failure {
                        // Check for duplicate key error code
                        if write_error.code == 11000 {
                            // Handle duplicate key error by returning a specific message
                            return Err(Box::new(io::Error::new(
                                io::ErrorKind::AlreadyExists,
                                "Object Already Exists", // Custom error message
                            )));
                        }
                    }
                }
                // For other errors, return the error as is
                Err(Box::new(e.clone())) // Return the original error
            }
        }
    }

    async fn create_batch_history(
        &self,
        collection_name: &str,
        histories: Vec<AllowedModel>, // Change to take a vector of AllowedModel
    ) -> Result<String, Box<dyn StdError>> {
        // Return a message instead of AllowedModel
        println!("entered create_batch_history");
        println!("{:#?}", histories);
        let collection: Collection<AllowedModel> = self.database.collection(collection_name);

        // Attempt to insert the documents
        let result = collection.insert_many(histories.clone(), None).await;

        // Handle the result with match
        match result {
            Ok(_) => {
                // If successful, return a success message
                Ok("All records inserted successfully.".to_string())
            }
            Err(ref e) => {
                // Borrow the error and check if it's a Write error
                if let ErrorKind::Write(ref write_failure) = *e.kind {
                    if let WriteFailure::WriteError(ref write_error) = write_failure {
                        // Check for duplicate key error code
                        if write_error.code == 11000 {
                            // Handle duplicate key error by returning a specific message
                            return Err(Box::new(io::Error::new(
                                io::ErrorKind::AlreadyExists,
                                "One or more objects already exist", // Custom error message
                            )));
                        }
                    }
                }
                // For other errors, return the error as is
                Err(Box::new(e.clone())) // Return the original error
            }
        }
    }

    async fn update_history(
        &self,
        collection_name: &str,
        history: AllowedModel,
    ) -> Result<AllowedModel, Box<dyn std::error::Error>> {
        let collection: Collection<AllowedModel> = self.database.collection(collection_name);
        let hist_id = match &history {
            AllowedModel::DepthHistory(depth_history) => &depth_history.hist_id,
            // Handle other variants if necessary
            _ => return Err("Unsupported AllowedModel variant".into()),
        };
        let filter = doc! { "histId": hist_id };
        let update = doc! { "$set": to_document(&history).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)? };
        collection.update_one(filter, update, None).await?;
        Ok(history) // Return the updated p
    }

    async fn delete_history(
        &self,
        collection_name: &str,
        hist_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection: Collection<AllowedModel> = self.database.collection(collection_name);
        let filter = doc! { "histId": hist_id };
        collection.delete_one(filter, None).await?;
        Ok(())
    }
}
