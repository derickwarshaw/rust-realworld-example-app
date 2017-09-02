
CREATE SEQUENCE public.users_id_seq;

CREATE TABLE public.Users (
                Id INTEGER NOT NULL DEFAULT nextval('public.users_id_seq'),
                Email VARCHAR(50) NOT NULL,
                Token VARCHAR(250) NOT NULL,
                UserName VARCHAR(150) NOT NULL,
                Bio text,
                Image VARCHAR(250),
                CONSTRAINT pk_users PRIMARY KEY (Id)
);


ALTER SEQUENCE public.users_id_seq OWNED BY public.Users.Id;

CREATE UNIQUE INDEX ix_email
 ON public.Users
 ( Email ASC );

CREATE UNIQUE INDEX ix_username
 ON public.Users
 ( UserName ASC );

CREATE SEQUENCE public.followings_id_seq;

CREATE TABLE public.Followings (
                Id INTEGER NOT NULL DEFAULT nextval('public.followings_id_seq'),
                FollowingId INTEGER NOT NULL,
                FollowerId INTEGER NOT NULL,
                CONSTRAINT followings_pk PRIMARY KEY (Id)
);


ALTER SEQUENCE public.followings_id_seq OWNED BY public.Followings.Id;

CREATE UNIQUE INDEX ix_followings
 ON public.Followings
 ( FollowingId ASC, FollowerId ASC );

CREATE SEQUENCE public.tags_id_seq;

CREATE TABLE public.Tags (
                Id INTEGER NOT NULL DEFAULT nextval('public.tags_id_seq'),
                Tag VARCHAR(250) NOT NULL,
                CONSTRAINT pk_tags PRIMARY KEY (Id)
);


ALTER SEQUENCE public.tags_id_seq OWNED BY public.Tags.Id;

CREATE UNIQUE INDEX ix_tag
 ON public.Tags
 ( Tag ASC );

CREATE SEQUENCE public.articles_id_seq;

CREATE TABLE public.Articles (
                Id INTEGER NOT NULL DEFAULT nextval('public.articles_id_seq'),
                Slug VARCHAR(250) NOT NULL,
                Title VARCHAR(250) NOT NULL,
                Description VARCHAR(250) NOT NULL,
                Body text NOT NULL,
                Created TIMESTAMP NOT NULL,
                Updated TIMESTAMP,
                Author INTEGER NOT NULL,
                CONSTRAINT pk_articles PRIMARY KEY (Id)
);


ALTER SEQUENCE public.articles_id_seq OWNED BY public.Articles.Id;

CREATE UNIQUE INDEX ix_slug
 ON public.Articles
 ( Slug ASC );

CREATE SEQUENCE public.favoritedarticles_id_seq;

CREATE TABLE public.FavoritedArticles (
                Id INTEGER NOT NULL DEFAULT nextval('public.favoritedarticles_id_seq'),
                ArticleId INTEGER NOT NULL,
                UserId INTEGER NOT NULL,
                CONSTRAINT favoritedarticles_pk PRIMARY KEY (Id)
);


ALTER SEQUENCE public.favoritedarticles_id_seq OWNED BY public.FavoritedArticles.Id;

CREATE SEQUENCE public.comments_id_seq;

CREATE TABLE public.Comments (
                Id INTEGER NOT NULL DEFAULT nextval('public.comments_id_seq'),
                createdAt TIMESTAMP NOT NULL,
                updatedAt TIMESTAMP,
                body text NOT NULL,
                ArticleId INTEGER NOT NULL,
                Author INTEGER NOT NULL,
                CONSTRAINT pk_comments PRIMARY KEY (Id)
);


ALTER SEQUENCE public.comments_id_seq OWNED BY public.Comments.Id;

CREATE SEQUENCE public.articletags_id_seq;

CREATE TABLE public.ArticleTags (
                Id INTEGER NOT NULL DEFAULT nextval('public.articletags_id_seq'),
                ArticleId INTEGER NOT NULL,
                TagId INTEGER NOT NULL,
                CONSTRAINT articletags_pk PRIMARY KEY (Id)
);


ALTER SEQUENCE public.articletags_id_seq OWNED BY public.ArticleTags.Id;

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