use axum::extract::Path;
use axum::Extension;
use rufu_article::entity::article_entity::Article;
use rufu_article::request::article_request::{ArticleListRequest, ArticleRequest};
use rufu_article::service::article_service::{
    article_add_service, article_delete_service, article_list_service, article_show_service,
    article_update_service,
};
use rufu_article::vo::article_vo::ArticleVo;
use rufu_auth::entity::admin_users_entity::AdminUsers;
use rufu_common::json::RufuJson;
use rufu_common::query::RufuQuery;
use rufu_common::request::batch_delete_request::BatchDeleteRequest;
use rufu_common::request::paginate_request::PaginateRequest;
use rufu_common::response::{AppResponse, AppResult, PageData};

/// 添加文章
#[axum::debug_handler]
pub async fn article_add_controller(
    Extension(admin_user): Extension<AdminUsers>,
    req: RufuJson<ArticleRequest>,
) -> AppResult<()> {
    article_add_service(req.validate()?, Some(admin_user.username)).await?;
    Ok(AppResponse::ok())
}

/// 更新文章
#[axum::debug_handler]
pub async fn article_update_controller(
    id: Path<u32>,
    req: RufuJson<ArticleRequest>,
) -> AppResult<()> {
    article_update_service(req.validate()?, Some(id.0)).await?;
    Ok(AppResponse::ok())
}

/// 文章列表
#[axum::debug_handler]
pub async fn article_list_controller(
    Extension(paginate): Extension<PaginateRequest>,
    RufuQuery(req): RufuQuery<ArticleListRequest>,
) -> AppResult<PageData<Vec<Article>>> {
    let res = article_list_service(req, paginate).await?;
    Ok(AppResponse::result(res))
}

/// 文章详情
#[axum::debug_handler]
pub async fn article_show_controller(id: Path<u32>) -> AppResult<ArticleVo> {
    let res = article_show_service(Some(id.0)).await?;
    Ok(AppResponse::result(res))
}

/// 删除文章
#[axum::debug_handler]
pub async fn article_delete_controller(req: RufuJson<BatchDeleteRequest>) -> AppResult<()> {
    article_delete_service(req.validate()?).await?;
    Ok(AppResponse::ok())
}
