use crate::db::scores;
use firestore_db_and_auth::UserSession;
use juniper::GraphQLInputObject;

pub struct Score {
    pub id: i32,
    pub user_id: String,
    pub clear_time: i32,
    pub created_at: i64,
}

impl From<scores::Score> for Score {
    fn from(score: scores::Score) -> Self {
        Self {
            id: score.id,
            user_id: score.user_id,
            clear_time: score.clear_time,
            created_at: score.created_at.timestamp(),
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct NewScore {
    clear_time: i32,
}

impl NewScore {
    pub fn into_form(self, session: &UserSession) -> scores::ScoreNewForm {
        scores::ScoreNewForm {
            user_id: session.user_id.clone(),
            clear_time: self.clear_time,
        }
    }
}
