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
pub enum SigninToken {
	Recaptcha(RecaptchaToken),
	Signature(SignatureToken)
}

#[derive(Serialize, Deserialize)]
pub struct SigninRequest {
	username: String,
	password: String,
	token: SigninToken
}
