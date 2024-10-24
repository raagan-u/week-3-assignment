use crate::db::crud_trait::HistoryCRUD;
use crate::models::allowed_model::AllowedModel;
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json},
    HttpResponse,
};

#[get("/get-depth-history")]
pub async fn get_depth_history(db: Data<dyn HistoryCRUD>) -> HttpResponse {
    println!("entered get depth");
    match db.read_history("depth_history").await {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get-earnings-history")]
pub async fn get_earn_history(db: Data<dyn HistoryCRUD>) -> HttpResponse {
    println!("entered get earn");
    match db.read_history("earnings_history").await {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get-swap-history")]
pub async fn get_swap_history(db: Data<dyn HistoryCRUD>) -> HttpResponse {
    println!("entered get swap");
    match db.read_history("swap_history").await {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get-runepool-history")]
pub async fn get_runepool_history(db: Data<dyn HistoryCRUD>) -> HttpResponse {
    println!("entered get runepool");
    match db.read_history("runepool_history").await {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/add_depth_history")]
pub async fn add_depth_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    println!("entered addpet");
    let history = history.into_inner();
    println!("{:#?}", history);
    match db.create_history("depth_history", history).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/add-earnings-history")]
pub async fn add_earnings_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    let history = history.into_inner();
    match db.create_history("earnings_history", history).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/add-swap-history")]
pub async fn add_swap_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    let history = history.into_inner();
    match db.create_history("swap_history", history).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/add-runepool-history")]
pub async fn add_rune_pool_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    let history = history.into_inner();
    match db.create_history("runepool_history", history).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/update-depth-history")]
pub async fn update_depth_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    let history = history.into_inner();
    match db.update_history("depth_history", history).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/delete-depth-history/{hist_id}")]
pub async fn delete_depth_history(db: Data<dyn HistoryCRUD>, path: web::Path<i64>) -> HttpResponse {
    let hist_id = path.into_inner();
    match db.delete_history("depth_history", hist_id).await {
        Ok(_) => HttpResponse::Ok().body("History deleted successfully"),
        Err(e) => {
            eprintln!("Error deleting History: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete History")
        }
    }
}
