use serde::{Deserialize, Serialize};

pub mod signin;

#[derive(Serialize, Deserialize, Debug)]
pub struct HTTPError {
	pub code: u16,

	pub message: String
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "direction")]
pub enum Either<L, R> {
	#[serde(rename = "left")]
	Left { value: L },

	#[serde(rename = "right")]
	Right { value: R }
}

pub type APIEither<T> = Either<HTTPError, T>;

impl<L, R> Into<Result<R, L>> for Either<L, R> {
	fn into(self) -> Result<R, L> {
		match self {
			Self::Left { value } => Err(value),
			Self::Right { value } => Ok(value)
		}
	}
}
