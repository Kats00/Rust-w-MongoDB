use warp::{Filter, Rejection, Reply};
use serde::{Serialize, Deserialize};
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}};
use mongodb::sync::{Client, Collection};
use futures::future::{Future};
use env_logger::Env;

#[macro_use]
extern crate log;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    name: String,
    age: u8,
    email: String,
    characteristics: Vec<String>
}

#[derive(Debug, Deserialize, Clone)]
struct UserRequest {
    name: String,
    age: u8,
    email: String,
    characteristics: Vec<String>
}

fn create_user(
    entry: UserRequest,
    collection: Collection<User>,
) -> impl Future<Output = Result<impl Reply, Rejection>> {
    async move {
        debug!("Received JSON payload: {:?}", entry);

        let user = User {
            name: entry.name.clone(),
            age: entry.age.clone(),
            email: entry.email.clone(),
            characteristics: entry.characteristics.clone(),
        };

        debug!("User object: {:?}", user);

        tokio::task::spawn_blocking(move || {
            collection.insert_one(user, None).expect("insertion error");
        });

        debug!("User created successfully");

        Ok(warp::reply::with_status(
            "User created",
            warp::http::StatusCode::CREATED,
        ))
    }
}

fn with_collection(
    collection: Collection<User>,
) -> impl Filter<Extract = (Collection<User>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || collection.clone())
}

fn main() {
    // environment to debug
    std::env::set_var("RUST_LOG", "debug");

    // my debug logger
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init(); 

    //mongodb atlas link here
    let mut client_options =
        ClientOptions::parse("your-atlas-connection")
            .unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();

    let db = client.database("mydatabase");

    let collection = db.collection::<User>("mycollection");

    let create = warp::post()
    .and(warp::body::json())
    .and(with_collection(collection.clone()))
    .and_then(|entry: UserRequest, collection: Collection<User>| async move {
        create_user(entry, collection).await
    });


    let routes = create;

    println!("Server is running at http://localhost:3030");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    });
}

