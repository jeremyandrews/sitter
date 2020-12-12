/// A Person is the primary object defined and managed by Sitter. It could
/// also be referred to as the User.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use sqlx::postgres::PgPool;

#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub pass: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PersonFilter {
    pub ids: Option<Vec<Uuid>>,
    pub names: Option<Vec<String>>,
    pub emails: Option<Vec<String>>,
}

/// A helper function to add a new Person object to the database.
pub async fn create(
    db: &PgPool,
    name: &str,
    email: &str,
    pass: &str
) -> anyhow::Result<String> {
    // @TODO: validate (and/or allow validation of) name, email and pass.

    // @TODO: error handling.
    let insert = sqlx::query!(
        r#"
INSERT INTO person (name, email, pass)
VALUES ($1, $2, $3)
RETURNING id
        "#,
        name,
        email,
        pass
    )
    .fetch_one(db)
    .await?;

    // @TODO: logging.

    // @TODO: hook_create.

    Ok(insert.id.to_string())
}

/// A helper function to add a new Person object to the database.
pub async fn read(
    db: &PgPool,
    _person_filter: Option<PersonFilter>
) -> anyhow::Result<Vec<Person>> {

    // @TODO build optional filter query.

    let persons = sqlx::query_as!(
        Person,
        r#"
SELECT id, name, email, pass
FROM person
        "#
    )
    .fetch_all(db)
    .await?;

    // @TODO: logging.

    // @TODO: hook_read.

    Ok(persons)
}
