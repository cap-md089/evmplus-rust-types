pub mod account;
pub mod member;

pub trait Identifiable {
	type Identifier: PartialEq;

	fn id(&self) -> Self::Identifier;
}
