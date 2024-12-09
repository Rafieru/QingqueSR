use crate::handlers::auth::{account, product};
use axum::{routing::post, Router};

pub fn auth_router() -> Router<()> {
    let mut router = Router::new();

    router = router.route("/account/risky/api/check", post(account::risky::api::check));

    router = router.route(
        "/:product/mdk/shield/api/login",
        post(product::mdk::shield::api::login),
    );

    router = router.route(
        "/:product/mdk/shield/api/verify",
        post(product::mdk::shield::api::verify),
    );

    router = router.route(
        "/:product/combo/granter/login/v2/login",
        post(product::combo::granter::login::v2::login),
    );

    router
}
