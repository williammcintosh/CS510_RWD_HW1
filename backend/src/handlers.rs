use axum::extract::{Query, State};
use axum::Json;

use shared::question::{CreateQuestion, GetQuestionById, Question, QuestionId};

use crate::db::AppDatabase;
use crate::http_error::{AppError, QuestionError};

// Basic hello-world route
pub async fn root() -> String {
    "Hello World!".into()
}

pub async fn get_questions(
    State(am_database): State<AppDatabase>,
) -> Result<Json<Vec<Question>>, AppError> {
    let mut questions = am_database.questions.lock().expect("Poisoned mutex");
    let db_count = questions.len() as usize;
    let question = Question::new(
        QuestionId(db_count),
        "Default Question".to_string(),
        "Default Content".to_string(),
        Some(vec!["default".to_string()]),
    );

    (*questions).push(question.clone());

    let all_questions: Vec<Question> = (*questions).clone();

    // This will currently always be true, of course
    #[allow(irrefutable_let_patterns)] //example code
    if let _ = question.id.0 as i32 {
        // If our question was found, we send it to the client
        Ok(Json(all_questions))
    } else {
        // Otherwise we send our error instead.
        // This will auto-populate the message for us
        Err(AppError::Question(QuestionError::InvalidId))
    }
}

pub async fn get_question_by_id(
    State(am_database): State<AppDatabase>,
    Query(query): Query<GetQuestionById>,
) -> Result<Json<Question>, AppError> {
    let questions = am_database.questions.lock().expect("Poisoned mutex");

    let result_question = (*questions).iter().find(|&q| q.id.0 == query.question_id);

    if let Some(question) = result_question {
        Ok(Json(question.clone()))
    } else {
        Err(AppError::Question(QuestionError::InvalidId))
    }
}

pub async fn create_question(
    State(am_database): State<AppDatabase>,
    Json(question): Json<CreateQuestion>,
) -> Result<Json<Question>, AppError> {
    let mut questions = am_database.questions.lock().expect("Poisoned mutex");
    let db_count = questions.len() as usize;
    let question_with_id = Question::new(
        QuestionId(db_count),
        question.content,
        question.title,
        question.tags,
    );
    // Must clone question to use it later
    (*questions).push(question_with_id.clone());

    // Send back the new question
    Ok(Json(question_with_id))
}

pub async fn delete_question(
    State(am_database): State<AppDatabase>,
    Query(query): Query<GetQuestionById>,
) -> Result<(), AppError> {
    let mut questions = am_database.questions.lock().expect("Poisoned mutex");

    questions.retain(|q| q.id.0 == query.question_id);
    Ok(())
}
