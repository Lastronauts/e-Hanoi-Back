use crate::{
    auth::new_session,
    db::{
        scores,
        users,
    },
    schemas::{
        root::{
            Mutation,
            Query,
        },
        score::{
            NewScore,
            Score,
        },
        user::{
            NewUser,
            User,
        },
        Context,
    },
};
use juniper::{
    graphql_object,
    FieldResult,
};

#[graphql_object(context = Context)]
impl Query {
    async fn get_user(context: &Context) -> FieldResult<User> {
        let session = new_session(&context.credentials, context.token.clone())?;
        let id = session.user_id;
        let user = users::Repository::find_by_id(&context.pool, &id)?;

        context
            .loaders
            .users
            .prime(user.id.clone(), user.clone())
            .await;

        Ok(user.into())
    }

    #[graphql(arguments(start(default = 0), range(default = 20)))]
    async fn list_user(context: &Context, name: String, start: i32, range: i32) -> FieldResult<Vec<User>> {
        let start: usize = start.try_into()?;
        let range: usize = range.try_into()?;
        let end = start + range;

        let users = users::Repository::find_by_name(&context.pool, &name)?;

        for user in &users {
            context
                .loaders
                .users
                .prime(user.id.clone(), user.clone())
                .await;
        }

        let users = match users.len() {
            n if n > end => users[start..end].to_vec(),
            n if n > start => users[start..].to_vec(),
            _ => Vec::new(),
        };

        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    async fn get_my_best_score(context: &Context) -> FieldResult<Score> {
        let session = new_session(&context.credentials, context.token.clone())?;
        let id = session.user_id;
        let scores = scores::Repository::find_user_scores(&context.pool, &id)?;

        context.loaders.user_scores.prime(id, scores.clone()).await;

        Ok(scores.first().unwrap().to_owned().into())
    }

    #[graphql(arguments(start(default = 0), range(default = 20)))]
    async fn ranking_score(context: &Context, start: i32, range: i32) -> FieldResult<Vec<Score>> {
        let start: usize = start.try_into()?;
        let range: usize = range.try_into()?;
        let end = start + range;

        let scores = scores::Repository::all(&context.pool)?;

        let mut scores_cache = Vec::new();
        for score in &scores {
            scores_cache.push((
                score.user_id.clone(),
                scores
                    .iter()
                    .filter(|s| s.user_id == score.user_id)
                    .map(|s| s.to_owned())
                    .collect(),
            ));
        }
        context.loaders.user_scores.prime_many(scores_cache).await;

        let scores = match scores.len() {
            n if n > end => scores[start..end].to_vec(),
            n if n > start => scores[start..].to_vec(),
            _ => Vec::new(),
        };

        Ok(scores.into_iter().map(|u| u.into()).collect())
    }
}

#[graphql_object(context = Context)]
impl Mutation {
    #[graphql(name = "createUserInDB")]
    fn create_user_on_db(context: &Context, new_user: NewUser) -> FieldResult<User> {
        let session = new_session(&context.credentials, context.token.clone())?;
        let created_user = users::Repository::insert(&context.pool, new_user.into_form(&session))?;

        Ok(created_user.into())
    }

    #[graphql(name = "deleteUserInDB")]
    fn delete_user_on_db(context: &Context) -> FieldResult<User> {
        let session = new_session(&context.credentials, context.token.clone())?;
        let deleted_user = users::Repository::delete(&context.pool, &session.user_id)?;

        Ok(deleted_user.into())
    }

    fn create_score(context: &Context, new_score: NewScore) -> FieldResult<Score> {
        let session = new_session(&context.credentials, context.token.clone())?;
        let created_score = scores::Repository::insert(&context.pool, new_score.into_form(&session))?;

        Ok(created_score.into())
    }

    fn delete_score(context: &Context, id: i32) -> FieldResult<Score> {
        let deleted_score = scores::Repository::delete(&context.pool, id)?;

        Ok(deleted_score.into())
    }
}
