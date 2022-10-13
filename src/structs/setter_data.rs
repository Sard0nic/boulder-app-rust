

pub struct SetterData {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub username: String
}

impl SetterData {
    pub fn new(
        id: String,
        name: String,
        avatar: String,
    ) -> SetterData {
        let username = get_username(name);
        SetterData { id, name, avatar, username }
    }
}

fn get_username(name: String) -> String {
    name
}