use enum_iterator::{all, Sequence};
use mongodb::bson::{doc, Bson, Document};

/// Supported binary comparison operators. This type provides GraphQL names, MongoDB operator
/// names, and aggregation pipeline code for each operator. Argument types are defined in
/// mongodb-agent-common/src/scalar_types_capabilities.rs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Sequence)]
pub enum ComparisonFunction {
    // Equality and inequality operators (except for `NotEqual`) are built into the v2 spec, but
    // the only built-in operator in v3 is `Equal`. So we need at minimum definitions for
    // inequality operators here.
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,

    Regex,
    /// case-insensitive regex
    IRegex,
}

use ndc_query_plan::QueryPlanError;
use ComparisonFunction as C;

impl ComparisonFunction {
    pub fn graphql_name(self) -> &'static str {
        match self {
            C::LessThan => "_lt",
            C::LessThanOrEqual => "_lte",
            C::GreaterThan => "_gt",
            C::GreaterThanOrEqual => "_gte",
            C::Equal => "_eq",
            C::NotEqual => "_neq",
            C::Regex => "_regex",
            C::IRegex => "_iregex",
        }
    }

    pub fn mongodb_name(self) -> &'static str {
        match self {
            C::LessThan => "$lt",
            C::LessThanOrEqual => "$lte",
            C::GreaterThan => "$gt",
            C::GreaterThanOrEqual => "$gte",
            C::Equal => "$eq",
            C::NotEqual => "$ne",
            C::Regex => "$regex",
            C::IRegex => "$regex",
        }
    }

    pub fn from_graphql_name(s: &str) -> Result<Self, QueryPlanError> {
        all::<ComparisonFunction>()
            .find(|variant| variant.graphql_name() == s)
            .ok_or(QueryPlanError::UnknownComparisonOperator(
                s.to_owned().into(),
            ))
    }

    /// Produce a MongoDB expression for use in a match query that applies this function to the given operands.
    pub fn mongodb_match_query(
        self,
        column_ref: impl Into<String>,
        comparison_value: Bson,
    ) -> Document {
        match self {
            C::IRegex => {
                doc! { column_ref: { self.mongodb_name(): comparison_value, "$options": "i" } }
            }
            _ => doc! { column_ref: { self.mongodb_name(): comparison_value } },
        }
    }

    /// Produce a MongoDB expression for use in an aggregation expression that applies this
    /// function to the given operands.
    pub fn mongodb_aggregation_expression(
        self,
        column_ref: impl Into<Bson>,
        comparison_value: impl Into<Bson>,
    ) -> Document {
        match self {
            C::Regex => {
                doc! { "$regexMatch": { "input": column_ref, "regex": comparison_value } }
            }
            C::IRegex => {
                doc! { "$regexMatch": { "input": column_ref, "regex": comparison_value, "options": "i" } }
            }
            _ => doc! { self.mongodb_name(): [column_ref, comparison_value] },
        }
    }
}
