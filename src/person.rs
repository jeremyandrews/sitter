/// A Person is the primary object defined and managed by Sitter. It could
/// also be referred to as the User.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{Done, FromRow, PgPool, Row};

#[derive(Serialize, FromRow, Debug)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub pass: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PersonRequest {
    pub name: String,
    pub email: String,
    pub pass: String,
}

impl Person {
    /// Add a Person object to the database.
    pub async fn create(person: PersonRequest, db: &PgPool) -> Result<Person> {
        // @TODO: validate (and/or allow validation of) name, email and pass.

        let mut transaction = db.begin().await?;
        let person = sqlx::query("INSERT INTO person (name, email, pass) VALUES ($1, $2, $3) RETURNING id, name, email, pass")
            .bind(&person.name)
            .bind(&person.email)
            .bind(&person.pass)
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

        // @TODO: logging.

        // @TODO: hook_create.

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

        // @TODO: logging.

        // @TODO: hook_read.

        Ok(persons)
    }

    pub async fn update(id: Uuid, person: PersonRequest, db: &PgPool) -> Result<Person> {
        let mut transaction = db.begin().await.unwrap();
        let person = sqlx::query(
            r#"
                UPDATE person SET name = $1, email = $2
                WHERE id = $3
                RETURNING id, name, email, pass
            "#,
        )
        .bind(&person.name)
        .bind(&person.email)
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
        Ok(deleted)
    }
}
