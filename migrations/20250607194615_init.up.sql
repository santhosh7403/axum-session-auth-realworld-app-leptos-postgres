CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS Users (
    id int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username text NOT NULL UNIQUE,
    email text NOT NULL UNIQUE,
    password text NOT NULL,
    bio text NULL,
    image text NULL
);

CREATE TABLE IF NOT EXISTS Follows (
    follower_id int NOT NULL REFERENCES Users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    influencer_id int NOT NULL REFERENCES Users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY (follower_id, influencer_id)
);

CREATE TABLE IF NOT EXISTS Articles (
    slug text NOT NULL PRIMARY KEY,
    author_id int NOT NULL REFERENCES Users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    title text NOT NULL,
    description text NOT NULL,
    body text NOT NULL,
    created_at TIMESTAMPTZ NOT NULL default NOW(),
    updated_at TIMESTAMPTZ NOT NULL default NOW(),
    fts_document tsvector
);

CREATE OR REPLACE FUNCTION articles_tsvector_trigger() RETURNS trigger AS $$
BEGIN
    new.fts_document :=
        setweight(to_tsvector('english', coalesce(new.title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(new.description, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(new.body, '')), 'C');
    RETURN new;
END
$$ LANGUAGE plpgsql;

-- Trigger for INSERT/UPDATE
CREATE TRIGGER articles_tsvector_update
BEFORE INSERT OR UPDATE ON Articles
FOR EACH ROW EXECUTE FUNCTION articles_tsvector_trigger();

-- GIN index for full-text search
CREATE INDEX IF NOT EXISTS articles_tsvector_idx ON Articles USING GIN (fts_document);


CREATE TABLE IF NOT EXISTS ArticleTags (
    article text NOT NULL REFERENCES Articles(slug) ON DELETE CASCADE ON UPDATE CASCADE,
    tag text NOT NULL,
    PRIMARY KEY (article, tag)
);

CREATE INDEX IF NOT EXISTS tags ON ArticleTags (tag);

CREATE TABLE IF NOT EXISTS FavArticles (
    article text NOT NULL REFERENCES Articles(slug) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id int NOT NULL REFERENCES Users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY (article, user_id)
);

CREATE TABLE IF NOT EXISTS Comments (
    id int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    article text NOT NULL REFERENCES Articles(slug) ON DELETE CASCADE ON UPDATE CASCADE,
    user_id int NOT NULL REFERENCES Users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    body text NOT NULL,
    created_at TIMESTAMPTZ NOT NULL default NOW()
);
