pub mod history {
    pub mod add {
        //! Add items to a user's watch history
        //!
        //! <https://trakt.docs.apiary.io/#reference/sync/add-to-history/add-items-to-watched-history>

        use bytes::BufMut;
        use serde::{Deserialize, Serialize};
        use time::OffsetDateTime;
        use trakt_core::{error::IntoHttpError, AuthRequirement, Context, Metadata};

        use crate::smo::{Episode, Ids, Movie, Season, Show};

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
        pub struct MovieItem {
            #[serde(flatten)]
            pub movie: Movie,

            #[serde(with = "time::serde::iso8601::option")]
            pub watched_at: Option<OffsetDateTime>,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
        pub struct ShowItem {
            #[serde(flatten)]
            pub show: Show,

            #[serde(with = "time::serde::iso8601::option")]
            pub watched_at: Option<OffsetDateTime>,
        }
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
        pub struct SeasonItem {
            #[serde(flatten)]
            pub season: Season,

            #[serde(with = "time::serde::iso8601::option")]
            pub watched_at: Option<OffsetDateTime>,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
        pub struct EpisodeItem {
            #[serde(flatten)]
            pub episode: Episode,

            #[serde(with = "time::serde::iso8601::option")]
            pub watched_at: Option<OffsetDateTime>,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Default)]
        pub struct Request {
            pub movies: Vec<MovieItem>,
            pub shows: Vec<ShowItem>,
            pub seasons: Vec<SeasonItem>,
            pub episodes: Vec<EpisodeItem>,
        }

        impl trakt_core::Request for Request {
            type Response = Response;
            const METADATA: Metadata = Metadata {
                endpoint: "/sync/history",
                method: http::Method::POST,
                auth: AuthRequirement::Required,
            };

            fn try_into_http_request<T: Default + BufMut>(
                self,
                ctx: Context,
            ) -> Result<http::Request<T>, IntoHttpError> {
                let body = T::default();
                let mut writer = body.writer();

                let json = serde_json::json!({
                    "movies": self.movies,
                    "shows": self.shows,
                    "seasons": self.seasons,
                    "episodes": self.episodes,
                });
                serde_json::to_writer(&mut writer, &json)?;

                trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
            }
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
        pub struct Added {
            movies: usize,
            episodes: usize,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
        pub struct NotFoundItem {
            pub ids: Ids,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
        pub struct NotFound {
            pub movies: Vec<NotFoundItem>,
            pub shows: Vec<NotFoundItem>,
            pub seasons: Vec<NotFoundItem>,
            pub episodes: Vec<NotFoundItem>,
        }

        #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
        #[trakt(expected = CREATED)]
        pub struct Response {
            pub added: Added,
            pub not_found: NotFound,
        }
    }
}
