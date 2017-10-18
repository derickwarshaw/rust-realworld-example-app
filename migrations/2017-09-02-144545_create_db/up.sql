
CREATE TABLE public.Users (
                Id SERIAL PRIMARY KEY,
                Email VARCHAR(50) NOT NULL,
                Token VARCHAR(250) NOT NULL,
                UserName VARCHAR(150) NOT NULL,
                Bio text,
                Image VARCHAR(250)
);


CREATE UNIQUE INDEX ix_email
 ON public.Users
 ( Email ASC );

CREATE UNIQUE INDEX ix_username
 ON public.Users
 ( UserName ASC );

CREATE TABLE public.Followings (
                Id SERIAL PRIMARY KEY,
                FollowingId INTEGER NOT NULL,
                FollowerId INTEGER NOT NULL
);


CREATE UNIQUE INDEX ix_followings
 ON public.Followings
 ( FollowingId ASC, FollowerId ASC );

CREATE TABLE public.Tags (
                Id SERIAL PRIMARY KEY,
                Tag VARCHAR(250) NOT NULL
);


CREATE UNIQUE INDEX ix_tag
 ON public.Tags
 ( Tag ASC );

CREATE TABLE public.Articles (
                Id SERIAL PRIMARY KEY,
                Slug VARCHAR(250) NOT NULL,
                Title VARCHAR(250) NOT NULL,
                Description VARCHAR(250) NOT NULL,
                Body text NOT NULL,
                CreatedAt TIMESTAMP NOT NULL,
                UpdatedAt TIMESTAMP,
                Author INTEGER NOT NULL
);


CREATE UNIQUE INDEX ix_slug
 ON public.Articles
 ( Slug ASC );

CREATE TABLE public.FavoritedArticles (
                Id SERIAL PRIMARY KEY,
                ArticleId INTEGER NOT NULL,
                UserId INTEGER NOT NULL
);


CREATE TABLE public.Comments (
                Id SERIAL PRIMARY KEY,
                createdAt TIMESTAMP NOT NULL,
                updatedAt TIMESTAMP,
                body text NOT NULL,
                ArticleId INTEGER NOT NULL,
                Author INTEGER NOT NULL
);


CREATE TABLE public.ArticleTags (
                Id SERIAL PRIMARY KEY,
                ArticleId INTEGER NOT NULL,
                TagId INTEGER NOT NULL
);


ALTER TABLE public.Articles ADD CONSTRAINT fk_articles_users
FOREIGN KEY (Author)
REFERENCES public.Users (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE public.Comments ADD CONSTRAINT fk_comments_users
FOREIGN KEY (Author)
REFERENCES public.Users (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE public.FavoritedArticles ADD CONSTRAINT fk_favoritedarticles_users
FOREIGN KEY (UserId)
REFERENCES public.Users (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE public.Followings ADD CONSTRAINT fk_followings_users
FOREIGN KEY (FollowerId)
REFERENCES public.Users (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE public.Followings ADD CONSTRAINT fk_followings_users1
FOREIGN KEY (FollowingId)
REFERENCES public.Users (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE public.ArticleTags ADD CONSTRAINT fk_articletags_tags
FOREIGN KEY (TagId)
REFERENCES public.Tags (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE public.ArticleTags ADD CONSTRAINT fk_articletags_articles
FOREIGN KEY (ArticleId)
REFERENCES public.Articles (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE public.Comments ADD CONSTRAINT fk_comments_articles
FOREIGN KEY (ArticleId)
REFERENCES public.Articles (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE public.FavoritedArticles ADD CONSTRAINT fk_favoritedarticles_articles
FOREIGN KEY (ArticleId)
REFERENCES public.Articles (Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;