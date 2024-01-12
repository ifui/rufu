use crate::entity::article_category_entity::ArticleCategory;
use crate::entity::article_data_entity::ArticleData;
use crate::entity::article_entity::Article;
use crate::request::article_request::{ArticleListRequest, ArticleRequest};
use crate::vo::article_vo::ArticleVo;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbatis::PageRequest;
use rufu_common::bootstrap::database::get_db;
use rufu_common::errors::AppError;
use rufu_common::request::paginate_request::PaginateRequest;
use rufu_common::response::PageData;
use serde_json::{from_value, json};

/// 公用 Request 映射 Entity 方法
fn request_map_entity(req: &ArticleRequest) -> (Article, ArticleData) {
    let article = Article {
        id: None,
        pre_title: req.clone().pre_title,
        title: req.clone().title,
        sub_title: req.clone().sub_title,
        sort: req.clone().sort.or(Some(0)),
        copyfrom: req.clone().copyfrom,
        author: req.clone().author,
        category_id: req.clone().category_id,
        status: req.clone().status,
        create_by: None,
        created_at: None,
        updated_at: Some(DateTime::now().format("YYYY-MM-DD hh:mm:ss")),
    };

    let article_data = ArticleData {
        article_id: None,
        content: req.clone().content,
    };

    (article, article_data)
}

/// 添加文章
pub async fn article_add_service(
    req: ArticleRequest,
    create_by: Option<String>,
) -> Result<(), AppError> {
    let db = get_db()?;

    let (mut article, mut article_data) = request_map_entity(&req);

    article.create_by = create_by;
    article.created_at = Some(DateTime::now().format("YYYY-MM-DD hh:mm:ss"));

    let article_inserted = Article::insert(db, &article).await?;

    // 设置附表文章ID
    article_data.article_id = Some(article_inserted.last_insert_id.u32());

    ArticleData::insert(db, &article_data).await?;

    Ok(())
}

/// 更新文章
pub async fn article_update_service(req: ArticleRequest, id: Option<u32>) -> Result<(), AppError> {
    let db = get_db()?;

    let (mut article, mut article_data) = request_map_entity(&req);

    article.id = id;
    article_data.article_id = id;

    Article::update_by_column(db, &article, "id").await?;

    ArticleData::update_by_column(db, &article_data, "article_id").await?;

    Ok(())
}

/// 显示文章详情
pub async fn article_show_service(id: Option<u32>) -> Result<ArticleVo, AppError> {
    let db = get_db()?;

    let article = Article::select_by_column(db, "id", Some(id)).await?;
    let article = article
        .first()
        .ok_or(AppError::RBATIS_ERROR("查询不到该条文章".to_string()))?;

    let article_data = ArticleData::select_by_column(db, "article_id", Some(id)).await?;
    let article_data = article_data.first().ok_or(AppError::RBATIS_ERROR(
        "没有找到该文章的附属表内容".to_string(),
    ))?;

    let article_category =
        ArticleCategory::select_by_column(db, "id", Some(article.clone().id)).await?;
    let article_category = article_category
        .first()
        .ok_or(AppError::RBATIS_ERROR("没有找到该文章的分类".to_string()))?;

    let mut article_vo: ArticleVo = from_value(json!(article))?;

    // 设置附属表内容
    article_vo.content = article_data.clone().content;
    // 设置文章分类
    article_vo.category = Some(article_category.clone());

    Ok(article_vo)
}

/// 获取文章列表
pub async fn article_list_service(
    req: ArticleListRequest,
    paginate: PaginateRequest,
) -> Result<PageData<Vec<Article>>, AppError> {
    let db = get_db()?;

    let article_list = Article::select_article_list(
        db,
        &PageRequest::new(paginate.page, paginate.page_size),
        req.title.or(None),
        req.author.or(None),
        req.copyfrom.or(None),
        req.id.or(None),
    )
    .await?;

    let res = PageData {
        total: article_list.total,
        page: article_list.page_no,
        page_size: article_list.page_size,
        records: Some(article_list.records),
    };

    Ok(res)
}

/// 删除文章
pub async fn article_delete_service(id: u32) -> Result<(), AppError> {
    let db = get_db()?;

    Article::delete_by_column(db, "id", &id).await?;
    // 删除附表
    ArticleData::delete_by_column(db, "article_id", &id).await?;

    Ok(())
}
