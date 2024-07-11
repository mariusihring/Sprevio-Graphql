use async_graphql::*;
mod expenses;
mod provider;
mod images;
mod auth;

use expenses::query::ExpensesQuery;
use crate::graphql::images::ImageMutation;
use crate::graphql::provider::mutation::ProviderMutation;
use crate::graphql::provider::query::ProviderQuery;

#[derive(MergedObject, Default)]
pub struct Query(ExpensesQuery, ProviderQuery);

#[derive(MergedSubscription, Default)]
pub struct Subscription();

#[derive(MergedObject, Default)]
pub struct Mutation(ProviderMutation, ImageMutation);