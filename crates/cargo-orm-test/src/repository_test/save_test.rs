#[cfg(test)]
mod tests {
    use crate::{init_sqlite, test_entities::User};
    use cargo_orm_core::prelude::*;

    #[tokio::test]
    async fn test_save() {
        let user = User {
            id: 1,
            name: "John".to_string(),
        };
        let driver = init_sqlite().await;
        let mut conn = driver.acquire_conn().await.unwrap();
        user.save(&mut conn).await.unwrap();
        let _saved_user = User::get_by_id(1, &mut conn).await.unwrap();
    }
}
