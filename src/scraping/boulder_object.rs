use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BoulderObject {
    pub createdAt: String,
    pub createdBy: String,
    pub content: BoulderContent,
    pub id: String,
    pub r#type: String,
    pub revisionId: i32
}

#[derive(Debug, Deserialize)]
pub struct BoulderContent {
    pub isDraft: i32,
    pub removed: i32,
    pub setDate: i64,
    pub setter: Vec<String>,
    pub sector: String,
    pub name: String,
    pub grade: String,
    pub gradeNr: i32
}