use crate::response::AppResponse;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::{async_trait, Json, RequestPartsExt};
use serde::Serialize;
use serde_json::{json, Value};
use validator::{Validate, ValidationErrors};

#[derive(Serialize)]
pub struct RufuJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for RufuJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        // Have to run that first since `Json` extraction consumes the request.
        // let path = parts
        //     .extract::<MatchedPath>()
        //     .await
        //     .map(|path| path.as_str().to_owned())
        //     .ok();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                let payload = AppResponse::<usize>::error(400, rejection.body_text());

                Err((StatusCode::OK, Json(json!(*payload))))
            }
        }
    }
}

impl<T> RufuJson<T>
where
    T: Validate,
{
    /// 使用 Validate 验证字段
    pub fn validate(self) -> Result<T, ValidationErrors> {
        match self.0.validate() {
            Ok(_) => Ok(self.0),
            Err(e) => Err(e),
        }
    }
}
