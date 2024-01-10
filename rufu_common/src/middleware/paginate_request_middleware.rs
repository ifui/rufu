use crate::errors::AppError;
use crate::request::paginate_request::PaginateRequest;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn paginate_request_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    match request.uri().query() {
        None => {}
        Some(q) => {
            let query: PaginateRequest = serde_urlencoded::from_str(q)?;
            request.extensions_mut().insert(query.to_owned());
        }
    }

    Ok(next.run(request).await)
}
