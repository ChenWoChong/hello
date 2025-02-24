use askama::Template;
use axum::{
    extract::{rejection::JsonRejection, Form, Json, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=114.116.219.50 user=test port=31432 dbname=todolist password=Test_123",
        NoTls,
    )
    .unwrap();

    let pool = Pool::builder().build(manager).await.unwrap();

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    let app = Router::new()
        .route("/", get(handler))
        .route("/hello", get(hello))
        .route("/query", get(query))
        .route("/greet/{name}", get(greet))
        .route("/qurey_from_db", get(qurey_from_db))
        .route("/default_json", post(get_json))
        .route("/form", post(accept_form))
        .route("/json", post(accept_json))
        .route("/user", post(create_user))
        .route("/todos", get(todos_list))
        .route("/todo", post(todo_create))
        .route("/todo/{id}", delete(todo_delete))
        .route("/todo", put(todo_update))
        .nest_service("/assets2", serve_dir)
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(TraceLayer::new_for_http())
        .fallback(hello)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
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

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    HelloTemplate { name: name }.to_string()
}

async fn qurey_from_db(State(pool): State<ConnectionPool>) -> Result<String, (StatusCode, String)> {
    tracing::debug!("get db conn pool {:?}", pool);

    let conn = pool.get().await.map_err(internal_error)?;

    tracing::debug!("query_from_db: 1");
    let row = conn
        .query_one("select 1+1", &[])
        .await
        .map_err(internal_error)?;

    tracing::debug!("query_from_db: 2");
    let two: i32 = row.try_get(0).map_err(internal_error)?;

    tracing::debug!("query_from_db: 3");
    tracing::debug!("qurey result: {:?}", two);

    Ok(two.to_string())
}

#[allow(dead_code)]
fn internal_error<E>(e: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

#[derive(Debug, Serialize)]
struct Todo {
    id: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

async fn todos_list(
    State(pool): State<ConnectionPool>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let offset = pagination.offset.unwrap_or(0);
    let limit = pagination.limit.unwrap_or(100);

    let rows = conn
        .query("select * from todo offset $1 limit $2", &[&offset, &limit])
        .await
        .map_err(internal_error)?;

    let mut todos = Vec::<Todo>::new();
    for row in rows {
        let id = row.get(0);
        let description = row.get(1);
        let completed = row.get(2);
        let todo = Todo {
            id,
            description,
            completed,
        };
        todos.push(todo);
    }
    Ok(Json(todos))
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub description: String,
}
async fn todo_create(
    State(pool): State<ConnectionPool>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let todo = Todo {
        id: Uuid::new_v4().simple().to_string(),
        description: input.description,
        completed: false,
    };

    let conn = pool.get().await.map_err(internal_error)?;
    let _ret = conn
        .execute(
            "insert into todo (id, description, completed) values ($1, $2, $3)",
            &[&todo.id, &todo.description, &todo.completed],
        )
        .await
        .map_err(internal_error)?;
    Ok((StatusCode::CREATED, Json(todo)))
}
async fn todo_delete(
    State(pool): State<ConnectionPool>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let _ret = conn
        .execute("delete from todo where id=$1", &[&id])
        .await
        .map_err(internal_error)?;
    Ok((StatusCode::OK, Json(id)))
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub id: String,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
async fn todo_update(
    State(pool): State<ConnectionPool>,
    Json(input): Json<UpdateTodo>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let id = input.id.clone();
    let description = input.description.unwrap_or("".to_string());
    let completed = input.completed.unwrap_or(false);
    tracing::debug!(
        "id, description, completed is:  {}, {}, {}",
        id,
        description,
        completed
    );

    let _ret = conn
        .execute(
            "update todo set description=$1, completed=$2 where id=$3",
            &[&description, &completed, &id],
        )
        .await
        .map_err(internal_error)?;
    Ok((StatusCode::OK, Json(input.id)))
}
