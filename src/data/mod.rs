use std::{fmt::Display, error::Error};

use rocket::serde::{Serialize, Deserialize};

/// An abstract represention of a data entity uniquely identifiable by a primary key.
pub trait Entity {
    /// The type of the primary key for this entity.
    type PrimaryKey: Serialize + for<'a> Deserialize<'a> + Eq + Display;

    /// Gets the primary key of this entity.
    fn get_pk(&self) -> Self::PrimaryKey;

    /// The default primary key for when none is explicitly assigned.
    const PK_DEFAULT: Self::PrimaryKey;
}

/// A backing store-agnostic representation of a data repository for a given entity type.
#[async_trait]
pub trait Repository {
    /// The type of entity this reposititory shall work with.
    type EntityType: Entity;

    /// The type of error to return when a repository operation fails.
    type ErrType: Error;

    /// The type of input object to use for entity creation and modification.
    type EntityInputType: Into<Self::EntityType>;

    /// Retrieve all entities available in this repository.
    async fn get_many() -> Result<Vec<Self::EntityType>, Self::ErrType>;

    /// Attempt to retrieve a single entity by its primary key.
    async fn get_by_pk<PK>(pk: PK) -> Result<Self::EntityType, Self::ErrType>
        where Self::EntityType: Entity<PrimaryKey = PK>;

    /// Creates a new entity with the given input data.
    async fn create(input: Self::EntityInputType) -> Result<Self::EntityType, Self::ErrType>;

    /// Updates the entity with the given primary key with the given input data.
    async fn update<PK>(pk: PK, updated_input: Self::EntityInputType) -> Result<(), Self::ErrType>
        where Self::EntityType: Entity<PrimaryKey = PK>;

    /// Attempts to remove the entity with the given primary key.
    async fn delete<PK>(pk: PK) -> Result<(), Self::ErrType>
        where Self::EntityType: Entity<PrimaryKey = PK>;
}