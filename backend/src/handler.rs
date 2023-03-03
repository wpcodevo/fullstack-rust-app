use crate::{
    model::FeedbackModel,
    schema::{CreateFeedbackSchema, FilterOptions, UpdateFeedbackSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build API with Rust, SQLX, Postgres,and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/feedbacks")]
pub async fn feedback_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        FeedbackModel,
        "SELECT * FROM feedbacks ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all feedback items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let feedbacks = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": feedbacks.len(),
        "feedbacks": feedbacks
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/feedbacks/")]
async fn create_feedback_handler(
    body: web::Json<CreateFeedbackSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        FeedbackModel,
        "INSERT INTO feedbacks (text,rating) VALUES ($1, $2) RETURNING *",
        body.text.to_string(),
        body.rating,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(feedback) => {
            let feedback_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "feedback": feedback
            })});

            return HttpResponse::Ok().json(feedback_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Feedback with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/feedbacks/{id}")]
async fn get_feedback_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let feedback_id = path.into_inner();
    let query_result = sqlx::query_as!(
        FeedbackModel,
        "SELECT * FROM feedbacks WHERE id = $1",
        feedback_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(feedback) => {
            let feedback_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "feedback": feedback
            })});

            return HttpResponse::Ok().json(feedback_response);
        }
        Err(_) => {
            let message = format!("feedback with ID: {} not found", feedback_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[patch("/feedbacks/{id}")]
async fn edit_feedback_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateFeedbackSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let feedback_id = path.into_inner();
    let query_result = sqlx::query_as!(
        FeedbackModel,
        "SELECT * FROM feedbacks WHERE id = $1",
        feedback_id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("Feedback with ID: {} not found", feedback_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let now = Utc::now();
    let feedback = query_result.unwrap();

    let query_result = sqlx::query_as!(
        FeedbackModel,
        "UPDATE feedbacks SET text = $1, rating = $2, updated_at = $3 WHERE id = $4 RETURNING *",
        body.text.to_owned().unwrap_or(feedback.text),
        body.rating.to_owned().unwrap_or(feedback.rating),
        now,
        feedback_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(feedback) => {
            let feedback_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "feedback": feedback
            })});

            return HttpResponse::Ok().json(feedback_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/feedbacks/{id}")]
async fn delete_feedback_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let feedback_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM feedbacks  WHERE id = $1", feedback_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Feedback with ID: {} not found", feedback_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(feedback_list_handler)
        .service(create_feedback_handler)
        .service(get_feedback_handler)
        .service(edit_feedback_handler)
        .service(delete_feedback_handler);

    conf.service(scope);
}
