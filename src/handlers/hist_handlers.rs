use crate::db::crud_trait::HistoryCRUD;
use crate::models::{allowed_model::AllowedModel, history::HistoryQuery};
use actix_web::{
    get, post, put,
    web::{self, Data, Json},
    HttpResponse,
};
use chrono::NaiveDate;
use std::error::Error;

#[get("/get-depth-history")]
pub async fn get_depth_history(
    db: Data<dyn HistoryCRUD>,
    query: web::Query<HistoryQuery>,
) -> HttpResponse {
    let query_params = query.into_inner();

    // Extract pagination, sorting, and filtering parameters
    let date_range = query_params.date_range.clone();
    let liquidity_gt = query_params.liquidity_gt;

    let sort_by = query_params
        .sort_by
        .clone()
        .unwrap_or_else(|| "timestamp".to_string());

    let order = query_params
        .order
        .clone()
        .unwrap_or_else(|| "asc".to_string());
    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let count = query_params.count.unwrap_or(30);
    let interval = query_params.interval;
    // gives preference to date_range before from,to
    let (start_epoch, end_epoch) = if date_range.is_none() {
        // If date_range is None, don't apply date filters
        if query_params.from.is_none() && query_params.to.is_none() {
            (None, None)
        } else {
            (
                Some(query_params.from.unwrap_or(0)),
                Some(query_params.to.unwrap_or(chrono::Utc::now().timestamp())),
            )
        }
    } else {
        match parse_date_range_to_epoch(date_range.clone()) {
            Ok((start, end)) => (Some(start), Some(end)), // If parsing is successful, set date ranges
            Err(_) => {
                println!("Invalid date range format");
                return HttpResponse::BadRequest().body("Invalid date range"); // Return error if invalid
            }
        }
    };
    println!("{:#?} {:#?}", start_epoch, end_epoch);
    match db
        .read_history(
            "depth_history",
            start_epoch,
            end_epoch,
            liquidity_gt,
            sort_by,
            order,
            page,
            limit,
            count,
            interval,
        )
        .await
    {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get-earnings-history")]
pub async fn get_earnings_history(
    db: Data<dyn HistoryCRUD>,
    query: web::Query<HistoryQuery>,
) -> HttpResponse {
    let query_params = query.into_inner();

    // Extract pagination, sorting, and filtering parameters
    let date_range = query_params.date_range.clone();
    let liquidity_gt = query_params.liquidity_gt;

    let sort_by = query_params
        .sort_by
        .clone()
        .unwrap_or_else(|| "timestamp".to_string());

    let order = query_params
        .order
        .clone()
        .unwrap_or_else(|| "asc".to_string());
    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let count = query_params.count.unwrap_or(30);
    let interval = query_params.interval;
    // gives preference to date_range before from,to
    let (start_epoch, end_epoch) = if date_range.is_none() {
        // If date_range is None, don't apply date filters
        if query_params.from.is_none() && query_params.to.is_none() {
            (None, None)
        } else {
            (
                Some(query_params.from.unwrap_or(0)),
                Some(query_params.to.unwrap_or(chrono::Utc::now().timestamp())),
            )
        }
    } else {
        match parse_date_range_to_epoch(date_range.clone()) {
            Ok((start, end)) => (Some(start), Some(end)), // If parsing is successful, set date ranges
            Err(_) => {
                println!("Invalid date range format");
                return HttpResponse::BadRequest().body("Invalid date range"); // Return error if invalid
            }
        }
    };
    println!("{:#?} {:#?}", start_epoch, end_epoch);
    match db
        .read_history(
            "earnings_history",
            start_epoch,
            end_epoch,
            liquidity_gt,
            sort_by,
            order,
            page,
            limit,
            count,
            interval,
        )
        .await
    {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get-swap-history")]
pub async fn get_swap_history(
    db: Data<dyn HistoryCRUD>,
    query: web::Query<HistoryQuery>,
) -> HttpResponse {
    let query_params = query.into_inner();

    // Extract pagination, sorting, and filtering parameters
    let date_range = query_params.date_range.clone();
    let liquidity_gt = query_params.liquidity_gt;

    let sort_by = query_params
        .sort_by
        .clone()
        .unwrap_or_else(|| "timestamp".to_string());

    let order = query_params
        .order
        .clone()
        .unwrap_or_else(|| "asc".to_string());
    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let count = query_params.count.unwrap_or(30);
    let interval = query_params.interval;
    // gives preference to date_range before from,to
    let (start_epoch, end_epoch) = if date_range.is_none() {
        // If date_range is None, don't apply date filters
        if query_params.from.is_none() && query_params.to.is_none() {
            (None, None)
        } else {
            (
                Some(query_params.from.unwrap_or(0)),
                Some(query_params.to.unwrap_or(chrono::Utc::now().timestamp())),
            )
        }
    } else {
        match parse_date_range_to_epoch(date_range.clone()) {
            Ok((start, end)) => (Some(start), Some(end)), // If parsing is successful, set date ranges
            Err(_) => {
                println!("Invalid date range format");
                return HttpResponse::BadRequest().body("Invalid date range"); // Return error if invalid
            }
        }
    };
    println!("{:#?} {:#?}", start_epoch, end_epoch);
    match db
        .read_history(
            "swap_history",
            start_epoch,
            end_epoch,
            liquidity_gt,
            sort_by,
            order,
            page,
            limit,
            count,
            interval,
        )
        .await
    {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get-runepool-history")]
pub async fn get_runepool_history(
    db: Data<dyn HistoryCRUD>,
    query: web::Query<HistoryQuery>,
) -> HttpResponse {
    let query_params = query.into_inner();

    // Extract pagination, sorting, and filtering parameters
    let date_range = query_params.date_range.clone();
    let liquidity_gt = query_params.liquidity_gt;

    let sort_by = query_params
        .sort_by
        .clone()
        .unwrap_or_else(|| "timestamp".to_string());

    let order = query_params
        .order
        .clone()
        .unwrap_or_else(|| "asc".to_string());
    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let count = query_params.count.unwrap_or(30);
    let interval = query_params.interval;
    // gives preference to date_range before from,to
    let (start_epoch, end_epoch) = if date_range.is_none() {
        // If date_range is None, don't apply date filters
        if query_params.from.is_none() && query_params.to.is_none() {
            (None, None)
        } else {
            (
                Some(query_params.from.unwrap_or(0)),
                Some(query_params.to.unwrap_or(chrono::Utc::now().timestamp())),
            )
        }
    } else {
        match parse_date_range_to_epoch(date_range.clone()) {
            Ok((start, end)) => (Some(start), Some(end)), // If parsing is successful, set date ranges
            Err(_) => {
                println!("Invalid date range format");
                return HttpResponse::BadRequest().body("Invalid date range"); // Return error if invalid
            }
        }
    };
    println!("{:#?} {:#?}", start_epoch, end_epoch);
    match db
        .read_history(
            "runepool_history",
            start_epoch,
            end_epoch,
            liquidity_gt,
            sort_by,
            order,
            page,
            limit,
            count,
            interval,
        )
        .await
    {
        Ok(histories) => HttpResponse::Ok().json(histories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/add-depth-history")]
pub async fn add_depth_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    let history = history.into_inner();
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

#[put("/update-swap-history")]
pub async fn update_swap_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    let history = history.into_inner();
    match db.update_history("swap_history", history).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/update-earnings-history")]
pub async fn update_earnings_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    let history = history.into_inner();
    match db.update_history("earnings_history", history).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/update-runepool-history")]
pub async fn update_runepool_history(
    db: Data<dyn HistoryCRUD>,
    history: Json<AllowedModel>,
) -> HttpResponse {
    let history = history.into_inner();
    match db.update_history("runepool_history", history).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

fn parse_date_range_to_epoch(date_range: Option<String>) -> Result<(i64, i64), Box<dyn Error>> {
    if let Some(range) = date_range {
        let dates: Vec<&str> = range.split(',').collect();

        if dates.len() == 2 {
            let start = if dates[0].is_empty() {
                // Use the Unix epoch if the start date is empty
                NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0).timestamp()
            } else {
                // Parse the provided start date
                NaiveDate::parse_from_str(dates[0], "%Y-%m-%d")?
                    .and_hms(0, 0, 0)
                    .timestamp()
            };

            let end = if dates[1].is_empty() {
                // Use the current time if the end date is empty
                chrono::Utc::now().timestamp()
            } else {
                // Parse the provided end date
                NaiveDate::parse_from_str(dates[1], "%Y-%m-%d")?
                    .and_hms(23, 59, 59)
                    .timestamp()
            };

            return Ok((start, end));
        }
    }

    Err("Invalid date range format".into())
}
