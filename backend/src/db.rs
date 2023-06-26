use std::sync::{Arc, Mutex};

use shared::question::Question;

// Mock database only
#[derive(Clone, Default)]
pub struct AppDatabase {
    pub questions: Arc<Mutex<Vec<Question>>>,
}
