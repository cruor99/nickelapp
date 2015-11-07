extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, Request, Response, MiddlewareResult,
HttpRouter, StaticFilesHandler, Mountable, JsonBody};

#[derive(RustcDecodable, RustcEncodable)]
struct Person{
    firstname: String,
    lastname: String,
}

fn templ_handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();

    data.insert("name", "Cruor99");
    data.insert("page_title", "Fickle nickel!");
    res.render("app/views/index.tpl", &data)
}


fn main() {
    let mut server = Nickel::new();

    server.get("/bar", middleware!("This is the /bar handler"));
    server.get("/user/:userid", middleware!{ |request|
        format!("This is user: {:?}", request.param("userid"))
    });
    server.get("/a/*/d", middleware!("matches /a/b/d but not /a/b/c/d"));
    server.get("/a/**/d", middleware!("matches /a/b/d and also /a/b/c/d"));

    server.mount("/static/files/", StaticFilesHandler::new("src/assets/"));

    server.post("/a/post/request", middleware!{ |request, response|
        let person = request.json_as::<Person>().unwrap();
        format!("Hello {} {}", person.firstname, person.lastname)
    });

    server.get("/", templ_handler);



    server.listen("localhost:8080");
}
