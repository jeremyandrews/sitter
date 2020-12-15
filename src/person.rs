/// A Person is the primary object defined and managed by Sitter. It could
/// also be referred to as the User.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{Done, FromRow, PgPool, Row};

use crate::{email, logger, password};

#[derive(Serialize, FromRow, Debug)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub pass: String,
}

#[allow(unused_variables)]
pub trait PersonHooks {
    /// Validate the PersonRequest.
    fn validate(person: &PersonRequest) -> Result<()>;
    /// Do any necessary preprocessing to the PersonRequest.
    fn presave(person: &mut PersonRequest) -> Result<()>;
    /// Do any necessary postprocessing to the Person.
    fn postsave(person: &mut Person, action: &str) -> Result<()>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PersonRequest {
    pub name: String,
    pub email: String,
    pub pass: String,
}

impl Person {
    /// Add a Person object to the database.
    pub async fn create(person_request: PersonRequest, db: &PgPool) -> Result<Person> {
        // @TODO: Properly register and invoke hook_validate().
        password::Password::validate(&person_request)?;
        email::Email::validate(&person_request)?;

        // @TODO: Properly register and invoke hook_presave().
        let mut request = person_request;
        password::Password::presave(&mut request)?;

        let mut transaction = db.begin().await?;
        let mut person = sqlx::query("INSERT INTO person (name, email, pass) VALUES ($1, $2, $3) RETURNING id, name, email, pass")
            .bind(&request.name)
            .bind(&request.email)
            .bind(&request.pass)
            .map(|row: PgRow| {
                Person {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    pass: row.get(3)
                }
            })
            .fetch_one(&mut transaction)
            .await?;
        transaction.commit().await?;

        // @TODO: Properly register and invoke hook_postsave().
        logger::Logger::postsave(&mut person, "created")?;

        Ok(person)
    }

    /// List one or more Person objects from the database.
    pub async fn read(uuid: Option<Uuid>, db: &PgPool) -> Result<Vec<Person>> {
        // @TODO build optional filter query.

        let records = if let Some(id) = uuid {
            sqlx::query(
                r#"
                    SELECT id, name, email, pass
                    FROM person
                    WHERE id = $1
                "#,
            )
            .bind(id)
            .map(|row: PgRow| Person {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                pass: row.get(3),
            })
            .fetch_all(db)
            .await?
        } else {
            sqlx::query(
                r#"
                    SELECT id, name, email, pass
                    FROM person
                "#,
            )
            .map(|row: PgRow| Person {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                pass: row.get(3),
            })
            .fetch_all(db)
            .await?
        };

        let mut persons = vec![];
        for record in records {
            persons.push(Person {
                id: record.id,
                name: record.name,
                email: record.email,
                pass: record.pass,
            });
        }

        Ok(persons)
    }

    pub async fn update(id: Uuid, person_request: PersonRequest, db: &PgPool) -> Result<Person> {
        // @TODO: Properly register and invoke hook_validate().
        if !person_request.pass.is_empty() {
            password::Password::validate(&person_request)?;
        }
        email::Email::validate(&person_request)?;

        // @TODO: Properly register and invoke hook_presave().
        let mut request = person_request;
        if !request.pass.is_empty() {
            password::Password::presave(&mut request)?;
        }

        let mut transaction = db.begin().await.unwrap();
        let mut person = sqlx::query(
            r#"
                UPDATE person SET name = $1, email = $2
                WHERE id = $3
                RETURNING id, name, email, pass
            "#,
        )
        .bind(&request.name)
        .bind(&request.email)
        .bind(id)
        .map(|row: PgRow| Person {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            pass: row.get(3),
        })
        .fetch_one(&mut transaction)
        .await?;
        transaction.commit().await.unwrap();

        // @TODO: Properly register and invoke hook_postsave().
        logger::Logger::postsave(&mut person, "updated")?;

        Ok(person)
    }

    pub async fn delete(id: Uuid, db: &PgPool) -> Result<u64> {
        let mut transaction = db.begin().await?;
        let deleted = sqlx::query("DELETE FROM person WHERE id = $1")
            .bind(id)
            .execute(&mut transaction)
            .await?
            .rows_affected();

        transaction.commit().await?;

        // @TODO: Properly register and invoke hook_postsave().
        // @TODO: Load person from db before deleting it, for logging?
        //logger::Logger::postsave(&mut person, "deleted")?;

        Ok(deleted)
    }
}
