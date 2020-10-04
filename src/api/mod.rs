use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HTTPError {
	pub code: u16,

	pub message: String
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "direction")]
pub enum Either<L, R> {
	#[serde(rename = "left")]
	Left(L),

	#[serde(rename = "right")]
	Right(R)
}

pub type APIEither<T> = Either<HTTPError, T>;
