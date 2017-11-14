﻿
CREATE TABLE adempiere.conduit_Users (
                conduit_Users_Id SERIAL PRIMARY KEY,

  conduit_Users_uu character varying(36) DEFAULT NULL::character varying,
  ad_client_id numeric(10,0) NOT NULL,
  ad_org_id numeric(10,0) NOT NULL,  
  updatedby numeric(10,0) NOT NULL,
  updated timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  isactive character(1) NOT NULL,
  created timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  createdby numeric(10,0) NOT NULL,
                  
                Email VARCHAR(50) NOT NULL,
                Token VARCHAR(250) NOT NULL,
                UserName VARCHAR(150) NOT NULL,
                Bio text,
                Image VARCHAR(250)
);


CREATE UNIQUE INDEX ix_email
 ON adempiere.conduit_Users
 ( Email ASC );

CREATE UNIQUE INDEX ix_username
 ON adempiere.conduit_Users
 ( UserName ASC );

CREATE TABLE adempiere.conduit_Followings (
                conduit_Followings_Id SERIAL PRIMARY KEY,

  conduit_Followings_uu character varying(36) DEFAULT NULL::character varying,
  ad_client_id numeric(10,0) NOT NULL,
  ad_org_id numeric(10,0) NOT NULL,  
  updatedby numeric(10,0) NOT NULL,
  updated timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  isactive character(1) NOT NULL,
  created timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  createdby numeric(10,0) NOT NULL,


                FollowingId INTEGER NOT NULL,
                FollowerId INTEGER NOT NULL
);


CREATE UNIQUE INDEX ix_followings
 ON adempiere.conduit_Followings
 ( FollowingId ASC, FollowerId ASC );

CREATE TABLE adempiere.conduit_Tags (
                conduit_Tags_Id SERIAL PRIMARY KEY,

  conduit_Tags_uu character varying(36) DEFAULT NULL::character varying,
  ad_client_id numeric(10,0) NOT NULL,
  ad_org_id numeric(10,0) NOT NULL,  
  updatedby numeric(10,0) NOT NULL,
  updated timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  isactive character(1) NOT NULL,
  created timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  createdby numeric(10,0) NOT NULL,


                Tag VARCHAR(250) NOT NULL
);


CREATE UNIQUE INDEX ix_tag
 ON adempiere.conduit_Tags
 ( Tag ASC );

CREATE TABLE adempiere.conduit_Articles (
                conduit_Articles_Id SERIAL PRIMARY KEY,

  conduit_Articles_uu character varying(36) DEFAULT NULL::character varying,
  ad_client_id numeric(10,0) NOT NULL,
  ad_org_id numeric(10,0) NOT NULL,  
  updatedby numeric(10,0) NOT NULL,
  updated timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  isactive character(1) NOT NULL,
  created timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  createdby numeric(10,0) NOT NULL,


                Slug VARCHAR(250) NOT NULL,
                Title VARCHAR(250) NOT NULL,
                Description VARCHAR(250) NOT NULL,
                Body text NOT NULL,
                CreatedAt TIMESTAMP NOT NULL,
                UpdatedAt TIMESTAMP,
                Author INTEGER NOT NULL
);


CREATE UNIQUE INDEX ix_slug
 ON adempiere.conduit_Articles
 ( Slug ASC );

CREATE TABLE adempiere.conduit_FavoritedArticles (
                conduit_FavoritedArticles_Id SERIAL PRIMARY KEY,

  conduit_FavoritedArticles_uu character varying(36) DEFAULT NULL::character varying,
  ad_client_id numeric(10,0) NOT NULL,
  ad_org_id numeric(10,0) NOT NULL,  
  updatedby numeric(10,0) NOT NULL,
  updated timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  isactive character(1) NOT NULL,
  created timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  createdby numeric(10,0) NOT NULL,


                ArticleId INTEGER NOT NULL,
                UserId INTEGER NOT NULL
);


CREATE TABLE adempiere.conduit_Comments (
                conduit_Comments_Id SERIAL PRIMARY KEY,

  conduit_Comments_uu character varying(36) DEFAULT NULL::character varying,
  ad_client_id numeric(10,0) NOT NULL,
  ad_org_id numeric(10,0) NOT NULL,  
  updatedby numeric(10,0) NOT NULL,
  updated timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  isactive character(1) NOT NULL,
  created timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  createdby numeric(10,0) NOT NULL,


                createdAt TIMESTAMP NOT NULL,
                updatedAt TIMESTAMP,
                body text NOT NULL,
                ArticleId INTEGER NOT NULL,
                Author INTEGER NOT NULL
);


CREATE TABLE adempiere.conduit_ArticleTags (
                conduit_ArticleTags_Id SERIAL PRIMARY KEY,

  conduit_ArticleTags_uu character varying(36) DEFAULT NULL::character varying,
  ad_client_id numeric(10,0) NOT NULL,
  ad_org_id numeric(10,0) NOT NULL,  
  updatedby numeric(10,0) NOT NULL,
  updated timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  isactive character(1) NOT NULL,
  created timestamp without time zone NOT NULL DEFAULT statement_timestamp(),
  createdby numeric(10,0) NOT NULL,


                ArticleId INTEGER NOT NULL,
                TagId INTEGER NOT NULL
);


ALTER TABLE adempiere.conduit_Articles ADD CONSTRAINT fk_articles_users
FOREIGN KEY (Author)
REFERENCES adempiere.conduit_Users (conduit_Users_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE adempiere.conduit_Comments ADD CONSTRAINT fk_comments_users
FOREIGN KEY (Author)
REFERENCES adempiere.conduit_Users (conduit_Users_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE adempiere.conduit_FavoritedArticles ADD CONSTRAINT fk_favoritedarticles_users
FOREIGN KEY (UserId)
REFERENCES adempiere.conduit_Users (conduit_Users_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE adempiere.conduit_Followings ADD CONSTRAINT fk_followings_users
FOREIGN KEY (FollowerId)
REFERENCES adempiere.conduit_Users (conduit_Users_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE adempiere.conduit_Followings ADD CONSTRAINT fk_followings_users1
FOREIGN KEY (FollowingId)
REFERENCES adempiere.conduit_Users (conduit_Users_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE adempiere.conduit_ArticleTags ADD CONSTRAINT fk_articletags_tags
FOREIGN KEY (TagId)
REFERENCES adempiere.conduit_Tags (conduit_Tags_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE adempiere.conduit_ArticleTags ADD CONSTRAINT fk_articletags_articles
FOREIGN KEY (ArticleId)
REFERENCES adempiere.conduit_Articles (conduit_Articles_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE adempiere.conduit_Comments ADD CONSTRAINT fk_comments_articles
FOREIGN KEY (ArticleId)
REFERENCES adempiere.conduit_Articles (conduit_Articles_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;

ALTER TABLE adempiere.conduit_FavoritedArticles ADD CONSTRAINT fk_favoritedarticles_articles
FOREIGN KEY (ArticleId)
REFERENCES adempiere.conduit_Articles (conduit_Articles_Id)
ON DELETE RESTRICT
ON UPDATE RESTRICT
NOT DEFERRABLE;