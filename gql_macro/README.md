# Gql Proc Macro

This crate provides a procedural macro for creating async-graphql dynamic fields from Rust functions.

## Usage

The `#[mutation]` macro transforms an async Rust function into an async-graphql dynamic field. The function must:

1. Be `async`
2. Have its first parameter be an `async_graphql::dynamic::Context`
3. Have additional parameters that will become GraphQL arguments
4. Return a value that can be converted to a GraphQL type

## Example

```rust
use gql_macro::mutation;

#[mutation]
async fn login(ctx: async_graphql::dynamic::Context, username: String, password: String) -> String {
    // Your login logic here
    format!("Login attempt for user: {}", username)
}

#[mutation]
async fn create_user(ctx: async_graphql::dynamic::Context, name: String, email: String, age: i32) -> String {
    // Your user creation logic here
    format!("Created user: {} with email: {} and age: {}", name, email, age)
}
```

This generates a function that returns an `async_graphql::dynamic::Field`:

```rust
pub fn login() -> async_graphql::dynamic::Field {
    async_graphql::dynamic::Field::new(
        "login",
        async_graphql::dynamic::TypeRef::named_nn("String"),
        move |ctx| {
            use async_graphql::dynamic::{FieldFuture, FieldValue};
            
            let username = ctx.args.get("username");
            let password = ctx.args.get("password");
            
            FieldFuture::new(async move {
                let result = login(ctx).await;
                Ok(Some(FieldValue::owned_any(result)))
            })
        },
    )
    .argument(async_graphql::dynamic::InputValue::new(
        "username",
        async_graphql::dynamic::TypeRef::named_nn("String"),
    ))
    .argument(async_graphql::dynamic::InputValue::new(
        "password",
        async_graphql::dynamic::TypeRef::named_nn("String"),
    ))
}
```

## Supported Types

The macro automatically maps Rust types to GraphQL types:

- `String` → `String`
- `&str` → `String`
- `i32` → `Int`
- `f64` → `Float`
- `bool` → `Boolean`

Other types default to `String`.

## Requirements

- Function must be `async`
- First parameter must be `async_graphql::dynamic::Context`
- All parameters must be simple identifiers (no destructuring)
- Function cannot have `self` parameter 