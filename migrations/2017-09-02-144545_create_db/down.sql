-- This file should undo anything in `up.sql`

drop table public.ArticleTags;

drop SEQUENCE public.articletags_id_seq;

drop TABLE public.Comments;

drop SEQUENCE public.comments_id_seq;

drop TABLE public.FavoritedArticles;

drop SEQUENCE public.favoritedarticles_id_seq;

drop TABLE public.Articles;

drop SEQUENCE public.articles_id_seq;

drop TABLE public.Tags;

drop SEQUENCE public.tags_id_seq;

drop TABLE public.Followings;

drop SEQUENCE public.followings_id_seq;

drop TABLE public.Users;

drop SEQUENCE public.users_id_seq;
