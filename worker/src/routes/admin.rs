use worker::*;

use crate::auth;

fn check_admin(req: &Request, env: &Env) -> Result<(), worker::Error> {
    if !auth::verify_api_key(req, env)? {
        return Err(worker::Error::from("Unauthorized"));
    }
    Ok(())
}

pub async fn handle_rotate_key(_req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    check_admin(&_req, &ctx.env)?;

    let new_key = generate_secure_key();
    log::warn!("KEY ROTATION REQUESTED. New key generated. Update secret manually!");

    Response::from_json(&serde_json::json!({
        "action": "rotate_key",
        "message": "To complete key rotation:",
        "steps": [
            "1. Copy new key below",
            "2. Go to GitHub repo → Settings → Secrets → API_KEY",
            "3. Update the secret with new key",
            "4. Push any small change to trigger deploy"
        ],
        "new_key": new_key,
        "warning": "Old key will stop working after deploy"
    }))
}

fn generate_secure_key() -> String {
    let ts = js_sys::Date::now() as u64;
    let rnd = (js_sys::Math::random() * 1000000.0) as u64;
    let rnd2 = (js_sys::Math::random() * 1000000.0) as u64;
    format!("nullith_{:x}_{:x}_{:x}", ts, rnd, rnd2)
}
