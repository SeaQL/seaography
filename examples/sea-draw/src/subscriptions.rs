use async_graphql::dynamic::{
    FieldValue, InputValue, ResolverContext, SubscriptionField, SubscriptionFieldFuture, TypeRef,
};
use seaography::{BuilderContext, CustomInputType};
use tokio_stream::Stream;
use uuid::Uuid;

pub fn subscriptions(context: &'static BuilderContext) -> Vec<SubscriptionField> {
    let mut fields: Vec<SubscriptionField> = Vec::new();

    fields.push(
        SubscriptionField::new("drawing", TypeRef::named_nn("String"), move |ctx| {
            SubscriptionFieldFuture::new(async move {
                let id = Uuid::parse_value(context, ctx.args.get("id")).map_err(|_| {
                    Into::<async_graphql::Error>::into("Missing or invalid argument: id")
                })?;
                Subscription::drawing(ctx, id).await
            })
        })
        .argument(InputValue::new("id", Uuid::gql_input_type_ref(context))),
    );

    fields
}

pub struct Subscription;

impl Subscription {
    async fn drawing<'a>(
        _ctx: ResolverContext<'_>,
        _id: Uuid,
    ) -> async_graphql::Result<impl Stream<Item = async_graphql::Result<FieldValue<'a>>>> {
        Ok(async_graphql::async_stream::stream! {
            yield Ok(FieldValue::owned_any("test".to_string()));

            // while let Some(msg) = rx.recv().await {
            //     tracing::info!(
            //         chat_id = id.to_string(),
            //         project_id = chat.project_id.to_string(),
            //         account_id = access.account_id().to_string(),
            //         message_id = msg.id.to_string(),
            //         "Sending message for chat"
            //     );
            //     yield Ok(FieldValue::owned_any(msg));
            // }

            // multiplexer.remove_chat(id, listener_id);
            // tracing::info!(
            //     chat_id = id.to_string(),
            //     project_id = chat.project_id.to_string(),
            //     account_id = access.account_id().to_string(),
            //     "Removed listener for chat"
            // );
        })
    }
}
