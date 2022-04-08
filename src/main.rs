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

const EXAMPLE_CIRCUIT: &str = "const WIDTH: usize = 3;
const RATE: usize = 2;

#[derive(Clone)]
struct HashConfig {
    poseidon_config: poseidon::Pow5Config<pallas::Base, WIDTH, RATE>,
    message_col: Column<Advice>,
    digest_col: Column<Instance>,
}

struct HashCircuit {
    message: Option<pallas::Base>,
}

impl Circuit<pallas::Base> for HashCircuit {
    type Config = HashConfig;
    type FloorPlanner = floor_planner::V1;

    fn without_witnesses(&self) -> Self {
        Self { message: None }
    }

    fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> HashConfig {
        let state = [0; WIDTH].map(|_| meta.advice_column());
        let partial_sbox = meta.advice_column();
        let rc_a = [0; WIDTH].map(|_| meta.fixed_column());
        let rc_b = [0; WIDTH].map(|_| meta.fixed_column());
        let digest_col = meta.instance_column();
        meta.enable_constant(rc_b[0]);
        meta.enable_equality(digest_col);

        let poseidon_config =
            poseidon::Pow5Chip::configure::<P128Pow5T3>(meta, state, partial_sbox, rc_a, rc_b);

        HashConfig {
            poseidon_config,
            message_col: state[0],
            digest_col,
        }
    }

    fn synthesize(
        &self,
        config: HashConfig,
        mut layouter: impl Layouter<pallas::Base>,
    ) -> Result<(), Error> {
        let message = layouter.assign_region(
            || \"load message\",
            |mut region| {
                let word = region.assign_advice(
                    || \"message\",
                    config.message_col,
                    0,
                    || self.message.ok_or(Error::Synthesis),
                )?;
                Ok([word])
            },
        )?;

        let hasher = poseidon::Hash::<_, _, P128Pow5T3, ConstantLength<1>, WIDTH, RATE>::init(
            poseidon::Pow5Chip::construct(config.poseidon_config),
            layouter.namespace(|| \"init\"),
        )?;
        let output = hasher.hash(layouter.namespace(|| \"digest\"), message)?;

        layouter.constrain_instance(output.cell(), config.digest_col, 0)
    }
}";

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    cal: calendar::Calendar,
    example_circuit: &'static str,
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
        example_circuit: EXAMPLE_CIRCUIT,
        lang: i18n::Language::new(lang),
        locales: i18n::EXPLICIT_LOCALE_INFO,
    }
}
