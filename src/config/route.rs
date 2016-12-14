use router;

use handlers;
use tests;
use private;

pub fn configure_routes(router: &mut router::Router) {
    // configure the router
    router
        .get("/", handlers::index, "index")
        .get("/:page", handlers::get_root_page, "get_root_index")
        .get("/:category/:page", handlers::get_category_page, "get_category_page")

        .get("/hello_world", tests::hello_world, "hello_world")
        .get("/locales_test/:lang", tests::locales_test, "locales_test");

    // configure private routes
    private::configure_routes(router);
}

