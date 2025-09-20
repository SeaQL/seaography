use async_graphql::{dynamic::*, Response};
use sea_orm::Database;
use seaography::async_graphql;

pub async fn schema() -> Schema {
    let database = Database::connect("postgres://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
    let schema = seaography_postgres_example::query_root::schema(database, None, None).unwrap();

    schema
}

pub fn assert_eq(a: Response, b: &str) {
    assert_eq!(
        a.data.into_json().unwrap(),
        serde_json::from_str::<serde_json::Value>(b).unwrap()
    )
}

#[tokio::test]
async fn test_entity_metadata() {
    let schema = schema().await;

    assert_eq(
        schema
            .execute(
                r#"{
                  city: _sea_orm_entity_metadata(table_name: "city") {
                    columns {
                      name
                      nullable
                      type_ {
                        primitive
                        array {
                          array {
                            primitive
                          }
                        }
                        enumeration {
                          name
                          variants
                        }
                      }
                    }
                    primary_key
                  }
                  film: _sea_orm_entity_metadata(table_name: "film") {
                    columns {
                      name
                      nullable
                      type_ {
                        primitive
                        array {
                          array {
                            primitive
                          }
                        }
                        enumeration {
                          name
                          variants
                        }
                      }
                    }
                    primary_key
                  }
                }
          "#,
            )
            .await,
        r#"
        {
          "city": {
            "columns": [
              {
                "name": "city_id",
                "nullable": false,
                "type_": {
                  "primitive": "integer",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "city",
                "nullable": false,
                "type_": {
                  "primitive": "string",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "country_id",
                "nullable": false,
                "type_": {
                  "primitive": "integer",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "last_update",
                "nullable": false,
                "type_": {
                  "primitive": "datetime",
                  "array": null,
                  "enumeration": null
                }
              }
            ],
            "primary_key": [
              "city_id"
            ]
          },
          "film": {
            "columns": [
              {
                "name": "film_id",
                "nullable": false,
                "type_": {
                  "primitive": "integer",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "title",
                "nullable": false,
                "type_": {
                  "primitive": "string",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "description",
                "nullable": true,
                "type_": {
                  "primitive": "string",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "release_year",
                "nullable": true,
                "type_": {
                  "primitive": "integer",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "language_id",
                "nullable": false,
                "type_": {
                  "primitive": "integer",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "original_language_id",
                "nullable": true,
                "type_": {
                  "primitive": "integer",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "rental_duration",
                "nullable": false,
                "type_": {
                  "primitive": "integer",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "rental_rate",
                "nullable": false,
                "type_": {
                  "primitive": "decimal",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "length",
                "nullable": true,
                "type_": {
                  "primitive": "integer",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "replacement_cost",
                "nullable": false,
                "type_": {
                  "primitive": "decimal",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "rating",
                "nullable": true,
                "type_": {
                  "primitive": null,
                  "array": null,
                  "enumeration": {
                    "name": "mpaa_rating",
                    "variants": [
                      "G",
                      "NC-17",
                      "PG",
                      "PG-13",
                      "R"
                    ]
                  }
                }
              },
              {
                "name": "last_update",
                "nullable": false,
                "type_": {
                  "primitive": "datetime",
                  "array": null,
                  "enumeration": null
                }
              },
              {
                "name": "special_features",
                "nullable": true,
                "type_": {
                  "primitive": null,
                  "array": {
                    "array": {
                      "primitive": "string"
                    }
                  },
                  "enumeration": null
                }
              },
              {
                "name": "metadata",
                "nullable": true,
                "type_": {
                  "primitive": "json",
                  "array": null,
                  "enumeration": null
                }
              }
            ],
            "primary_key": [
              "film_id"
            ]
          }
        }
          "#,
    )
}
