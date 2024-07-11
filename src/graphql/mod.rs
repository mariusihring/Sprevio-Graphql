use async_graphql::*;
mod auth;
mod expenses;
mod images;
mod provider;

use crate::graphql::auth::signup::SignUpMutation;
use crate::graphql::images::ImageMutation;
use crate::graphql::provider::mutation::ProviderMutation;
use crate::graphql::provider::query::ProviderQuery;
use expenses::query::ExpensesQuery;

#[derive(MergedObject, Default)]
pub struct Query(ExpensesQuery, ProviderQuery);

#[derive(MergedSubscription, Default)]
pub struct Subscription();

#[derive(MergedObject, Default)]
pub struct Mutation(ProviderMutation, ImageMutation, SignUpMutation);
