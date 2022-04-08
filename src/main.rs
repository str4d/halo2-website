use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use time::Weekday;
use tower_http::services::ServeDir;
use tracing::instrument;
use tracing_subscriber::fmt::format::FmtSpan;
use unic_langid::LanguageIdentifier;

mod calendar;
mod i18n;

#[tokio::main]
async fn main() {
    // Filter traces based on the RUST_LOG env var, or, if it's not set,
    // default to show INFO-level details.
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "halo2_dev=info,tracing=info,axum=info".to_owned());

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let app = Router::new()
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .route("/", get(index))
        .route("/:lang", get(index_locale));

    // IPv6 + IPv6 any addr
    let addr = ([0, 0, 0, 0, 0, 0, 0, 0], 3000).into();
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    cal: calendar::Calendar,
    lang: i18n::Language,
    locales: &'static [i18n::LocaleInfo],
}

async fn index() -> IndexTemplate {
    render_index(i18n::EN_US).await
}

async fn index_locale(Path(lang): Path<LanguageIdentifier>) -> IndexTemplate {
    render_index(lang).await
}

#[instrument]
async fn render_index(lang: LanguageIdentifier) -> IndexTemplate {
    IndexTemplate {
        cal: calendar::Calendar::new(&[Weekday::Tuesday]),
        lang: i18n::Language::new(lang),
        locales: i18n::EXPLICIT_LOCALE_INFO,
    }
}
