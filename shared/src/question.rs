use std::fmt;

use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize)]
#[display(
    fmt = "id: {}, title: {}, content: {}, tags: {:?}",
    id,
    title,
    content,
    tags
)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

#[derive(Clone, Debug, Display, Serialize, Deserialize)]
pub struct QuestionId(pub usize);

#[derive(Debug)]
pub struct ParseQuestionIdError;

impl fmt::Display for ParseQuestionIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse question id")
    }
}

impl std::error::Error for ParseQuestionIdError {}

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GetQuestionById {
    pub question_id: usize,
}

/* Manual impl (without derive_more)
impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}
*/
