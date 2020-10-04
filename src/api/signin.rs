use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecaptchaToken {
	recaptcha_token: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureToken {
	nonce: String,

	signature: String,

	#[serde(rename = "signatureID")]
	signature_id: String
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SigninToken {
	Recaptcha(RecaptchaToken),
	Signature(SignatureToken)
}

impl SigninToken {
	pub fn new_signature(
		nonce: String,
		signature: String,
		signature_id: String
	) -> Self {
		Self::Signature(SignatureToken {
			nonce,
			signature,
			signature_id
		})
	}
}

#[derive(Serialize, Deserialize)]
pub struct SigninRequest {
	username: String,
	password: String,
	token: SigninToken
}

impl SigninRequest {
	pub fn new(username: String, password: String, token: SigninToken) -> Self {
		Self {
			username,
			password,
			token
		}
	}
}
