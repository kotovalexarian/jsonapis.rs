mod data;
mod document;
mod error_object;
mod error_source;
mod jsonapi;
mod link;
mod links;
mod meta_or_attrs;
mod relationship;
mod relationships;
mod resource;

pub use data::DataBuilder;
pub use document::DocumentBuilder;
pub use error_object::ErrorObjectBuilder;
pub use error_source::ErrorSourceBuilder;
pub use jsonapi::JsonApiBuilder;
pub use link::LinkBuilder;
pub use links::LinksBuilder;
pub use meta_or_attrs::MetaOrAttrsBuilder;
pub use relationship::RelationshipBuilder;
pub use relationships::RelationshipsBuilder;
pub use resource::ResourceBuilder;

use super::entities::*;

use std::collections::HashMap;
use std::fmt::Debug;

use serde_json::Value;

#[derive(Debug)]
pub struct BuildErrors;

pub trait Builder<'de>: Clone + Debug + Eq + PartialEq + Sized {
    type Entity: Entity<'de>;

    fn finish(self) -> Result<Self::Entity, BuildErrors>;

    fn expect(self, msg: &str) -> Self::Entity {
        self.finish().expect(msg)
    }

    fn expect_err(self, msg: &str) -> BuildErrors {
        self.finish().expect_err(msg)
    }

    fn unwrap(self) -> Self::Entity {
        self.finish().unwrap()
    }

    fn unwrap_err(self) -> BuildErrors {
        self.finish().unwrap_err()
    }
}
