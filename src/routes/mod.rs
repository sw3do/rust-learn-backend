mod home;

use axum::Router;
use axum::routing::get;


pub fn get_app() -> Router {
    Router::new()
        .route("/", get(home::homepage))
}
