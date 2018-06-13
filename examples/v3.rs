extern crate sendgrid;

use std::collections::HashMap;

use sendgrid::v3::*;

fn main() {
    let mut m = SGMailV3::new();

    let mut e = Email::new();
    e.set_email("g@gmail.com");

    let mut c = Content::new();
    c.set_content_type("text/html");
    c.set_value("Test");

    let mut p = Personalization::new();
    p.add_to(e.clone());

    let mut cool_header = HashMap::new();
    cool_header.insert(String::from("x-cool"), String::from("indeed"));
    cool_header.insert(String::from("x-cooler"), String::from("cold"));
    p.add_headers(cool_header);

    m.set_from(e.clone());
    m.set_subject("Subject");
    m.add_content(c);
    m.add_personalization(p);

    let mut env_vars = ::std::env::vars();
    let api_key = env_vars.find(|v| v.0 == "SG_API_KEY").unwrap();
    let sender = V3Sender::new(api_key.1);
    let code = sender.send(&m);
    println!("{:?}", code);
}
