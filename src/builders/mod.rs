mod data;
mod document;
mod jsonapi;
mod link;
mod links;
mod meta_or_attrs;
mod relationship;
mod relationships;
mod resource;

pub use data::DataBuilder;
pub use document::DocumentBuilder;
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

pub trait Builder: Clone + Debug + Eq + PartialEq + Sized {
    type Entity: Entity;

    fn finish(self) -> Result<Self::Entity, ()>;

    fn unwrap(self) -> Self::Entity {
        self.finish().unwrap()
    }
}
