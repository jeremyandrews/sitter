pub mod db;
pub mod email;
pub mod person;

#[cfg(test)]
mod tests {
    #[async_std::test]
    async fn connect_to_database() {
        let _ = crate::db::connect().await;
    }
}
