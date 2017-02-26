#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::vec;

use rocket::request::Form;

#[derive(FromForm)]
struct UserInfo {
    name: String,
    nicknames: Vec<String>,
    lucky_numbers: Vec<i32>,
}

#[post("/", data = "<form_data>")]
fn bug(form_data: Form<UserInfo>) -> String {
    let inner = form_data.into_inner();
    let mut output_string = format!("name {}", inner.name.to_owned());

    output_string = format!("{}\nnicknames ", output_string);
    debug!("form_data.nicknames: {:?}", inner.nicknames.len());
    for nname in &inner.nicknames {
        debug!("\t{}", nname);
        output_string = format!("{}{},", output_string, nname);
    }
    output_string = output_string[..(output_string.len()-1)].to_string();

    output_string = format!("{}\nlucky_numbers ", output_string);
    debug!("form_data.lucky_numbers: {:?}", inner.lucky_numbers.len());
    for ln in &inner.lucky_numbers {
        debug!("\t{}", ln);
        output_string = format!("{}{},", output_string, ln);
    }
    output_string = output_string[..(output_string.len()-1)].to_string();

    debug!("==>{}|", output_string);
    output_string
}

#[cfg(feature = "testing")]
mod tests {
    use super::*;
    use rocket::testing::MockRequest;
    use rocket::http::Method::*;
    use rocket::http::ContentType;
    use rocket::http::Status;

    fn check_multivalue_form(raw: &str, name: &str, nicknames: Vec<String>,
                             lucky_numbers: Vec<i32>) {
        let rocket = rocket::ignite().mount("/", routes![bug]);
        let mut req = MockRequest::new(Post, "/")
            .header(ContentType::Form)
            .body(format!("{}", raw));

        let mut response = req.dispatch_with(&rocket);
        let body_string = response.body().and_then(|b| b.into_string());
        assert_eq!(response.status(), Status::Ok);

        match body_string {
            None => assert!(false),
            Some(bs) => {
                debug!("body string: |{}|", bs);
                for line in bs.split("\n"){
                    let mut parts: Vec<&str> = Vec::new();
                    parts.extend(line.split(" "));
                    match parts[0] {
                        "name" => assert_eq!(name, parts[1]),
                        "nicknames" => {
                            if parts.len() > 1 {
                                let mut nnames: Vec<String> =
                                    parts[1].split(",").map(|x| x.to_string()).collect();
                                assert_eq!(nnames, nicknames);
                            }
                        },
                        "lucky_numbers" => {
                            if parts.len() > 1 {
                                let mut lns: Vec<i32> =
                                    parts[1].split(",").map(|x| x.parse::<i32>().unwrap_or(0)
                                    ).collect();
                                assert_eq!(lns, lucky_numbers);
                            }
                        }
                        _ => assert!(false), //we shouldn't get anything unexpected
                    }
                }
            }
        };
    }

    #[test]
    fn test_single_element() {
        check_multivalue_form("name=bob&nicknames=builder", "bob",
                              vec!["builder".to_string()], vec![]);
    }

    #[test]
    fn test_multi_element_single_key() {
        check_multivalue_form("name=bob&nicknames=builder&nicknames=steve", "bob",
                              vec!["builder".to_string(), "steve".to_string()],
                              vec![]);
    }

    #[test]
    fn test_multi_element_multi_key() {
        check_multivalue_form(
            "name=bob&nicknames=builder&nicknames=steve&nicknames=alfred&lucky_numbers=7&\
                lucky_numbers=13&lucky_numbers=42",
            "bob",
            vec!["builder".to_string(), "steve".to_string(), "alfred".to_string()],
            vec![7, 13, 42]
        )
    }
}
