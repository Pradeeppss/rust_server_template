use crate::states::AppState;

mod models;
mod states;
mod handlers;
mod routes;
mod db;


#[tokio::main]
async fn main() {
    let pool = db::sqlite_db().await.expect("Failed to connect to db");
    let state = AppState::new(pool);
    let app = routes::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server running on port 3000"); 

    axum::serve(listener, app).await.unwrap();

}


// async fn root() -> &'static str {
//     "server is alive"
// }

// async fn health() -> &'static str {
//     "Ok"
// }

// async fn list_tasks() -> Json<Vec<Task>> {
//     Json(seed_tasks())
// }

// async fn create_task(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>){
//     let task= Task {
//         id: 20,
//         title: payload.title,
//         done: false
//     };

//     (StatusCode::CREATED, Json(task))
// }

// async fn get_task(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
//     let tasks = seed_tasks();
//     tasks
//         .into_iter()
//         .find(|t| t.id == id)
//         .map(Json)
//         .ok_or(StatusCode::NOT_FOUND)
// }