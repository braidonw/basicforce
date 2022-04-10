use serde::Deserialize;
pub struct User {
    id: String,
    name: String,
    full_name: String,
    email: String,
}

pub struct Client {
    session_id: String,
    user: User,
    client_id: String,
    api_version: String,
    base_url: String,
    instance_url: String,
    use_tooling_api: bool,
}

impl Client {
    pub fn new(client_id: String) -> Client {
        todo!()
    }
}

#[derive(Deserialize)]
pub struct QueryRespone<T> {
    total_size: u32,
    done: bool,
    next_records_url: String,
    pub records: Vec<T>,
}
