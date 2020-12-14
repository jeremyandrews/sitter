/// A Person is the primary object defined and managed by Sitter. It could
/// also be referred to as the User.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use anyhow::Result;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgRow;

#[derive(Deserialize, Serialize, Debug)]
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
    /// A helper function to add a new Person object to the database.
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

    pub async fn read(_person_filter: Option<PersonFilter>, db: &PgPool) -> Result<Vec<Person>> {
        // @TODO build optional filter query.

        let mut persons = vec![];

        let records = sqlx::query(
            r#"
                SELECT id, name, email, pass
                FROM person
            "#
        )
            .map(|row: PgRow| {
                Person {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    pass: row.get(3)
                }
            })
            .fetch_all(db)
            .await?;
        
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
}


#[derive(Deserialize, Serialize, Debug)]
pub struct PersonFilter {
    pub ids: Option<Vec<Uuid>>,
    pub names: Option<Vec<String>>,
    pub emails: Option<Vec<String>>,
}
