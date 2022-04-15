use crate::schemas::{
    score::Score,
    Context,
};
use juniper::{
    graphql_object,
    FieldError,
    FieldResult,
    ID,
};

#[graphql_object(context = Context)]
impl Score {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    #[graphql(name = "userID")]
    fn user_id(&self) -> String {
        self.user_id.clone()
    }

    fn clear_time(&self) -> i32 {
        self.clear_time
    }

    fn created_at(&self) -> FieldResult<i32> {
        self.created_at.try_into().map_err(FieldError::from)
    }
}
