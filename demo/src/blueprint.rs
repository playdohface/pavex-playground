use pavex::blueprint::constructor::CloningStrategy;
use pavex::blueprint::{constructor::Lifecycle, router::GET, router::POST, Blueprint};
use pavex::f;
use pavex::kit::ApiKit;

/// The main blueprint, containing all the routes, constructors and error handlers
/// required by our API.
pub fn blueprint() -> Blueprint {
    println!("Am I being called?");
    let mut bp = Blueprint::new();
    ApiKit::new().register(&mut bp);

    add_telemetry_middleware(&mut bp);
    bp.constructor(
        f!(crate::lifecycle::Lifecycle::startup),
        Lifecycle::Singleton,
    );
    bp.constructor(
        f!(crate::user_agent::UserAgent::extract),
        Lifecycle::RequestScoped,
    )
    .error_handler(f!(crate::user_agent::invalid_user_agent));
    bp.constructor(
        f!(crate::repository::book::BookMemoryRepository::new),
        Lifecycle::Singleton,
    );

    bp.route(GET, "/api/greet/:name", f!(crate::routes::greet::greet));
    bp.route(GET, "/api/ping", f!(crate::routes::status::ping));
    bp.route(POST, "/api/books", f!(crate::routes::books::save));
    bp.route(GET, "/api/books/:id", f!(crate::routes::books::get_by_id));
    bp
}

/// Add the telemetry middleware, as well as the constructors of its dependencies.
fn add_telemetry_middleware(bp: &mut Blueprint) {
    bp.constructor(
        f!(crate::telemetry::RootSpan::new),
        Lifecycle::RequestScoped,
    )
    .cloning(CloningStrategy::CloneIfNecessary);

    bp.wrap(f!(crate::telemetry::logger));
    bp.error_observer(f!(crate::telemetry::log_error));
}
