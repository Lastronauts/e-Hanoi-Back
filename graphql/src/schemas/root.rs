use crate::schemas::Context;
use juniper::{
    EmptySubscription,
    RootNode,
};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub struct Query;

pub struct Mutation;
