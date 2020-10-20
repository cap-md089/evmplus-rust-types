use serde::{Deserialize, Serialize};

use crate::data::Identifiable;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbsenteeInformation {
	pub comments: String,

	pub absent_until: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CAPMemberContactInstance {
	#[serde(rename = "PRIMARY")]
	pub primary: Option<String>,

	#[serde(rename = "PRIMARY")]
	pub secondary: Option<String>,

	#[serde(rename = "PRIMARY")]
	pub emergency: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CAPMemberContact {
	#[serde(rename = "ALPHAPAGER")]
	pub alpha_pager: CAPMemberContactInstance,

	#[serde(rename = "ASSISTANT")]
	pub assistant: CAPMemberContactInstance,

	#[serde(rename = "CADETPARENTEMAIL")]
	pub cadet_parent_email: CAPMemberContactInstance,

	#[serde(rename = "CADETPARENTPHONE")]
	pub cadet_parent_phone: CAPMemberContactInstance,

	#[serde(rename = "CELLPHONE")]
	pub cell_phone: CAPMemberContactInstance,

	#[serde(rename = "DIGITALPAGER")]
	pub digital_pager: CAPMemberContactInstance,

	#[serde(rename = "EMAIL")]
	pub email: CAPMemberContactInstance,

	#[serde(rename = "HOMEFAX")]
	pub home_fax: CAPMemberContactInstance,

	#[serde(rename = "HOMEPHONE")]
	pub home_phone: CAPMemberContactInstance,

	#[serde(rename = "INSTANTMESSENGER")]
	pub instant_messanger: CAPMemberContactInstance,

	#[serde(rename = "ISDN")]
	pub isdn: CAPMemberContactInstance,

	#[serde(rename = "RADIO")]
	pub radio: CAPMemberContactInstance,

	#[serde(rename = "TELEX")]
	pub telex: CAPMemberContactInstance,

	#[serde(rename = "WORKFAX")]
	pub work_fax: CAPMemberContactInstance,

	#[serde(rename = "WORKPHONE")]
	pub work_phone: CAPMemberContactInstance
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewShortCAPUnitDutyPosition {
	pub duty: String,

	pub expires: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortCAPUnitDutyPosition {
	#[serde(flatten)]
	pub duty_information: NewShortCAPUnitDutyPosition,

	pub date: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortNHQDutyPosition {
	pub duty: String,

	pub date: u64,

	pub orgid: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ShortDutyPosition {
	CAPUnit(ShortCAPUnitDutyPosition),
	NHQ(ShortNHQDutyPosition)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum MemberReference {
	CAPNHQMember { id: u32 },
	CAPProspectiveMember { id: String }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CAPNHQMember {
	pub id: u32,

	pub expiration_date: u64,

	pub date_of_birth: u64,

	pub duty_positions: Vec<ShortDutyPosition>,

	pub squadron: String,

	pub flight: Option<String>,

	#[serde(rename = "teamIDs")]
	pub team_ids: Vec<u16>,

	pub member_rank: String,

	pub senior_member: bool,

	pub orgid: u16,

	pub contact: CAPMemberContact,

	#[serde(rename = "usrID")]
	pub user_id: String,

	pub name_first: String,

	pub name_middle: String,

	pub name_last: String,

	pub name_suffix: String,

	pub absentee_information: Option<AbsenteeInformation>
}

impl PartialEq for CAPNHQMember {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Identifiable for CAPNHQMember {
	type Identifier = MemberReference;

	fn id(&self) -> Self::Identifier {
		MemberReference::CAPNHQMember { id: self.id }
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewCAPProspectiveMember {
	pub flight: Option<String>,

	pub senior_member: bool,

	pub contact: CAPMemberContact,

	pub name_first: String,

	pub name_middle: String,

	pub name_last: String,

	pub name_suffix: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullCAPProspectiveMember {
	#[serde(flatten)]
	pub prospective_member_info: NewCAPProspectiveMember,

	pub id: String,

	pub absentee_information: Option<AbsenteeInformation>,

	#[serde(rename = "teamIDs")]
	pub team_ids: Vec<u16>,

	#[serde(rename = "usrID")]
	pub user_id: String,

	pub duty_positions: Vec<ShortDutyPosition>,

	#[serde(rename = "accountID")]
	pub account_id: String,

	pub member_rank: String,

	pub orgid: u16,

	pub squadron: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeCAPProspectiveMember {
	pub id: String,

	pub nhq_reference: MemberReference
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CAPProspectiveMember {
	Upgraded(UpgradeCAPProspectiveMember),
	Regular(FullCAPProspectiveMember)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Member {
	CAPNHQMember(CAPNHQMember),
	CAPProspectiveMember(CAPProspectiveMember)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CAPSquadronMemberPermissions {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CAPGroupMemberPermissions {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CAPWingmemberPermissions {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CAPRegionMemberPermissions {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CAPEventMemberPermissions {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MemberPermissions {
	CAPSquadron(CAPSquadronMemberPermissions),
	CAPGroup(CAPGroupMemberPermissions),
	CAPWing(CAPWingmemberPermissions),
	CAPRegion(CAPRegionMemberPermissions),
	CAPEvent(CAPEventMemberPermissions)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
	#[serde(flatten)]
	pub member: Member,

	pub permissions: MemberPermissions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountLinkTarget {
	pub id: String,

	pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "error")]
pub enum SigninReturn {
	None {
		member: User,

		#[serde(rename = "notificationCount")]
		notification_count: u16,

		#[serde(rename = "taskCount")]
		task_count: u16,

		#[serde(rename = "linkableAccounts")]
		linkable_accounts: Vec<AccountLinkTarget>
	},
	IncorrectCredentials,
	ServerError,
	PasswordExpired,
	InvalidSessionID,
	TokenInvalid,
	UnknownServerError,
	DatabaseError,
	MFAChallengeRequired
}
