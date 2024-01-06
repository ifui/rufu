use crate::errors::AppError;
use crate::response::AppResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn map_response_middleware(res: Response) -> Response {
    let service_error = res.extensions().get::<AppError>();
    let client_status_error = service_error.map(|se| se.get_error());

    let error_response = client_status_error.as_ref().map(|(code, msg, val)| {
        (
            StatusCode::OK,
            AppResponse::new(*code, msg.clone(), val.clone()),
        )
            .into_response()
    });

    // TODO: 日志实现

    error_response.unwrap_or(res)
}
