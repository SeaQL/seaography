use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptyMutation, EmptySubscription, Response, Schema};
use generated::{OrmDataloader, QueryRoot};
use sea_orm::Database;

pub async fn get_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
  let database = Database::connect("postgres://sea:sea@127.0.0.1/dvdrental")
      .await
      .unwrap();
  let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
      OrmDataloader {
          db: database.clone(),
      },
      tokio::spawn,
  );
  let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
      .data(database)
      .data(orm_dataloader)
      .finish();

  schema
}

pub fn assert_eq(a: Response, b: &str) {
  assert_eq!(
      a.data.into_json().unwrap(),
      serde_json::from_str::<serde_json::Value>(b).unwrap()
  )
}

#[tokio::test]
async fn test_simple_query() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
              r#"
                {
                  rental (pagination:{ limit: 4, page: 0}) {
                    data {
                      rentalId
                      rentalDate
                      customerCustomer {
                        firstName
                        lastName
                      }
                    }
                    pages
                    current
                  }
                }
          "#)
            .await,
        r#"
        {
          "rental": {
            "data": [
              {
                "rentalId": 2,
                "rentalDate": "2005-05-24T22:54:33",
                "customerCustomer": {
                  "firstName": "Tommy",
                  "lastName": "Collazo"
                }
              },
              {
                "rentalId": 3,
                "rentalDate": "2005-05-24T23:03:39",
                "customerCustomer": {
                  "firstName": "Manuel",
                  "lastName": "Murrell"
                }
              },
              {
                "rentalId": 4,
                "rentalDate": "2005-05-24T23:04:41",
                "customerCustomer": {
                  "firstName": "Andrew",
                  "lastName": "Purdy"
                }
              },
              {
                "rentalId": 5,
                "rentalDate": "2005-05-24T23:05:21",
                "customerCustomer": {
                  "firstName": "Delores",
                  "lastName": "Hansen"
                }
              }
            ],
            "pages": 4011,
            "current": 0
          }
        }
        "#,
    )
}