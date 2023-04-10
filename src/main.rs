use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use orientdb_client::common::types::error::OrientError;
use orientdb_client::common::types::OResult;
use orientdb_client::OrientDB;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Customer {
    rid: String,
    firstname: String,
    lastname: String,
    email: String,
}

async fn print_api(req: HttpRequest) -> impl Responder {
    let client = OrientDB::connect(("localhost", 2424)).unwrap();
    let session = client.session("CustomerDb", "root", "rootpwd").unwrap();
    let results: Vec<Result<OResult, OrientError>> = session
        .query("select @rid.asString() as id, firstname, lastname, email from Customers")
        .run()
        .unwrap()
        .collect();

    let customers: Vec<Customer> = results
        .into_iter()
        .filter_map(Result::ok)
        .map(|res| {
            Customer {
                rid: res.get("id"),
                firstname: res.get("firstname"),
                lastname: res.get("lastname"),
                email: res.get("email"),
            }
        })
        .collect();

    print!("{:?}", customers);

    HttpResponse::Ok()
}

async fn insert_api(req: HttpRequest) -> impl Responder {
    let client = OrientDB::connect(("localhost", 2424)).unwrap();
    let session = client.session("CustomerDb", "root", "rootpwd").unwrap();
    let results : Vec<_> = session
        .command("INSERT INTO Customers (email, firstname, lastname) VALUES (:email, :firstname, :lastname)")
        .named(&[("email", &"keanu@reeves.com"), ("firstname", &"keanu"), ("lastname", &"reeves")])
        .run()
        .unwrap()
        .collect();

    for item in results.iter() {
        println!("{:?}", item);
    }

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/print", web::get().to(print_api))
            .route("/insert", web::get().to(insert_api))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
