use chrono::Local;

use crate::tenants::token::{Claims, TinyTenant, create_token, decode_token};

fn test_csrf_key() {
    std::env::set_var("CSRF_TOKEN_KEY", "01234567890123456789012345678901");
}

fn sample_claims(exp: i64) -> Claims {
    Claims {
        iat: Local::now().timestamp(),
        sub: 42,
        exp,
        xsrf_token: "test-xsrf".into(),
        email: "u@example.com".into(),
        username: "user1".into(),
        role: "admin".into(),
        login_session: "sess-1".into(),
    }
}

#[test]
fn claims_is_expired_respects_exp() {
    let past = sample_claims(Local::now().timestamp() - 60);
    assert!(past.is_expired());

    let future = sample_claims(Local::now().timestamp() + 3600);
    assert!(!future.is_expired());
}

#[test]
fn claims_user_and_role_helpers() {
    let c = sample_claims(Local::now().timestamp() + 10);
    assert!(c.is_claimed_user(42));
    assert!(!c.is_claimed_user(99));
    assert!(c.has_role("admin"));
    assert!(!c.has_role("guest"));
    assert_eq!(c.get_tenant_id(), 42);
}

#[test]
fn tiny_tenant_from_claims() {
    let c = sample_claims(Local::now().timestamp() + 10);
    let t = TinyTenant::from(c);
    assert_eq!(t.id, 42);
    assert_eq!(t.email, "u@example.com");
    assert_eq!(t.username, "user1");
    assert_eq!(t.role, "admin");
    assert_eq!(t.login_session, "sess-1");
}

#[test]
fn decode_token_rejects_garbage() {
    assert!(decode_token("not-a-valid-jwt").is_err());
}

#[test]
fn create_token_round_trips_through_decode() {
    test_csrf_key();
    let token = create_token(7, "a@b.c", "alice", "member", "login-xyz");
    let claims = decode_token(&token).expect("valid token");
    assert_eq!(claims.sub, 7);
    assert_eq!(claims.email, "a@b.c");
    assert_eq!(claims.username, "alice");
    assert_eq!(claims.role, "member");
    assert_eq!(claims.login_session, "login-xyz");
    assert!(!claims.xsrf_token.is_empty());
}
