use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use orientdb_client::common::types::error::OrientError;
use orientdb_client::common::types::OResult;
use orientdb_client::types::value::OValue;
use orientdb_client::OrientDB;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Customer {
    firstname: String,
    lastname: String,
    email: String,
}

// fn get_option_value(value: Option<&OValue>) -> &OValue {
//     let Some(val) = value;
//     val
// }

fn map_results_to_customers(results: Vec<Result<OResult, OrientError>>) -> Vec<Customer> {
    let customers: Vec<Customer> = results
    .into_iter()
    .filter_map(Result::ok)
    .map(|res| {
        // let firstname = get_option_value(res.get_raw("firstname"));
        // let dsds = res.get_raw();
        // let mut map = res.into().unwrap().into_map().unwrap();
        // let firstname = map.remove("firstname").unwrap().into_string().unwrap();
        // let lastname = map.remove("lastname").unwrap().into_string().unwrap();
        // let email = map.remove("email").unwrap().into_string().unwrap();
        let firstname = String::from("keanu");
        let lastname = String::from("reeves");
        let email = String::from("keanu@reeves.com");
        Customer { firstname, lastname, email }
    })
    .collect();
    customers
}

async fn print_api(req: HttpRequest) -> impl Responder {
    let client = OrientDB::connect(("localhost", 2424)).unwrap();
    let session = client.session("CustomerDb", "root", "rootpwd").unwrap();
    let results: Vec<Result<OResult, OrientError>> = session
        .query("select from Customers")
        .run()
        .unwrap()
        .collect();

    let customers = map_results_to_customers(results);

    println!("{:?}", customers);
    // println!("{:?}", print_type_of(&results));

    // for item in results.iter() {
    //     let fName = item.as_ref().unwrap().get_raw("firstname");
    //     let Some(val) = fName;

    //     // println!("{:?}", fName);

    //     // .unwrap().get_raw("firstname")
    // }

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
