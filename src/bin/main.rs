extern crate http_cursed;
use http_cursed::{
    *,
    todo,
    server::send::{
        send_file,
        send_http,
        send_empty_ok,
        send_contents
    },
    server::{
        Server,
        common::{
            EndPoint,
            ServerImpl
        }
    },
    http::*,
    common::{
        typedef::TcpStream
    }
};

use todo::{
    database::DataBase,
    mongodb::MongoDBDataBase,
    task::TaskStatus
};

use mongodb::bson::Document;

fn handle_404(stream : &TcpStream, _request : HTTPRequest) {
    send_file(stream.try_clone().unwrap(), "/404.html", "./root").unwrap_or(());
}

fn handle_bad_req(stream : &TcpStream) {
    send_http(
        stream.try_clone().unwrap(),
        &HTTPResponse {
            status: HTTPStatus {version: format!("1.1"), code: HTTPCode::BadReq },
            header: HTTPHeader::empty(),
            body: HTTPBody::empty()
        }
    ).unwrap_or(());
}

fn doc_from_bytes(contents : &Vec<u8>) -> Result<Document,()> {
    let body = unwrap_result_or_return!(
        std::str::from_utf8(contents.as_slice()),
        Err(())
    );

    let json_map : serde_json::Map<String, serde_json::Value> = unwrap_result_or_return!(
        serde_json::from_str(body),
        Err(())
    );

    Ok(unwrap_result_or_return!(
        Document::try_from(json_map),
        Err(())
    ))
}

fn handle_create_task(stream : &TcpStream, request : HTTPRequest) {
    let mut db_handler = MongoDBDataBase::new();

    let doc = match doc_from_bytes(&request.body.contents) {
        Ok(d) => d,
        Err(_) => {handle_bad_req(stream); return}
    };

    let title = match doc.get("title") {
        Some(t) => match t.as_str() {
            Some(t) => t,
            None => {handle_bad_req(stream); return}
        },
        None => {handle_bad_req(stream); return}
    };

    let description = match doc.get("description") {
        Some(d) => Some(match d.as_str() {
            Some(d) => d,
            None => {handle_bad_req(stream); return}
        }),
        None => None
    };

    db_handler.register_task(title, description).unwrap_or(());

    send_empty_ok(stream);
}

fn handle_update_task(stream : &TcpStream, request : HTTPRequest) {
    let mut db_handler = MongoDBDataBase::new();

    let doc = match doc_from_bytes(&request.body.contents) {
        Ok(d) => d,
        Err(_) => {handle_bad_req(stream); return}
    };

    let id = match doc.get("id") {
        Some(i) => match i.as_i64() {
            Some(i) => i,
            None => {handle_bad_req(stream); return}
        },
        None => {handle_bad_req(stream); return}
    };

    let status_number = match doc.get("status") {
        Some(s) => match s.as_i32() {
            Some(s) => s,
            None => {handle_bad_req(stream); return}
        },
        None => {handle_bad_req(stream); return}
    };

    let status = match TaskStatus::try_from(status_number) {
        Ok(s) => s,
        Err(_) => {handle_bad_req(stream); return}
    };

    db_handler.mark_task_as(id, status).unwrap_or(());

    send_empty_ok(stream);
}

fn join_docs_in_json(docs : &Vec<Document>) -> String {
    format!("[\n{}\n]", docs
        .iter()
        .map(|d| serde_json::to_string_pretty(d).unwrap()
            .split("\n")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| format!("  {s}"))
            .collect::<Vec<_>>()
            .join("\n")
        )
        .collect::<Vec<_>>()
        .join(",\n")
    )
}

fn handle_list_tasks(stream : &TcpStream, request : HTTPRequest) {
    let mut db_handler = MongoDBDataBase::new();

    let doc = match doc_from_bytes(&request.body.contents) {
        Ok(d) => d,
        Err(_) => {handle_bad_req(stream); return}
    };

    let title = match doc.get("title") {
        Some(t) => Some(match t.as_str() {
            Some(t) => t,
            None => {handle_bad_req(stream); return}
        }),
        None => None
    };

    let tasks = match
        match title {
            Some(title) => db_handler.get_tasks_by_title(title),
            None => db_handler.get_all_tasks()
        }
    {
        Ok(list) => list,
        Err(_) => return ()
    };

    let body = Vec::from(join_docs_in_json(
        &tasks
            .iter()
            .map(|t| MongoDBDataBase::format_task(t))
            .collect::<Vec<_>>()
    ).as_bytes());
    
    send_contents(stream.try_clone().unwrap(), body).unwrap_or(());
}

fn main2() {
    let mut server = Server::new();

    server.register_404(handle_404);
    server.register_bad_req(handle_bad_req);

    server.register(EndPoint{
        method: HTTPMethod::POST,
        route: String::from("/create"),
        handler: handle_create_task
    });
    server.register(EndPoint{
        method: HTTPMethod::POST,
        route: String::from("/update"),
        handler: handle_update_task
    });
    server.register(EndPoint{
        method: HTTPMethod::GET,
        route: String::from("/list"),
        handler: handle_list_tasks
    });
    
    server.bind(7878);
}

use mongodb::bson;
use mongodb::bson::RawBson;
use mongodb::bson::Bson;
use mongodb::bson::Array;

fn main() {
    let _d = bson::doc!["asdf": 2];
    println!("{:#?}", _d);

    let a = 3;

    let _d = bson::bson!(["asdf", 2, {"asf": 31}, a]);
    println!("{:#?}", _d);
}


/*******************/
/*  MONAD EXAMPLE  */
/*******************/
// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Foo<T> {
//     x : T
// }

// impl<T> Foo<T> {
//     fn new(x : T) -> Self {
//         Self {x}
//     }

//     fn map<U>(self, f : fn(T) -> Foo<U>) -> Foo<U> {
//         f(self.x)
//     }
// }

// fn foo(x : i32) -> Foo<String> {
//     Foo::new(x.to_string())
// }

// fn bar(x : String) -> Foo<usize> {
//     Foo::new(x.len())
// }

// pub fn main() {
//     let x : i32 = 123;
//     let y : Foo<i32> = Foo::new(234);
//     let z : Foo<i32> = Foo::new(345);

//     let s1 : Foo<String> = foo(x);
//     let s2 : Foo<String> = y.clone().map(foo);
//     let s3 : Foo<usize>  = z.clone().map(foo).map(bar);

//     println!("{}", Foo::new(5).map(foo) == foo(5));
//     println!("{}", s1.clone().map(Foo::new) == s1);
//     println!("{}", y.clone().map(|x| foo(x).map(bar)) == y.clone().map(foo).map(bar));

//     println!("{:?}", s1);
//     println!("{:?}", s2);
//     println!("{:?}", s3);
// }
