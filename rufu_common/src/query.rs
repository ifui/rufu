use crate::errors::AppError;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Uri;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy, Default)]
pub struct RufuQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for RufuQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Self::try_from_uri(&parts.uri)
    }
}

impl<T> RufuQuery<T>
where
    T: DeserializeOwned,
{
    pub fn try_from_uri(value: &Uri) -> Result<Self, AppError> {
        let query = value.query().unwrap_or_default();
        let params = serde_urlencoded::from_str(query)?;
        Ok(RufuQuery(params))
    }
}
