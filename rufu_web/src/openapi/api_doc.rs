use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::app::controller::admin::admin_article_controller::article_add_controller,
        crate::app::controller::admin::admin_article_controller::article_list_controller,
        crate::app::controller::admin::admin_article_controller::article_delete_controller,
        crate::app::controller::admin::admin_article_controller::article_update_controller,
        crate::app::controller::admin::admin_article_controller::article_show_controller,

        crate::app::controller::admin::admin_article_category_controller::article_category_add_controller,
        crate::app::controller::admin::admin_article_category_controller::article_category_list_controller,
        crate::app::controller::admin::admin_article_category_controller::article_category_delete_controller,
        crate::app::controller::admin::admin_article_category_controller::article_category_update_controller,

        crate::app::controller::admin::admin_auth_controller::admin_login_controller,
        crate::app::controller::admin::admin_auth_controller::admin_register_controller,
        crate::app::controller::admin::admin_auth_controller::admin_userinfo_controller,
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "rufu_auth", description = "用户中心"),
        (name = "rufu_article", description = "文章模块")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("rufu_apikey"))),
            )
        }
    }
}
