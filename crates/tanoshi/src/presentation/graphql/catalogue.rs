use super::{chapter::Chapter, common::InputList, guard::AdminGuard, manga::Manga};

use crate::{
    domain::services::{chapter::ChapterService, manga::MangaService},
    infrastructure::domain::repositories::{
        chapter::ChapterRepositoryImpl, manga::MangaRepositoryImpl,
    },
};

use async_graphql::{Context, Object, Result};
use rayon::prelude::*;

#[derive(Default)]
pub struct CatalogueRoot;

#[Object]
impl CatalogueRoot {
    async fn get_popular_manga(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "source id")] source_id: i64,
        #[graphql(desc = "page")] page: i64,
    ) -> Result<Vec<Manga>> {
        let fetched_manga = ctx
            .data::<MangaService<MangaRepositoryImpl>>()?
            .fetch_source_popular_manga(source_id, page)
            .await?
            .into_par_iter()
            .map(Manga::from)
            .collect();

        Ok(fetched_manga)
    }
    async fn get_latest_manga(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "source id")] source_id: i64,
        #[graphql(desc = "page")] page: i64,
    ) -> Result<Vec<Manga>> {
        let fetched_manga = ctx
            .data::<MangaService<MangaRepositoryImpl>>()?
            .fetch_source_latest_manga(source_id, page)
            .await?
            .into_par_iter()
            .map(Manga::from)
            .collect();

        Ok(fetched_manga)
    }

    async fn browse_source(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "source id")] source_id: i64,
        #[graphql(desc = "page")] page: i64,
        #[graphql(desc = "query")] query: Option<String>,
        #[graphql(desc = "filters")] filters: Option<InputList>,
    ) -> Result<Vec<Manga>> {
        let fetched_manga = ctx
            .data::<MangaService<MangaRepositoryImpl>>()?
            .fetch_source_manga(source_id, page, query, filters.map(|filters| filters.0))
            .await?
            .into_par_iter()
            .map(Manga::from)
            .collect();

        Ok(fetched_manga)
    }

    async fn manga_by_source_path(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "source id")] source_id: i64,
        #[graphql(desc = "path to manga in source")] path: String,
    ) -> Result<Manga> {
        let manga = ctx
            .data::<MangaService<MangaRepositoryImpl>>()?
            .fetch_manga_by_source_path(source_id, &path)
            .await?;

        Ok(manga.into())
    }

    async fn manga(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "manga id")] id: i64,
        #[graphql(desc = "refresh data from source", default = false)] refresh: bool,
    ) -> Result<Manga> {
        let manga = ctx
            .data::<MangaService<MangaRepositoryImpl>>()?
            .fetch_manga_by_id(id, refresh)
            .await?;

        Ok(manga.into())
    }

    async fn chapter(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "chapter id")] id: i64,
    ) -> Result<Chapter> {
        let chapter = ctx
            .data::<ChapterService<ChapterRepositoryImpl>>()?
            .fetch_chapter_by_id(id)
            .await?
            .into();

        Ok(chapter)
    }
}

#[derive(Default)]
pub struct CatalogueMutationRoot;

#[Object]
impl CatalogueMutationRoot {
    #[graphql(guard = "AdminGuard::new()")]
    async fn remove_chapter(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "chapter id")] id: i64,
    ) -> Result<bool> {
        ctx.data::<ChapterService<ChapterRepositoryImpl>>()?
            .delete_chapter(id)
            .await?;

        Ok(true)
    }
}
