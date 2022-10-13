use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SetterObject {
    pub name: String,
    pub avatar: String
}