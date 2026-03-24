#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Comment {
    pub id: i32,
    pub article: String,
    pub username: String,
    pub body: String,
    pub created_at: String,
    pub user_image: Option<String>,
}

impl Comment {
    #[cfg(feature = "ssr")]
    pub async fn insert(
        article: String,
        username: String,
        body: String,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let db = crate::database::get_db();
        let user_id = sqlx::query_scalar!("SELECT id FROM Users WHERE username=$1", username).fetch_one(db).await?;
        sqlx::query!(
            "INSERT INTO Comments(article, user_id, body) VALUES ($1, $2, $3)",
            article,
            user_id,
            body
        )
        .execute(db)
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn get_all(article: String) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query!(
            "
        SELECT c.id, c.article, u.username, c.body, c.created_at, u.image FROM Comments as c
            JOIN Users as u ON u.id=c.user_id
        WHERE c.article=$1
        ORDER BY c.created_at",
            article
        )
        .map(|x| Self {
            id: x.id,
            article: x.article,
            username: x.username,
            body: x.body,
            created_at: x.created_at.format(super::DATE_FORMAT).to_string(),
            user_image: x.image,
        })
        .fetch_all(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn delete(
        id: i32,
        user: String,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let db = crate::database::get_db();
        let user_id = sqlx::query_scalar!("SELECT id FROM Users WHERE username=$1", user).fetch_one(db).await?;
        sqlx::query!("DELETE FROM Comments WHERE id=$1 and user_id=$2", id, user_id)
            .execute(db)
            .await
    }
}
