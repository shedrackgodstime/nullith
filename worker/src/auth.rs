use worker::*;

pub fn verify_api_key(req: &Request, env: &Env) -> Result<bool, worker::Error> {
    let api_key = match env.secret("API_KEY") {
        Ok(key) => {
            log::info!("API_KEY secret found");
            key.to_string()
        }
        Err(e) => {
            log::warn!("API_KEY secret error: {:?}", e);
            return Ok(false);
        }
    };

    if api_key.is_empty() {
        log::warn!("API_KEY is empty");
        return Ok(false);
    }

    match req.headers().get("x-api-key") {
        Ok(Some(key)) if key == api_key => Ok(true),
        Ok(Some(_)) => {
            log::warn!("Invalid API key provided");
            Ok(false)
        }
        Ok(None) => {
            log::warn!("No API key provided");
            Ok(false)
        }
        Err(e) => {
            log::error!("Error getting header: {}", e);
            Ok(false)
        }
    }
}

pub fn is_auth_enabled(env: &Env) -> bool {
    match env.secret("API_KEY") {
        Ok(key) => !key.to_string().is_empty(),
        Err(_) => false,
    }
}