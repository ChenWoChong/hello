use axum::{
    extract::{rejection::JsonRejection, Form, Json, Query},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    let app = Router::new()
        .route("/", get(handler))
        .route("/hello", get(hello))
        .route("/query", get(query))
        .route("/default_json", post(get_json))
        .route("/form", post(accept_form))
        .route("/json", post(accept_json))
        .route("/user", post(create_user))
        .nest_service("/assets2", serve_dir)
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(TraceLayer::new_for_http())
        .fallback(hello);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("http listen on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1> hello world</h1>")
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct InputParams {
    foo: i32,
    bar: String,
    third: Option<i32>,
}

async fn query(Query(params): Query<InputParams>) -> impl IntoResponse {
    tracing::debug!("query params {:?}", params);
    Html("<h3> Test query </h3>")
}

#[derive(Default, Deserialize, Debug, serde::Serialize)]
#[allow(dead_code)]
struct Input {
    name: String,
    age: i32,
    email: String,
}

async fn accept_form(Form(input): Form<Input>) -> impl IntoResponse {
    tracing::debug!("input content: {:?}", input);
    Html("<h3>Form post</h3>")
}

async fn accept_json(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("input json: {:?}", input);
    Html("<h3> Json post </h3>")
}

async fn create_user(payload: Result<Json<Input>, JsonRejection>) -> impl IntoResponse {
    match payload {
        Ok(Json(input)) => {
            tracing::debug!("create user: {:?}", input);
            Json(input).into_response()
        }
        Err(JsonRejection::MissingJsonContentType(e)) => {
            format!("MissingJsonContentType err! : {}", e).into_response()
        }
        Err(JsonRejection::JsonDataError(e)) => {
            format!("JsonDataError err! : {}", e).into_response()
        }
        _ => Redirect::permanent("/").into_response(),
    }
}

async fn hello() -> impl IntoResponse {
    "hello world!"
}

async fn get_json(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("get json: {:?}", input);
    Json(json!({ "result": "ok", "number": 1, }))
}
