#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Room {
    pub imports: Vec<String>,
}