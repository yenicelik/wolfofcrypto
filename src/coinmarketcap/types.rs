#[derive(Serialize, Deserialize, Debug)]
pub struct FloatRecord {
    pub unixtime: i64,
    pub floatfield: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntRecord {
    pub unixtime: i64,
    pub intfield: i64,
}
