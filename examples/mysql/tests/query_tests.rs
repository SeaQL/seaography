use async_graphql::{dynamic::*, Response};
use sea_orm::Database;

pub async fn get_schema() -> Schema {
    let database = Database::connect("mysql://sea:sea@127.0.0.1/sakila")
        .await
        .unwrap();
    let schema = seaography_mysql_example::query_root::schema(database, None, None).unwrap();

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
                    store {
                    nodes {
                        storeId
                        staff {
                        firstName
                        lastName
                        }
                    }
                    }
                }
                "#,
            )
            .await,
        r#"
            {
                "store": {
                "nodes": [
                    {
                    "storeId": 1,
                    "staff": {
                        "firstName": "Mike",
                        "lastName": "Hillyer"
                    }
                    },
                    {
                    "storeId": 2,
                    "staff": {
                        "firstName": "Jon",
                        "lastName": "Stephens"
                    }
                    }
                ]
                }
            }
            "#,
    )
}

#[tokio::test]
async fn test_simple_query_with_filter() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    store(filters: {storeId:{eq: 1}}) {
                        nodes {
                        storeId
                        staff {
                            firstName
                            lastName
                        }
                        }
                    }
                }
                "#,
            )
            .await,
        r#"
            {
                "store": {
                "nodes": [
                    {
                    "storeId": 1,
                    "staff": {
                        "firstName": "Mike",
                        "lastName": "Hillyer"
                    }
                    }
                ]
                }
            }
            "#,
    )
}

#[tokio::test]
async fn test_filter_with_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    customer(
                      filters: { active: { eq: 0 } }
                      pagination: { page: { page: 2, limit: 3 } }
                    ) {
                      nodes {
                        customerId
                      }
                      paginationInfo {
                        pages
                        current
                      }
                    }
                  }
                "#,
            )
            .await,
        r#"
            {
                "customer": {
                  "nodes": [
                    {
                      "customerId": 315
                    },
                    {
                      "customerId": 368
                    },
                    {
                      "customerId": 406
                    }
                  ],
                  "paginationInfo": {
                    "pages": 5,
                    "current": 2
                  }
                }
            }
            "#,
    )
}

#[tokio::test]
async fn test_complex_filter_with_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    payment(
                      filters: { amount: { gt: "11.1" } }
                      pagination: { page: { limit: 2, page: 3 } }
                    ) {
                      nodes {
                        paymentId
                        amount
                      }
                      paginationInfo {
                        pages
                        current
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
            {
                "payment": {
                "nodes": [
                    {
                    "paymentId": 8272,
                    "amount": "11.99"
                    },
                    {
                    "paymentId": 9803,
                    "amount": "11.99"
                    }
                ],
                "paginationInfo": {
                    "pages": 5,
                    "current": 3
                }
                }
            }
            "#,
    )
}

#[tokio::test]
async fn test_cursor_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    payment(
                        filters: { amount: { gt: "11" } }
                        pagination: { cursor: { limit: 5 } }
                    ) {
                        edges {
                        node {
                            paymentId
                            amount
                            customer {
                            firstName
                            }
                        }
                        }
                        pageInfo {
                        hasPreviousPage
                        hasNextPage
                        startCursor
                        endCursor
                        }
                    }
                }
                "#,
            )
            .await,
        r#"
            {
            "payment": {
                "edges": [
                {
                    "node": {
                    "paymentId": 342,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "KAREN"
                    }
                    }
                },
                {
                    "node": {
                    "paymentId": 3146,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "VICTORIA"
                    }
                    }
                },
                {
                    "node": {
                    "paymentId": 5280,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "VANESSA"
                    }
                    }
                },
                {
                    "node": {
                    "paymentId": 5281,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "ALMA"
                    }
                    }
                },
                {
                    "node": {
                    "paymentId": 5550,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "ROSEMARY"
                    }
                    }
                }
                ],
                "pageInfo": {
                "hasPreviousPage": false,
                "hasNextPage": true,
                "startCursor": "Int[3]:342",
                "endCursor": "Int[4]:5550"
                }
            }
            }
            "#,
    )
}

#[tokio::test]
async fn test_cursor_pagination_prev() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  payment(
                    filters: { amount: { gt: "11" } }
                    pagination: { cursor: { limit: 3, cursor: "SmallUnsigned[4]:5550" } }
                  ) {
                    edges {
                      node {
                        paymentId
                        amount
                        customer {
                          firstName
                        }
                      }
                    }
                    pageInfo {
                      hasPreviousPage
                      hasNextPage
                      startCursor
                      endCursor
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
            {
            "payment": {
                "edges": [
                {
                    "node": {
                    "paymentId": 6409,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "TANYA"
                    }
                    }
                },
                {
                    "node": {
                    "paymentId": 8272,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "RICHARD"
                    }
                    }
                },
                {
                    "node": {
                    "paymentId": 9803,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "NICHOLAS"
                    }
                    }
                }
                ],
                "pageInfo": {
                "hasPreviousPage": true,
                "hasNextPage": true,
                "startCursor": "Int[4]:6409",
                "endCursor": "Int[4]:9803"
                }
            }
            }
            "#,
    )
}

#[tokio::test]
async fn test_cursor_pagination_no_next() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  payment(
                    filters: { amount: { gt: "11" } }
                    pagination: { cursor: { limit: 3, cursor: "SmallUnsigned[4]:9803" } }
                  ) {
                    edges {
                      node {
                        paymentId
                        amount
                        customer {
                          firstName
                        }
                      }
                    }
                    pageInfo {
                      hasPreviousPage
                      hasNextPage
                      startCursor
                      endCursor
                    }
                  }
                }
                "#,
            )
            .await,
        r#"
            {
            "payment": {
                "edges": [
                {
                    "node": {
                    "paymentId": 15821,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "KENT"
                    }
                    }
                },
                {
                    "node": {
                    "paymentId": 15850,
                    "amount": "11.99",
                    "customer": {
                        "firstName": "TERRANCE"
                    }
                    }
                }
                ],
                "pageInfo": {
                "hasPreviousPage": true,
                "hasNextPage": false,
                "startCursor": "Int[5]:15821",
                "endCursor": "Int[5]:15850"
                }
            }
            }
            "#,
    )
}

#[tokio::test]
async fn test_self_ref() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    staff {
                      nodes {
                        firstName
                        reportsToId
                        selfRefReverse {
                          nodes {
                            staffId
                            firstName
                          }
                        }
                        selfRef {
                          staffId
                          firstName
                        }
                      }
                    }
                  }
                "#,
            )
            .await,
        r#"
            {
                "staff": {
                "nodes": [
                    {
                    "firstName": "Mike",
                    "reportsToId": null,
                    "selfRefReverse": {
                        "nodes": [
                        {
                            "staffId": 2,
                            "firstName": "Jon"
                        }
                        ]
                    },
                    "selfRef": null
                    },
                    {
                    "firstName": "Jon",
                    "reportsToId": 1,
                    "selfRefReverse": {
                        "nodes": []
                    },
                    "selfRef": {
                        "staffId": 1,
                        "firstName": "Mike"
                    }
                    }
                ]
                }
            }
            "#,
    )
}

#[tokio::test]
async fn related_queries_filters() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                    customer(
                      filters: { active: { eq: 0 } }
                      pagination: { cursor: { limit: 3, cursor: "Int[3]:271" } }
                    ) {
                      nodes {
                        customerId
                        lastName
                        email
                        address {
                          address
                        }
                        payment(filters: { amount: { gt: "8" } }, orderBy: { amount: DESC }) {
                          nodes {
                            paymentId
                          }
                        }
                      }
                      pageInfo {
                        hasPreviousPage
                        hasNextPage
                        endCursor
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
        {
          "customer": {
            "nodes": [
              {
                "customerId": 315,
                "lastName": "GOODEN",
                "email": "KENNETH.GOODEN@sakilacustomer.org",
                "address": {
                  "address": "1542 Lubumbashi Boulevard"
                },
                "payment": {
                  "nodes": [
                    {
                      "paymentId": 8547
                    },
                    {
                      "paymentId": 8537
                    }
                  ]
                }
              },
              {
                "customerId": 368,
                "lastName": "ARCE",
                "email": "HARRY.ARCE@sakilacustomer.org",
                "address": {
                  "address": "1922 Miraj Way"
                },
                "payment": {
                  "nodes": [
                    {
                      "paymentId": 9945
                    },
                    {
                      "paymentId": 9953
                    },
                    {
                      "paymentId": 9962
                    },
                    {
                      "paymentId": 9967
                    }
                  ]
                }
              },
              {
                "customerId": 406,
                "lastName": "RUNYON",
                "email": "NATHAN.RUNYON@sakilacustomer.org",
                "address": {
                  "address": "264 Bhimavaram Manor"
                },
                "payment": {
                  "nodes": [
                    {
                      "paymentId": 10998
                    }
                  ]
                }
              }
            ],
            "pageInfo": {
              "hasPreviousPage": true,
              "hasNextPage": true,
              "endCursor": "Int[3]:406"
            }
          }
        }
        "#,
    );

    assert_eq(
        schema
            .execute(
                r#"
                {
                    film(filters:{filmId: {eq: 1}}) {
                      nodes {
                        title
                        description
                        releaseYear
                        actor {
                          nodes {
                            firstName
                            lastName
                          }
                        }
                      }
                    }
                }
                "#,
            )
            .await,
        r#"
        {
            "film": {
              "nodes": [
                {
                  "title": "ACADEMY DINOSAUR",
                  "description": "A Epic Drama of a Feminist And a Mad Scientist who must Battle a Teacher in The Canadian Rockies",
                  "releaseYear": 2006,
                  "actor": {
                    "nodes": [
                      {
                        "firstName": "PENELOPE",
                        "lastName": "GUINESS"
                      },
                      {
                        "firstName": "CHRISTIAN",
                        "lastName": "GABLE"
                      },
                      {
                        "firstName": "LUCILLE",
                        "lastName": "TRACY"
                      },
                      {
                        "firstName": "SANDRA",
                        "lastName": "PECK"
                      },
                      {
                        "firstName": "JOHNNY",
                        "lastName": "CAGE"
                      },
                      {
                        "firstName": "MENA",
                        "lastName": "TEMPLE"
                      },
                      {
                        "firstName": "WARREN",
                        "lastName": "NOLTE"
                      },
                      {
                        "firstName": "OPRAH",
                        "lastName": "KILMER"
                      },
                      {
                        "firstName": "ROCK",
                        "lastName": "DUKAKIS"
                      },
                      {
                        "firstName": "MARY",
                        "lastName": "KEITEL"
                      }
                    ]
                  }
                }
              ]
            }
        }
        "#,
    )
}

#[tokio::test]
async fn related_queries_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  customer(
                    filters: { active: { eq: 0 } }
                    pagination: { cursor: { limit: 3, cursor: "Int[3]:271" } }
                  ) {
                    nodes {
                      customerId
                      lastName
                      email
                      address {
                        address
                      }
                      payment(
                        filters: { amount: { gt: "7" } }
                        orderBy: { amount: ASC }
                        pagination: { page: { limit: 1, page: 1 } }
                      ) {
                        nodes {
                          paymentId
                          amount
                        }
                        paginationInfo {
                          pages
                          current
                        }
                        pageInfo {
                          hasPreviousPage
                          hasNextPage
                        }
                      }
                    }
                    pageInfo {
                      hasPreviousPage
                      hasNextPage
                      endCursor
                    }
                  }
                }
        "#,
            )
            .await,
        r#"
        {
          "customer": {
            "nodes": [
              {
                "customerId": 315,
                "lastName": "GOODEN",
                "email": "KENNETH.GOODEN@sakilacustomer.org",
                "address": {
                  "address": "1542 Lubumbashi Boulevard"
                },
                "payment": {
                  "nodes": [
                    {
                      "paymentId": 8547,
                      "amount": "9.99"
                    }
                  ],
                  "paginationInfo": {
                    "pages": 2,
                    "current": 1
                  },
                  "pageInfo": {
                    "hasPreviousPage": true,
                    "hasNextPage": false
                  }
                }
              },
              {
                "customerId": 368,
                "lastName": "ARCE",
                "email": "HARRY.ARCE@sakilacustomer.org",
                "address": {
                  "address": "1922 Miraj Way"
                },
                "payment": {
                  "nodes": [
                    {
                      "paymentId": 9972,
                      "amount": "7.99"
                    }
                  ],
                  "paginationInfo": {
                    "pages": 6,
                    "current": 1
                  },
                  "pageInfo": {
                    "hasPreviousPage": true,
                    "hasNextPage": true
                  }
                }
              },
              {
                "customerId": 406,
                "lastName": "RUNYON",
                "email": "NATHAN.RUNYON@sakilacustomer.org",
                "address": {
                  "address": "264 Bhimavaram Manor"
                },
                "payment": {
                  "nodes": [
                    {
                      "paymentId": 10989,
                      "amount": "7.99"
                    }
                  ],
                  "paginationInfo": {
                    "pages": 3,
                    "current": 1
                  },
                  "pageInfo": {
                    "hasPreviousPage": true,
                    "hasNextPage": true
                  }
                }
              }
            ],
            "pageInfo": {
              "hasPreviousPage": true,
              "hasNextPage": true,
              "endCursor": "Int[3]:406"
            }
          }
        }
        "#,
    )
}

#[tokio::test]
async fn enumeration_filter() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
                {
                  film(
                    filters: { rating: { eq: NC17 } }
                    pagination: { page: { page: 1, limit: 5 } }
                  ) {
                    nodes {
                      filmId
                      rating
                    }
                  }
                }
        "#,
            )
            .await,
        r#"
        {
          "film": {
            "nodes": [
              {
                "filmId": 27,
                "rating": "NC17"
              },
              {
                "filmId": 29,
                "rating": "NC17"
              },
              {
                "filmId": 31,
                "rating": "NC17"
              },
              {
                "filmId": 34,
                "rating": "NC17"
              },
              {
                "filmId": 38,
                "rating": "NC17"
              }
            ]
          }
        }
        "#,
    )
}
