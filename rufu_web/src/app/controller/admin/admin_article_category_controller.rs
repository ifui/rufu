use axum::extract::Path;
use rufu_article::entity::article_category_entity::ArticleCategory;
use rufu_article::request::article_category_request::ArticleCategoryRequest;
use rufu_article::service::article_category_service::{
    article_category_add_service, article_category_delete_service, article_category_list_service,
    article_category_update_service,
};
use rufu_article::vo::article_category_vo::ArticleCategoryVo;
use rufu_common::json::RufuJson;
use rufu_common::response::{AppResponse, AppResult};

/// 添加文章类别
#[axum::debug_handler]
#[utoipa::path(post, path = "/admin/article/category", tag = "rufu_article")]
pub async fn article_category_add_controller(
    req: RufuJson<ArticleCategoryRequest>,
) -> AppResult<ArticleCategory> {
    let res = article_category_add_service(req.validate()?).await?;
    Ok(AppResponse::result(res))
}

/// 浏览文章类别列表
#[axum::debug_handler]
#[utoipa::path(get, path = "/admin/article/category", tag = "rufu_article")]
pub async fn article_category_list_controller() -> AppResult<Vec<ArticleCategoryVo>> {
    let res = article_category_list_service().await?;
    Ok(AppResponse::result(res))
}

/// 更新文章类别
#[axum::debug_handler]
#[utoipa::path(put, path = "/admin/article/category", tag = "rufu_article")]
pub async fn article_category_update_controller(
    id: Path<u32>,
    req: RufuJson<ArticleCategoryRequest>,
) -> AppResult<ArticleCategory> {
    let res = article_category_update_service(req.validate()?, id.0).await?;
    Ok(AppResponse::result(res))
}

/// 删除文章类别
#[axum::debug_handler]
#[utoipa::path(delete, path = "/admin/article/category/:id", tag = "rufu_article")]
pub async fn article_category_delete_controller(id: Path<u32>) -> AppResult<ArticleCategory> {
    article_category_delete_service(id.0).await?;
    Ok(AppResponse::ok())
}
