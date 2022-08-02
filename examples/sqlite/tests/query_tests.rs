use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptyMutation, EmptySubscription, Response, Schema};
use generated::{OrmDataloader, QueryRoot};
use sea_orm::Database;

pub async fn get_schema() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
    let database = Database::connect("sqlite://chinook.db").await.unwrap();
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
                employees {
                    data {
                    employeeId
                    firstName
                    lastName
                    }
                }
            }
        "#,
            )
            .await,
        r#"
        {
            "employees": {
            "data": [
              {
                "employeeId": 1,
                "firstName": "Andrew",
                "lastName": "Adams"
              },
              {
                "employeeId": 2,
                "firstName": "Nancy",
                "lastName": "Edwards"
              },
              {
                "employeeId": 3,
                "firstName": "Jane",
                "lastName": "Peacock"
              },
              {
                "employeeId": 4,
                "firstName": "Margaret",
                "lastName": "Park"
              },
              {
                "employeeId": 5,
                "firstName": "Steve",
                "lastName": "Johnson"
              },
              {
                "employeeId": 6,
                "firstName": "Michael",
                "lastName": "Mitchell"
              },
              {
                "employeeId": 7,
                "firstName": "Robert",
                "lastName": "King"
              },
              {
                "employeeId": 8,
                "firstName": "Laura",
                "lastName": "Callahan"
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
              playlists(filters: { or: [{playlistId:{eq: 16}}, {playlistId:{eq: 2}}]}) {
                data {
                  playlistId
                  playlistPlaylistTrack {
                    playlistId
                    trackId
                    trackTracks {
                      trackId
                      name
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
            "playlists": {
              "data": [
                {
                  "playlistId": 2,
                  "playlistPlaylistTrack": []
                },
                {
                  "playlistId": 16,
                  "playlistPlaylistTrack": [
                    {
                      "playlistId": 16,
                      "trackId": 52,
                      "trackTracks": {
                        "trackId": 52,
                        "name": "Man In The Box"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2003,
                      "trackTracks": {
                        "trackId": 2003,
                        "name": "Smells Like Teen Spirit"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2004,
                      "trackTracks": {
                        "trackId": 2004,
                        "name": "In Bloom"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2005,
                      "trackTracks": {
                        "trackId": 2005,
                        "name": "Come As You Are"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2007,
                      "trackTracks": {
                        "trackId": 2007,
                        "name": "Lithium"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2010,
                      "trackTracks": {
                        "trackId": 2010,
                        "name": "Drain You"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2013,
                      "trackTracks": {
                        "trackId": 2013,
                        "name": "On A Plain"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2194,
                      "trackTracks": {
                        "trackId": 2194,
                        "name": "Evenflow"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2195,
                      "trackTracks": {
                        "trackId": 2195,
                        "name": "Alive"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2198,
                      "trackTracks": {
                        "trackId": 2198,
                        "name": "Jeremy"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2206,
                      "trackTracks": {
                        "trackId": 2206,
                        "name": "Daughter"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2512,
                      "trackTracks": {
                        "trackId": 2512,
                        "name": "Outshined"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2516,
                      "trackTracks": {
                        "trackId": 2516,
                        "name": "Black Hole Sun"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 2550,
                      "trackTracks": {
                        "trackId": 2550,
                        "name": "Plush"
                      }
                    },
                    {
                      "playlistId": 16,
                      "trackId": 3367,
                      "trackTracks": {
                        "trackId": 3367,
                        "name": "Hunger Strike"
                      }
                    }
                  ]
                }
              ]
            }
          }
        "#,
    )
}

#[tokio::test]
async fn test_simple_pagination() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
            {
              employees (pagination:{limit: 2, page: 0}) {
                data {
                  employeeId
                  lastName
                  title
                },
                pages
                current
              }
            }
        "#,
            )
            .await,
        r#"
          {
            "employees": {
              "data": [
                {
                  "employeeId": 1,
                  "lastName": "Adams",
                  "title": "General Manager"
                },
                {
                  "employeeId": 2,
                  "lastName": "Edwards",
                  "title": "Sales Manager"
                }
              ],
              "pages": 4,
              "current": 0
            }
          }
        "#,
    )
}

#[tokio::test]
async fn test_complex_string_filtering() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
            {
              artists( filters: { name:{ eq:"AC/DC", }}) {
                data {
                  name
                  artistAlbums {
                    title
                    albumTracks {
                      name
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
            "artists": {
              "data": [
                {
                  "name": "AC/DC",
                  "artistAlbums": [
                    {
                      "title": "For Those About To Rock We Salute You",
                      "albumTracks": [
                        {
                          "name": "For Those About To Rock (We Salute You)"
                        },
                        {
                          "name": "Put The Finger On You"
                        },
                        {
                          "name": "Let's Get It Up"
                        },
                        {
                          "name": "Inject The Venom"
                        },
                        {
                          "name": "Snowballed"
                        },
                        {
                          "name": "Evil Walks"
                        },
                        {
                          "name": "C.O.D."
                        },
                        {
                          "name": "Breaking The Rules"
                        },
                        {
                          "name": "Night Of The Long Knives"
                        },
                        {
                          "name": "Spellbound"
                        }
                      ]
                    },
                    {
                      "title": "Let There Be Rock",
                      "albumTracks": [
                        {
                          "name": "Go Down"
                        },
                        {
                          "name": "Dog Eat Dog"
                        },
                        {
                          "name": "Let There Be Rock"
                        },
                        {
                          "name": "Bad Boy Boogie"
                        },
                        {
                          "name": "Problem Child"
                        },
                        {
                          "name": "Overdose"
                        },
                        {
                          "name": "Hell Ain't A Bad Place To Be"
                        },
                        {
                          "name": "Whole Lotta Rosie"
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          }
        "#,
    )
}

#[tokio::test]
async fn test_number_filtering() {
    let schema = get_schema().await;

    assert_eq(
        schema
            .execute(
                r#"
            {
              tracks ( filters:{ milliseconds:{gt: 3000000 }}) {
                data {
                  milliseconds
                  name
                  albumAlbums {
                    artistArtists {
                      name
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
          "tracks": {
            "data": [
              {
                "milliseconds": 5286953,
                "name": "Occupation / Precipice",
                "albumAlbums": {
                  "artistArtists": {
                    "name": "Battlestar Galactica"
                  }
                }
              },
              {
                "milliseconds": 5088838,
                "name": "Through a Looking Glass",
                "albumAlbums": {
                  "artistArtists": {
                    "name": "Lost"
                  }
                }
              }
            ]
          }
        }
        "#,
    )
}
