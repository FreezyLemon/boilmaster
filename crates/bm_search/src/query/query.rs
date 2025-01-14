#[derive(Debug, Clone, PartialEq)]
pub enum Node<F, T> {
	Group(Group<F, T>),
	Leaf(Leaf<F, T>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group<F, T> {
	pub clauses: Vec<(Occur, Node<F, T>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Occur {
	Must,
	Should,
	MustNot,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Leaf<F, T> {
	/// Column offset this leaf targets.
	pub field: F,
	pub operation: Operation<F, T>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation<F, T> {
	Relation(Relation<F, T>),

	Match(String),

	Eq(Value),

	Gt(Number),
	Gte(Number),
	Lt(Number),
	Lte(Number),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Relation<F, T> {
	pub target: T,
	/// Query to be executed on the target sheet's index.
	pub query: Box<Node<F, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Boolean(bool),
	Number(Number),
	String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
	/// A positive integer.
	U64(u64),
	/// A negative integer.
	I64(i64),
	/// A floating point number.
	F64(f64),
}
