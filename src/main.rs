extern crate rustc_serialize;
#[macro_use] extern crate nickel;
extern crate postgres;

use std::collections::HashMap;
use nickel::{Nickel, Request, Response, MiddlewareResult,
HttpRouter, StaticFilesHandler, Mountable, JsonBody};
use postgres::{Connection, SslMode};
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable)]
struct Person{
    firstname: String,
    lastname: String,
}

struct Blog{
    id: i32,
    content: String,
    author: String,
    datepost: String,
}

fn save_db<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a>{

    let conn = Connection::connect("postgres://cruor:nickelblog@localhost/nickelblog", &SslMode::None).unwrap();

    let blog = Blog {
        id: 0,
        content: "My blog post!".to_string(),
        author: "ME!".to_string(),
        datepost: "".to_string(),
    };

    conn.execute("INSERT INTO blogs (content, author) VALUES($1, $2)",
    &[&blog.content, &blog.author]).unwrap();


    let mut data = HashMap::<&str, String>::new();
    data.insert("content", blog.content);
    data.insert("author", blog.author);
    data.insert("page_title", "Save blog data".to_string());
    res.render("app/views/save.tpl", &data)
}


fn get_db<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a>{

    let conn = Connection::connect("postgres://cruor:nickelblog@localhost/nickelblog", &SslMode::None).unwrap();

    let stmt = conn.prepare("SELECT id, content, author, datepost FROM blogs WHERE id = $1").unwrap();

    let id: Option<i32> = req.param("blogid").and_then(|x| x.trim().parse::<i32>().ok());

    let rows = match stmt.query(&[&id]) {
        Ok(rows) => rows,
        Err(err) => panic!("Error running query: {:?}", err)
    };

    let mut blog = Blog {
        id: 0,
        content: "".to_string(),
        author: "".to_string(),
        datepost: "".to_string()
    };

    for row in &rows {
        blog = Blog {
            id: row.get(0),
            content: row.get(1),
            author: row.get(2),
            datepost: row.get(3)
        };
        break;
    }

    let mut data = HashMap::<&str, String>::new();
    data.insert("content", blog.content);
    data.insert("author", blog.author);
    data.insert("datepost", blog.datepost);
    data.insert("page_title", "Showing blog data".to_string());
    res.render("app/views/show.tpl", &data)
}

fn create_post<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a>{
    let mut data = HashMap::<&str, &str>::new();


    data.insert("name", "Cruor99");
    data.insert("page_title", "Fickle nickel!");
    res.render("app/views/index.tpl", &data)

}

fn templ_handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();


    data.insert("name", "Cruor99");
    data.insert("page_title", "Fickle nickel!");
    res.render("app/views/index.tpl", &data)
}


fn main() {
    let mut server = Nickel::new();

    server.utilize(middleware!{ |request|
        println!("Accessing: {:?}", request.origin.uri);
    });

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
    server.get("/save", save_db);
    server.get("/show/:blogid", get_db);

    server.get("/createpost", middleware!{ |_, res|
        let mut data = HashMap::new();
        data.insert("title", "Create blog post");

        return res.render("app/views/createpost.html", &data)
    });

    server.post("/confirmation", middleware!{|req, res|
        let mut form_data = String::new();
        req.origin.read_to_string(&mut form_data).unwrap();

        println!("{}", form_data);

        let mut data = HashMap::new();
        data.insert("title", "confirmation");
        data.insert("formData", &form_data);

        return res.render("app/views/confirmation.html", &data)
    });



    server.listen("localhost:8080");
}
