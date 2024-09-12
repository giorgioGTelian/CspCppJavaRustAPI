pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/todo/:todo_id", get(get_todo))
        .route("/todo/:todo_id", delete(delete_todo))
        .route("/todo/:todo_id", put(complete_todo))
        .route("/todo", post(create_todo))
        .route("/todo", get(get_all_todos))
        .route("/todo/random", post(create_random_todo))
        .with_state(Arc::new(state))
        .layer(TraceLayer::new_for_http())
}

#[instrument]
async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let mut conn = state.pool.get().map_err(internal_error)?;

    info!("Creating Todo {:?}", &new_todo);

    let res = diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}

#[instrument]
async fn create_random_todo(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    let random_activity: Activity = reqwest::get("https://www.boredapi.com/api/activity")
        .await
        .map_err(internal_error)?
        .json()
        .await
        .map_err(internal_error)?;

    info!("Got: {:?}", random_activity);

    let new_todo = NewTodo {
        title: random_activity.activity,
        body: random_activity.activity_type,
    };

    let mut conn = state.pool.get().map_err(internal_error)?;

    info!("Creating random Todo {:?}", &new_todo);

    let res = diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(res))
}
