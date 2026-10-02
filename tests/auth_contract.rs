use honey_id_types::endpoints::auth_flow::{HoneySubmitPasswordResponse, HoneySubmitUsernameRequest};
use serde_json::{Value, json};

#[test]
fn old_username_request_omits_hosted_request_identifier() {
    let request: HoneySubmitUsernameRequest = serde_json::from_value(json!({
        "appPublicId": "AbCdEf0123456789",
        "username": "legacy-user",
    }))
    .expect("existing two-parameter username clients remain valid");
    let encoded = serde_json::to_value(request).unwrap();
    assert_eq!(encoded["username"], "legacy-user");
    assert!(encoded.get("loginRequestId").is_none_or(Value::is_null));
}

#[test]
fn hosted_username_request_retains_its_optional_binding() {
    let request: HoneySubmitUsernameRequest = serde_json::from_value(json!({
        "appPublicId": "AbCdEf0123456789",
        "username": "hosted-user",
        "loginRequestId": "opaque-hosted-request",
    }))
    .unwrap();
    let encoded = serde_json::to_value(request).unwrap();
    assert_eq!(encoded["loginRequestId"], "opaque-hosted-request");
}

#[test]
fn totp_completion_wire_shape_has_no_usable_session() {
    let response: HoneySubmitPasswordResponse = serde_json::from_value(json!({
        "nextStep": "Totp",
        "twoFactorExpiresAt": 1_800_000_000_i64,
    }))
    .expect("pending TOTP is a supported password response");
    let encoded = serde_json::to_value(response).unwrap();
    assert!(encoded.get("accessToken").is_none_or(Value::is_null));
    assert!(encoded.get("encryptionKey").is_none_or(Value::is_null));
    assert_eq!(encoded["twoFactorExpiresAt"], 1_800_000_000_i64);
}

#[test]
fn normal_completion_preserves_uuid_token_and_composed_key() {
    let token = "89e4c1a5-3882-4e50-b167-d45889844bd2";
    let response: HoneySubmitPasswordResponse = serde_json::from_value(json!({
        "nextStep": "Complete",
        "accessToken": token,
        "encryptionKey": "composed-app-user-key",
    }))
    .unwrap();
    let encoded = serde_json::to_value(response).unwrap();
    assert_eq!(encoded["accessToken"], token);
    assert_eq!(encoded["encryptionKey"], "composed-app-user-key");
}
