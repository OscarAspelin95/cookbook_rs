use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::header,
};
use serde::{Deserialize, de::DeserializeOwned};
use validator::Validate;

use crate::errors::AppError;

#[derive(Debug, Validate, Deserialize)]
pub struct User {
    #[validate(length(min = 5, max = 15))]
    user_name: String,
    #[validate(email)]
    email: String,
}

/// This extractor is very similar to axums own `Json<T>` extractor,
/// but not as comprehensive and puts an addition trait bound of `Validate`.
pub struct ValidatedJson<T>(pub T)
where
    T: DeserializeOwned + Validate;

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match req.headers().get(header::CONTENT_TYPE).map(|c| c.to_str()) {
            Some(Ok("application/json")) => {}
            _ => {
                return Err(AppError::InvalidPayloadError(
                    "missing json body".to_string(),
                ));
            }
        };

        let bytes = Bytes::from_request(req, state).await?;
        let t: T = serde_json::from_slice(&bytes[..])?;
        t.validate()?;

        Ok(ValidatedJson(t))
    }
}
