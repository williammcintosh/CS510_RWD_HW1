use axum::{
    http::{Response, StatusCode},
    routing::*,
    Router,
};
use hyper::Body;

use crate::db::AppDatabase;
use crate::handlers;

pub fn get_router() -> Router {
    let db = AppDatabase::default();
    Router::new()
        .route("/", get(handlers::root))
        .route("/questions", get(handlers::get_questions))
        .route("/question", get(handlers::get_question_by_id))
        .route("/question", post(handlers::create_question))
        .route("/question", delete(handlers::delete_question))
        // This is our catchall, MUST be at the end!
        .route("/*_", get(handle_404))
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested resource could not be found"))
        .unwrap()
}
