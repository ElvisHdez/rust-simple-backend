use std::collections::HashMap;

use axum::{body::Body, extract::{MatchedPath, Query}, http::{StatusCode, Uri}, response::Redirect, routing::get, Router};
use axum_template::{engine::Engine, Key, TemplateEngine};
use tokio::{self, net::TcpListener};


type AppEngine = Engine<handlebars::Handlebars<'static>>;
#[derive(Clone)]
struct AppEngineState{
    pub engine:AppEngine
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/msg", get(get_msge))
        .nest_service("/",tower_http::services::ServeDir::new("./pages"));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();


    axum::serve(listener,app).await.unwrap();
}
async fn get_msge(Query(msge): Query<HashMap<String,String>>)->Redirect{
    println!("{:?}",msge.get("msg").unwrap().clone());
    Redirect::to("/index.html")
}