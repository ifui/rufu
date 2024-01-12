use crate::entity::article_category_entity::ArticleCategory;
use crate::request::article_category_request::ArticleCategoryRequest;
use crate::vo::article_category_vo::ArticleCategoryVo;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;

/// 添加文章类别
pub async fn article_category_add_service(
    req: ArticleCategoryRequest,
) -> Result<ArticleCategory, AppError> {
    let db = get_db()?;

    let article_category = ArticleCategory {
        id: None,
        name: req.name,
        sort: req.sort.or(Some(0)),
        pid: req.pid.or(Some(0)),
    };

    ArticleCategory::insert(db, &article_category).await?;

    Ok(article_category)
}

/// 获取文章类别列表
pub async fn article_category_list_service() -> Result<Vec<ArticleCategoryVo>, AppError> {
    let db = get_db()?;
    let article_category_vec = ArticleCategory::select_all(db).await?;

    let res = vec_to_tree(&article_category_vec, Some(0));

    Ok(res)
}

/// vec 转树形数据
fn vec_to_tree(vec: &Vec<ArticleCategory>, pid: Option<u32>) -> Vec<ArticleCategoryVo> {
    let mut arr: Vec<ArticleCategoryVo> = Vec::new();

    for item in vec {
        if item.pid.eq(&pid) {
            arr.push(ArticleCategoryVo {
                id: item.id.clone(),
                name: item.name.clone(),
                sort: item.sort.clone(),
                pid: item.pid.clone(),
                children: vec_to_tree(vec, item.id.clone()),
            })
        }
    }

    // 排序，根据sort排序
    arr.sort_by(|a, b| a.sort.cmp(&b.sort));

    arr
}

/// 更新文章类别
pub async fn article_category_update_service(
    req: ArticleCategoryRequest,
    id: u32,
) -> Result<ArticleCategory, AppError> {
    let db = get_db()?;

    let article_category = ArticleCategory {
        id: Some(id),
        name: req.name,
        sort: req.sort.or(Some(0)),
        pid: req.pid.or(Some(0)),
    };

    ArticleCategory::update_by_column(db, &article_category, "id").await?;

    Ok(article_category)
}

/// 删除文章类别
pub async fn article_category_delete_service(id: u32) -> Result<(), AppError> {
    let db = get_db()?;

    ArticleCategory::delete_by_column(db, "id", id).await?;

    Ok(())
}
