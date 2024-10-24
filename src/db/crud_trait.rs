use crate::models::allowed_model::AllowedModel;

#[async_trait::async_trait]
pub trait HistoryCRUD: Send + Sync {
    async fn read_history(
        &self,
        collection_name: &str,
    ) -> Result<Vec<AllowedModel>, Box<dyn std::error::Error>>;

    async fn create_history(
        &self,
        collection_name: &str,
        history: AllowedModel,
    ) -> Result<AllowedModel, Box<dyn std::error::Error>>;

    async fn update_history(
        &self,
        collection_name: &str,
        history: AllowedModel,
    ) -> Result<AllowedModel, Box<dyn std::error::Error>>;

    async fn delete_history(
        &self,
        collection_name: &str,
        hist_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
