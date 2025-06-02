use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RespData {
    Vector(Option<Vec<Data>>),
    Item(Option<Data>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZohoApiResponse {
    pub status: Option<Status>,
    pub data: Option<RespData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub code: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub country: Option<String>,
    #[serde(rename = "lastLogin")]
    pub last_login: Option<i64>,
    #[serde(rename = "mxStatus")]
    pub mx_status: Option<bool>,
    #[serde(rename = "activeSyncEnabled")]
    pub active_sync_enabled: Option<bool>,
    #[serde(rename = "mobileNumber")]
    pub mobile_number: Option<String>,
    #[serde(rename = "isCustomAdmin")]
    pub is_custom_admin: Option<bool>,
    #[serde(rename = "incomingBlocked")]
    pub incoming_blocked: Option<bool>,
    pub language: Option<String>,
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    #[serde(rename = "extraStorage")]
    pub extra_storage: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "incomingUserName")]
    pub incoming_user_name: Option<String>,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<Vec<EmailAddress>>,
    #[serde(rename = "mailboxStatus")]
    pub mailbox_status: Option<String>,
    #[serde(rename = "ediscoveryStorage")]
    pub ediscovery_storage: Option<EdiscoveryStorage>,
    #[serde(rename = "popBlocked")]
    pub pop_blocked: Option<bool>,
    #[serde(rename = "usedStorage")]
    pub used_storage: Option<i64>,
    #[serde(rename = "spamcheckEnabled")]
    pub spamcheck_enabled: Option<bool>,
    #[serde(rename = "imapAccessEnabled")]
    pub imap_access_enabled: Option<bool>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "accountCreationTime")]
    pub account_creation_time: Option<i64>,
    pub zuid: Option<i64>,
    #[serde(rename = "webBlocked")]
    pub web_blocked: Option<bool>,
    #[serde(rename = "planStorage")]
    pub plan_storage: Option<i64>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    pub sequence: Option<i64>,
    #[serde(rename = "mailboxAddress")]
    pub mailbox_address: Option<String>,
    #[serde(rename = "lastPasswordReset")]
    pub last_password_reset: Option<i64>,
    #[serde(rename = "tfaEnabled")]
    pub tfa_enabled: Option<bool>,
    #[serde(rename = "iamStatus")]
    pub iam_status: Option<i32>,
    #[serde(rename = "phoneNumer")]
    pub phone_number: Option<String>,
    pub status: Option<bool>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "accountDisplayName")]
    pub account_display_name: Option<String>,
    pub role: Option<String>,
    pub gender: Option<String>,
    #[serde(rename = "accountName")]
    pub account_name: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<Vec<serde_json::Value>>,
    #[serde(rename = "isLogoExist")]
    pub is_logo_exist: Option<bool>,
    #[serde(rename = "primaryEmailAddress")]
    pub primary_email_address: Option<String>,
    pub enabled: Option<bool>,
    #[serde(rename = "mailboxCreationTime")]
    pub mailbox_creation_time: Option<i64>,
    #[serde(rename = "basicStorage")]
    pub basic_storage: Option<String>,
    #[serde(rename = "lastClient")]
    pub last_client: Option<String>,
    #[serde(rename = "allowedStorage")]
    pub allowed_storage: Option<i64>,
    #[serde(rename = "sendMailDetails")]
    pub send_mail_details: Option<Vec<SendMailDetail>>,
    #[serde(rename = "popFetchTime")]
    pub pop_fetch_time: Option<i64>,
    pub address: Option<Address>,
    #[serde(rename = "planType")]
    pub plan_type: Option<i64>,
    #[serde(rename = "userExpiry")]
    pub user_expiry: Option<i64>,
    #[serde(rename = "popAccessEnabled")]
    pub pop_access_enabled: Option<bool>,
    #[serde(rename = "imapBlocked")]
    pub imap_blocked: Option<bool>,
    #[serde(rename = "iamUserRole")]
    pub iam_user_role: Option<String>,
    #[serde(rename = "outgoingBlocked")]
    pub outgoing_blocked: Option<bool>,
    #[serde(rename = "policyId")]
    pub policy_id: Option<PolicyId>,
    #[serde(rename = "smtpStatus")]
    pub smtp_status: Option<bool>,
    #[serde(rename = "extraEDiscoveryStorage")]
    pub extra_ediscovery_storage: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailAddress {
    #[serde(rename = "isAlias")]
    pub is_alias: Option<bool>,
    #[serde(rename = "isPrimary")]
    pub is_primary: Option<bool>,
    #[serde(rename = "mailId")]
    pub mail_id: Option<String>,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EdiscoveryStorage {
    #[serde(rename = "iseDiscoveryStorageEnabled")]
    pub is_ediscovery_storage_enabled: Option<bool>,
    #[serde(rename = "ediscoveryUsedStorage")]
    pub ediscovery_used_storage: Option<i64>,
    #[serde(rename = "ediscoveryAllowedStorage")]
    pub ediscovery_allowed_storage: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMailDetail {
    #[serde(rename = "sendMailId")]
    pub send_mail_id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "serverName")]
    pub server_name: Option<String>,
    #[serde(rename = "signatureId")]
    pub signature_id: Option<String>,
    #[serde(rename = "serverPort")]
    pub server_port: Option<i32>,
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
    #[serde(rename = "connectionType")]
    pub connection_type: Option<String>,
    pub mode: Option<String>,
    pub validated: Option<bool>,
    #[serde(rename = "fromAddress")]
    pub from_address: Option<String>,
    #[serde(rename = "smtpConnection")]
    pub smtp_connection: Option<i32>,
    #[serde(rename = "validationRequired")]
    pub validation_required: Option<bool>,
    #[serde(rename = "validationState")]
    pub validation_state: Option<i32>,
    pub status: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub country: Option<String>,
    #[serde(rename = "streetAddr")]
    pub street_addr: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PolicyId {
    #[serde(rename = "1082700000192558003")]
    pub business_policy: Option<String>,
    pub zoid: Option<i64>,
}
