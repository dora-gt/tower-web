use tower_service;
use tower_web::{derive_resource_impl, impl_web};

mod support;
use crate::support::*;

#[derive(Clone, Debug)]
struct One;

#[derive(Clone, Debug)]
struct Two;

#[derive(Clone, Debug)]
struct Three;

#[derive(Clone, Debug)]
struct Four;

impl_web! {
    impl One {
        #[get("/one")]
        fn action(&self) -> Result<&'static str, ()> {
            Ok("/one")
        }
    }

    impl Two {
        #[get("/two")]
        fn action(&self) -> Result<&'static str, ()> {
            Ok("/two")
        }
    }

    impl Three {
        #[get("/three")]
        fn action(&self) -> Result<&'static str, ()> {
            Ok("/three")
        }
    }

    impl Four {
        #[get("/four")]
        fn action(&self) -> Result<&'static str, ()> {
            Ok("/four")
        }
    }
}

#[test]
fn multi_resource() {
    use tower_service::NewService;

    let mut web = ::tower_web::ServiceBuilder::new()
        .resource(One)
        .resource(Two)
        .resource(Three)
        .resource(Four)
        .build_new_service()
        .new_service()
        .wait().unwrap();

    for path in &["/one", "/two", "/three", "/four"] {
        let response = web.call_unwrap(get!(*path));
        assert_ok!(response);
        assert_body!(response, *path);
    }
}
