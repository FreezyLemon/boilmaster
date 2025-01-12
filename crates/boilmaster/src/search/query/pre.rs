use ironworks::excel;

use super::query;

// Re-export the query types, with assigned generics.
pub type Node = query::Node<LeafField, RelationTarget>;
pub type Group = query::Group<LeafField, RelationTarget>;
pub type Leaf = query::Leaf<LeafField, RelationTarget>;
pub type Operation = query::Operation<LeafField, RelationTarget>;
pub type Relation = query::Relation<LeafField, RelationTarget>;

pub use query::{Number, Occur, Value};

// Types specific to pre-normalised queries
pub type LeafField = Option<FieldSpecifier>;
pub type RelationTarget = ();

#[derive(Debug, Clone, PartialEq)]
pub enum FieldSpecifier {
	Struct(String, Option<excel::Language>),
	Array(Option<u32>),
}
