//RESPONSE types
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub status: i32,
    pub result: T,
    pub error: String,
}

