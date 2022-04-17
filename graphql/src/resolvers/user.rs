use crate::schemas::{
    score::Score,
    user::User,
    Context,
};
use juniper::{
    graphql_object,
    FieldResult,
    ID,
};

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn scores(&self, context: &Context) -> FieldResult<Vec<Score>> {
        Ok(context
            .loaders
            .user_scores
            .load(self.id.clone())
            .await
            .into_iter()
            .map(|s| s.into())
            .collect())
    }
}
