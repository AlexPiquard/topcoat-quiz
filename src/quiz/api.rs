use std::sync::Arc;

use topcoat::{
    Result,
    context::{Cx, try_app_context},
};

use crate::quiz::model::{Question, QuizApiResponse, QuizCache};

pub async fn fetch_questions(
    cx: &Cx,
    difficulty: &str,
    session: &str,
) -> Result<Arc<Vec<Question>>> {
    let cache: &QuizCache =
        try_app_context(cx).ok_or_else(|| api_error("quiz cache not registered"))?;
    if let Some(questions) = cache.get(session) {
        return Ok(questions);
    }

    let api_url = format!("https://quizzapi.fr/api/v2/quiz?limit=5&difficulty={difficulty}");

    let response = reqwest::get(api_url)
        .await?
        .json::<QuizApiResponse>()
        .await?;
    Ok(cache.insert(
        session.to_owned(),
        response.quizzes.into_iter().map(Into::into).collect(),
    ))
}

fn api_error(error: impl std::fmt::Display) -> topcoat::router::error::BadRequestError {
    tracing::error!("api error: {error}");
    topcoat::router::error::bad_request(error.to_string())
}
