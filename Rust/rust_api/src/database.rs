pub struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn get_connection_pool(config: &Configurations) -> AppState {
    let url = get_database_url(config);
    let manager = ConnectionManager::<PgConnection>::new(url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    AppState { pool }
}
