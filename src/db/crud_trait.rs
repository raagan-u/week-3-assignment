use crate::models::allowed_model::AllowedModel;

#[async_trait::async_trait]
pub trait HistoryCRUD: Send + Sync {
    async fn read_history(
        &self,
        collection_name: &str,
        start_epoch: Option<i64>,
        end_epoch: Option<i64>,
        sort_by: String,
        order: String,
        page: u32,
        limit: u32,
        count: u32,
        interval: Option<String>,
        cmp_field: String,
        cmp_units: Option<i64>,
        cmp_op: String,
    ) -> Result<Vec<AllowedModel>, Box<dyn std::error::Error>>;

    async fn create_history(
        &self,
        collection_name: &str,
        history: AllowedModel,
    ) -> Result<AllowedModel, Box<dyn std::error::Error>>;

    async fn create_batch_history(
        &self,
        collection_name: &str,
        histories: Vec<AllowedModel>,
    ) -> Result<String, Box<dyn std::error::Error>>;

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
