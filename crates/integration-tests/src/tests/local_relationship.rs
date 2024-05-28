use crate::graphql_query;
use insta::assert_yaml_snapshot;
use serde_json::json;

#[tokio::test]
async fn joins_local_relationships() -> anyhow::Result<()> {
    assert_yaml_snapshot!(
        graphql_query(
            r#"
                query {
                  movies(limit: 2, order_by: {title: Asc}, where: {title: {_iregex: "Rear"}}) {
                    id
                    title
                    comments(limit: 2, order_by: {id: Asc}) {
                      email
                      text
                      movie {
                        id
                        title
                      }
                      user {
                        email
                        comments(limit: 2, order_by: {id: Asc}) {
                          email
                          text
                          user {
                            email
                            comments(limit: 2, order_by: {id: Asc}) {
                              id
                              email
                            }
                          }
                        }
                      }
                    }
                  }
                }
            "#
        )
        .variables(json!({ "limit": 11, "movies_limit": 2 }))
        .run()
        .await?
    );
    Ok(())
}

// TODO: Tests an upcoming change in MBD-14
#[ignore]
#[tokio::test]
async fn filters_by_field_of_related_collection() -> anyhow::Result<()> {
    assert_yaml_snapshot!(
        graphql_query(
            r#"
            query {
              comments(limit: 10, where: {movie: {title: {_is_null: false}}}) {
                movie {
                  title
                }
              }
            }
            "#
        )
        .variables(json!({ "limit": 11, "movies_limit": 2 }))
        .run()
        .await?
    );
    Ok(())
}
