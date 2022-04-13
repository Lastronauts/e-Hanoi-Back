use crate::schemas::{
    user::User,
    Context,
};
use juniper::{
    graphql_object,
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
}
