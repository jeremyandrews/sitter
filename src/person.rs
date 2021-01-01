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
    pub email: String,
    pub pass: String,
}

#[allow(unused_variables)]
pub trait PersonHook {
    /// Validate the PersonRequest.
    fn validate(&self, person: &PersonRequest, action: &str) -> Result<()>;
    /// Do any necessary preprocessing to the PersonRequest.
    fn prepare(&self, person: &mut PersonRequest, action: &str) -> Result<()>;
    /// Do any necessary preprocessing to the Id (Read and Delete).
    fn prepare_id(&self, id: &mut Uuid, action: &str) -> Result<()>;
    /// Do any necessary postprocessing to the Person.
    fn processed(&self, person: &mut Person, action: &str) -> Result<()>;
}

pub struct PersonHooks {
    hooks: Vec<Box<dyn PersonHook + Send>>,
}
impl PersonHooks {
    pub fn initialize() -> Self {
        Self { hooks: Vec::new() }
    }

    pub fn register_hook<H: PersonHook + 'static + Send>(&mut self, hook: H) {
        self.hooks.push(Box::new(hook));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PersonRequest {
    pub email: String,
    pub pass: String,
}

impl Person {
    /// Add a Person object to the database.
    pub async fn create(
        person_request: PersonRequest,
        db: &PgPool,
        hooks: &PersonHooks,
    ) -> Result<Person> {
        let action = "create";

        // Invoke hook_validate().
        for hook in &hooks.hooks {
            hook.validate(&person_request, action)?;
        }

        // Invoke hook_prepare().
        let mut request = person_request;
        for hook in &hooks.hooks {
            hook.prepare(&mut request, action)?;
        }

        let mut transaction = db.begin().await?;
        let mut person = sqlx::query("INSERT INTO person (email, pass) VALUES ($1, $2) RETURNING id, email, pass")
            .bind(&request.email)
            .bind(&request.pass)
            .map(|row: PgRow| {
                Person {
                    id: row.get(0),
                    email: row.get(1),
                    pass: row.get(2)
                }
            })
            .fetch_one(&mut transaction)
            .await?;
        transaction.commit().await?;

        // Invoke hook_processed().
        for hook in &hooks.hooks {
            hook.processed(&mut person, action)?;
        }

        Ok(person)
    }

    /// List one or more Person objects from the database.
    pub async fn read(uuid: Option<Uuid>, db: &PgPool, hooks: &PersonHooks) -> Result<Vec<Person>> {
        let action = "read";

        if let Some(mut id) = uuid {
            // Invoke hook_prepare_id().
            for hook in &hooks.hooks {
                hook.prepare_id(&mut id, action)?;
            }
        }

        let records = if let Some(id) = uuid {
            sqlx::query(
                r#"
                    SELECT id, email, pass
                    FROM person
                    WHERE id = $1
                "#,
            )
            .bind(id)
            .map(|row: PgRow| Person {
                id: row.get(0),
                email: row.get(1),
                pass: row.get(2),
            })
            .fetch_all(db)
            .await?
        } else {
            sqlx::query(
                r#"
                    SELECT id, email, pass
                    FROM person
                "#,
            )
            .map(|row: PgRow| Person {
                id: row.get(0),
                email: row.get(1),
                pass: row.get(2),
            })
            .fetch_all(db)
            .await?
        };

        let mut persons = vec![];
        for record in records {
            persons.push(Person {
                id: record.id,
                email: record.email,
                pass: record.pass,
            });
        }

        Ok(persons)
    }

    pub async fn update(
        id: Uuid,
        person_request: PersonRequest,
        db: &PgPool,
        hooks: &PersonHooks,
    ) -> Result<Person> {
        let action = "update";
        // @TODO: Properly register and invoke hook_validate().
        // Invoke hook_validate().
        for hook in &hooks.hooks {
            hook.validate(&person_request, action)?;
        }

        // Invoke hook_prepare().
        let mut request = person_request;
        if !request.pass.is_empty() {
            for hook in &hooks.hooks {
                hook.prepare(&mut request, action)?;
            }
        }

        let mut transaction = db.begin().await.unwrap();
        let mut person = sqlx::query(
            r#"
                UPDATE person SET email = $1
                WHERE id = $2
                RETURNING id, email, pass
            "#,
        )
        .bind(&request.email)
        .bind(id)
        .map(|row: PgRow| Person {
            id: row.get(0),
            email: row.get(1),
            pass: row.get(2),
        })
        .fetch_one(&mut transaction)
        .await?;
        transaction.commit().await.unwrap();

        // Invoke hook_processed().
        for hook in &hooks.hooks {
            hook.processed(&mut person, action)?;
        }

        Ok(person)
    }

    pub async fn delete(mut id: Uuid, db: &PgPool, hooks: &PersonHooks) -> Result<u64> {
        let action = "delete";

        // Invoke hook_prepare_id().
        for hook in &hooks.hooks {
            hook.prepare_id(&mut id, action)?;
        }

        let mut transaction = db.begin().await?;
        let deleted = sqlx::query("DELETE FROM person WHERE id = $1")
            .bind(id)
            .execute(&mut transaction)
            .await?
            .rows_affected();

        transaction.commit().await?;

        // @TODO: Load person from db before deleting it, for logging?
        // Invoke hook_processed().
        //for hook in &hooks.hooks {
        //    hook.processed(&mut person, "deleted")?;
        //}

        Ok(deleted)
    }
}
