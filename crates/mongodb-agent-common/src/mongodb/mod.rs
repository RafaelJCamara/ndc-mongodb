mod accumulator;
mod collection;
mod database;
mod pipeline;
mod projection;
pub mod sanitize;
mod selection;
mod stage;

#[cfg(test)]
pub mod test_helpers;

pub use self::{
    accumulator::Accumulator,
    collection::CollectionTrait,
    database::DatabaseTrait,
    pipeline::Pipeline,
    projection::{ProjectAs, Projection},
    selection::Selection,
    stage::Stage,
};

// MockCollectionTrait is generated by automock when the test flag is active.
#[cfg(test)]
pub use self::collection::MockCollectionTrait;

// MockDatabase is generated by automock when the test flag is active.
#[cfg(test)]
pub use self::database::MockDatabaseTrait;
