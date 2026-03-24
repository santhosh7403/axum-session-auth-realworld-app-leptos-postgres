use super::UserPreview;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Article {
    pub slug: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub description: String,
    pub created_at: String,
    pub favorites_count: i64,
    pub tag_list: Vec<String>,
    pub author: UserPreview,
    pub fav: bool,
    pub comments_count: i64,
}

impl Article {
    #[tracing::instrument]
    #[cfg(feature = "ssr")]
    pub async fn for_home_page(
        page: i64,
        amount: i64,
        tag: String,
        my_feed: bool,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let username = crate::auth::get_username();
        sqlx::query!(
            "
SELECT
    a.slug,
    a.title,
    a.description,
    a.created_at,
    (SELECT COUNT(*) FROM FavArticles WHERE article=a.slug) as favorites_count,
    (SELECT COUNT(*) FROM comments WHERE article=a.slug) as comments_count,
    u.username, u.image,
    EXISTS(SELECT 1 FROM FavArticles WHERE article=a.slug and user_id=(SELECT id FROM Users WHERE username=$5)) as fav,
    EXISTS(SELECT 1 FROM Follows WHERE follower_id=(SELECT id FROM Users WHERE username=$5) and influencer_id=u.id) as following,
    (SELECT string_agg(tag, ' ') FROM ArticleTags WHERE article = a.slug) as tag_list
FROM Articles as a
    JOIN Users as u ON a.author_id = u.id
WHERE
    CASE WHEN $3!='' THEN a.slug in (SELECT distinct article FROM ArticleTags WHERE tag=$3)
    ELSE 1=1
    END
    AND
    CASE WHEN $4 THEN u.id in (SELECT influencer_id FROM Follows WHERE follower_id=(SELECT id FROM Users WHERE username=$5))
    ELSE 1=1
    END
ORDER BY a.created_at desc
LIMIT $1 OFFSET $2",
            amount,
            page * amount,
            tag,
            my_feed,
            username,
        )
        .map(|x| Self {
            slug: x.slug,
            title: x.title,
            body: None, // no need
            fav: x.fav.unwrap_or_default(),
            description: x.description,
            created_at: x.created_at.format(super::DATE_FORMAT).to_string(),
            favorites_count: x.favorites_count.unwrap_or_default(),
            author: UserPreview {
                username: x.username,
                image: x.image,
                following: x.following.unwrap_or_default(),
            },
            tag_list: x
                .tag_list
                .unwrap_or_default()
                .split(' ')
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
            comments_count: x.comments_count.unwrap_or_default(),
        })
        .fetch_all(crate::database::get_db())
        .await
    }

    #[tracing::instrument(level = tracing::Level::DEBUG)]
    #[cfg(feature = "ssr")]
    pub async fn for_user_profile(
        username: String,
        favourites: bool,
    ) -> Result<Vec<Self>, sqlx::Error> {
        // use tracing::Level;

        let logged_user = crate::auth::get_username();
        sqlx::query!(
            "
SELECT
    a.slug,
    a.title,
    a.description,
    a.created_at,
    u.username,
    u.image,
    (SELECT COUNT(*) FROM FavArticles WHERE article=a.slug) as favorites_count,
    (SELECT COUNT(*) FROM comments WHERE article=a.slug) as comments_count,
    EXISTS(SELECT 1 FROM FavArticles WHERE article=a.slug and user_id=(SELECT id FROM Users WHERE username=$2)) as fav,
    EXISTS(SELECT 1 FROM Follows WHERE follower_id=(SELECT id FROM Users WHERE username=$2) and influencer_id=a.author_id) as following,
    (SELECT string_agg(tag, ' ') FROM ArticleTags WHERE article = a.slug) as tag_list
FROM Articles as a
    JOIN Users as u ON u.id = a.author_id
WHERE
    CASE WHEN $3 THEN
        EXISTS(SELECT fa.article, fa.user_id FROM FavArticles as fa WHERE fa.article=a.slug AND fa.user_id=(SELECT id FROM Users WHERE username=$1))
    ELSE u.username = $1
    END",
            username,
            logged_user,
            favourites,
        )
        .map(|x| Self {
            slug: x.slug,
            title: x.title,
            body: None, // no need
            fav: x.fav.unwrap_or_default(),
            description: x.description,
            created_at: x.created_at.format(super::DATE_FORMAT).to_string(),
            favorites_count: x.favorites_count.unwrap_or_default(),
            tag_list: x
                .tag_list
                .map(|x| x.split(' ').map(ToString::to_string).collect::<Vec<_>>())
                .unwrap_or_default(),
            author: UserPreview {
                username: x.username,
                image: x.image,
                following: x.following.unwrap_or_default(),
            },
            comments_count: x.comments_count.unwrap_or_default(),
        })
        .fetch_all(crate::database::get_db())
        .await
    }

    #[tracing::instrument(level = tracing::Level::DEBUG)]
    #[cfg(feature = "ssr")]
    pub async fn for_user_profile_home(
        username: String,
        favourites: bool,
        page: i64,
        amount: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let logged_user = crate::auth::get_username();
        sqlx::query!(
            "
SELECT
    a.slug,
    a.title,
    a.description,
    a.created_at,
    u.username,
    u.image,
    (SELECT COUNT(*) FROM FavArticles WHERE article=a.slug) as favorites_count,
    (SELECT COUNT(*) FROM comments WHERE article=a.slug) as comments_count,
    EXISTS(SELECT 1 FROM FavArticles WHERE article=a.slug and user_id=(SELECT id FROM Users WHERE username=$2)) as fav,
    EXISTS(SELECT 1 FROM Follows WHERE follower_id=(SELECT id FROM Users WHERE username=$2) and influencer_id=a.author_id) as following,
    (SELECT string_agg(tag, ' ') FROM ArticleTags WHERE article = a.slug) as tag_list
FROM Articles as a
    JOIN Users as u ON u.id = a.author_id
WHERE
    CASE WHEN $3 THEN
        EXISTS(SELECT fa.article, fa.user_id FROM FavArticles as fa WHERE fa.article=a.slug AND fa.user_id=(SELECT id FROM Users WHERE username=$1))
    ELSE u.username = $1
    END
    ORDER BY a.created_at desc
    LIMIT $4 OFFSET $5",
            username,
            logged_user,
            favourites,
            amount,
            page * amount,
        )
        .map(|x| Self {
            slug: x.slug,
            title: x.title,
            body: None, // no need
            fav: x.fav.unwrap_or_default(),
            description: x.description,
            created_at: x.created_at.format(super::DATE_FORMAT).to_string(),
            favorites_count: x.favorites_count.unwrap_or_default(),
            tag_list: x
                .tag_list
                .map(|x| x.split(' ').map(ToString::to_string).collect::<Vec<_>>())
                .unwrap_or_default(),
            author: UserPreview {
                username: x.username,
                image: x.image,
                following: x.following.unwrap_or_default(),
            },
            comments_count: x.comments_count.unwrap_or_default(),
        })
        .fetch_all(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn for_article(slug: String) -> Result<Self, sqlx::Error> {
        let username = crate::auth::get_username();
        sqlx::query!(
            "
    SELECT
        a.slug as slug,
        a.title as title,
        a.description as description,
        a.body as body,
        a.created_at as created_at,
        (SELECT string_agg(tag, ' ') FROM ArticleTags WHERE article = a.slug) as tag_list,
        (SELECT COUNT(*) FROM FavArticles WHERE article = a.slug) as fav_count,
        (SELECT COUNT(*) FROM comments WHERE article = a.slug) as comments_count,
        u.*,
        EXISTS(SELECT 1 FROM FavArticles WHERE article=a.slug and user_id=(SELECT id FROM Users WHERE username=$2)) as fav,
        EXISTS(SELECT 1 FROM Follows WHERE follower_id=(SELECT id FROM Users WHERE username=$2) and influencer_id=a.author_id) as following
    FROM Articles a
        JOIN Users u ON a.author_id = u.id
    WHERE slug = $1
    ",
            slug,
            username,
        )
        .map(|x| Self {
            slug: x.slug,
            title: x.title,
            description: x.description,
            body: Some(x.body),
            tag_list: x
                .tag_list
                .unwrap_or_default()
                .split_ascii_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>(),
            favorites_count: x.fav_count.unwrap_or_default(),
            created_at: x.created_at.format(super::DATE_FORMAT).to_string(),
            fav: x.fav.unwrap_or_default(),
            author: UserPreview {
                username: x.username,
                image: x.image,
                following: x.following.unwrap_or_default(),
            },
            comments_count: x.comments_count.unwrap_or_default(),
        })
        .fetch_one(crate::database::get_db())
        .await
    }

    #[cfg(feature = "ssr")]
    pub async fn delete(
        slug: String,
        author: String,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let db = crate::database::get_db();
        let author_id = sqlx::query_scalar!("SELECT id FROM Users WHERE username=$1", author)
            .fetch_one(db)
            .await?;
        sqlx::query!(
            "DELETE FROM Articles WHERE slug=$1 and author_id=$2",
            slug,
            author_id
        )
        .execute(db)
        .await
    }
}
