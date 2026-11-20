use seaography::async_graphql::{
    self,
    dynamic::{Object, Schema},
    value,
};
use seaography::{
    BuilderContext, CustomFields, CustomInputObject, CustomInputType, CustomOutputObject,
    CustomOutputType,
};
use std::sync::LazyLock;

#[derive(Clone, CustomInputType, CustomOutputType)]
#[seaography(rename_all = "camelCase")]
struct Post {
    #[seaography(rename = "byline")]
    author_name: String,
    is_public: bool,
}

struct Queries;

#[CustomFields(rename_all = "camelCase")]
impl Queries {
    async fn echo_post(post: Post) -> async_graphql::Result<Post> {
        Ok(post)
    }

    #[seaography(rename = "numberOfPosts")]
    async fn post_count() -> async_graphql::Result<i32> {
        Ok(1)
    }
}

#[tokio::test]
async fn rename_controls_graphql_field_names() {
    static CONTEXT: LazyLock<BuilderContext> = LazyLock::new(BuilderContext::default);

    let mut query = Object::new("Query");
    for field in Queries::to_fields(&CONTEXT) {
        query = query.field(field);
    }

    let schema = Schema::build("Query", None, None)
        .register(query)
        .register(Post::input_object(&CONTEXT))
        .register(Post::basic_object(&CONTEXT))
        .finish()
        .unwrap();

    let response = schema
        .execute(
            r#"
            {
              echoPost(post: { byline: "Bob", isPublic: true }) {
                byline
                isPublic
              }
              numberOfPosts
            }
            "#,
        )
        .await
        .into_result()
        .unwrap();
    assert_eq!(
        response.data,
        value!({ "echoPost": { "byline": "Bob", "isPublic": true }, "numberOfPosts": 1 })
    );
}
