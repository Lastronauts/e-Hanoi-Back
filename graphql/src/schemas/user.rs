use crate::db::users;
use firestore_db_and_auth::UserSession;
use juniper::GraphQLInputObject;

pub struct User {
    pub id: String,
    pub name: String,
}

impl From<users::User> for User {
    fn from(user: users::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    pub fn into_form(self, session: &UserSession) -> users::UserNewForm {
        users::UserNewForm {
            id: session.user_id.clone(),
            name: self.name,
        }
    }
}
