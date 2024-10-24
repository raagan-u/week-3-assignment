use super::config::DbConfig;
use crate::db::crud_trait::HistoryCRUD;
use crate::models::allowed_model::AllowedModel;

use async_trait::async_trait; // Ensure you import async_trait
use futures::StreamExt;
use mongodb::{
    bson::{self, doc, to_document, Document},
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
    ) -> Result<Vec<AllowedModel>, Box<dyn StdError>> {
        let collection = self.database.collection::<Document>(collection_name); // Specify Document type
        let mut cursor = collection.find(None, None).await?;
        let mut histories = Vec::new();

        println!("something");
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    println!("{:#?}", document);
                    // Attempt to deserialize the document into AllowedModel
                    match bson::from_document::<AllowedModel>(document) {
                        Ok(depth_history) => {
                            // Push the successfully deserialized model into items
                            histories.push(depth_history);
                        }
                        Err(e) => {
                            eprintln!("Error deserializing document to AllowedModel: {:?}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Error retrieving histories: {:?}", e),
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
        let filter = doc! { "hist_id": hist_id };
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
        let filter = doc! { "hist_id": hist_id };
        collection.delete_one(filter, None).await?;
        Ok(())
    }
}
