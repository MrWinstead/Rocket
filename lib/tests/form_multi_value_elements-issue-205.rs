#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::vec;

use rocket::request::Form;

#[derive(FromForm)]
struct UserInfo {
    name: String,
    nicknames: Vec<String>,
}

#[post("/", data = "<form_data>")]
fn bug(form_data: Form<UserInfo>) -> String {
    let inner = form_data.into_inner();
    let mut output_string = inner.nicknames[0].to_owned();

    println!("form_data.nicknames: {:?}", inner.nicknames.len());
    output_string
}

#[cfg(feature = "testing")]
mod tests {
    use super::*;
    use rocket::testing::MockRequest;
    use rocket::http::Method::*;
    use rocket::http::ContentType;
    use rocket::http::Status;

    fn check_multivalue_form<T>(raw: &str, expected: Vec<T>) {
        let rocket = rocket::ignite().mount("/", routes![bug]);
        let mut req = MockRequest::new(Post, "/")
            .header(ContentType::Form)
            .body(format!("{}", raw));

        let mut response = req.dispatch_with(&rocket);
        let body_string = response.body().and_then(|b| b.into_string());
        assert_eq!(response.status(), Status::Ok);
        println!("body_string: {:?}", body_string);

    }
    #[test]
    fn test_single_element() {
        check_multivalue_form("name=bob&nicknames=builder", vec!["builder"]);
    }

    #[test]
    fn test_multi_element_single_key() {
        check_multivalue_form("name=bob&nicknames=builder&nicknames=steve",
                              vec!["builder", "steve"]);
    }
}
