use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Deserialize)]
pub struct QuizApiResponse {
    pub quizzes: Vec<ApiQuestion>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiQuestion {
    pub question: String,
    #[serde(rename = "badAnswers")]
    pub bad_answers: Vec<String>,
    pub answer: String,
}

#[derive(Debug, Clone)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub answer: usize,
}

impl From<ApiQuestion> for Question {
    fn from(val: ApiQuestion) -> Self {
        let len = val.bad_answers.len();
        let position = rand::rng().random_range(0..=len);
        let mut answers = val.bad_answers;
        answers.insert(position, val.answer);
        Question {
            question: val.question,
            answers,
            answer: position,
        }
    }
}

pub struct QuizCache {
    sessions: Mutex<HashMap<String, Arc<Vec<Question>>>>,
}

impl QuizCache {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, session: &str) -> Option<Arc<Vec<Question>>> {
        self.sessions.lock().unwrap().get(session).cloned()
    }

    pub fn insert(&self, session: String, questions: Vec<Question>) -> Arc<Vec<Question>> {
        let questions = Arc::new(questions);
        self.sessions
            .lock()
            .unwrap()
            .insert(session, Arc::clone(&questions));
        questions
    }
}
