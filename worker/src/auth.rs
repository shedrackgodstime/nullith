use worker::*;

pub fn verify_api_key(req: &Request, env: &Env) -> Result<bool, worker::Error> {
    let api_key = match env.secret("API_KEY") {
        Ok(key) => {
            log::info!(
                "API_KEY secret found, value length: {}",
                key.to_string().len()
            );
            key.to_string()
        }
        Err(e) => {
            log::warn!("API_KEY secret error: {:?}", e);
            return Err(worker::Error::from("API_KEY not configured"));
        }
    };

    if api_key.is_empty() {
        log::warn!("API_KEY is empty");
        return Err(worker::Error::from("API_KEY is empty"));
    }

    match req.headers().get("x-api-key") {
        Ok(Some(key)) if key == api_key => Ok(true),
        Ok(Some(_)) => {
            log::warn!("Invalid API key provided");
            Err(worker::Error::from("Invalid API key"))
        }
        Ok(None) => {
            log::warn!("No API key provided");
            Err(worker::Error::from("Missing API key"))
        }
        Err(e) => {
            log::error!("Error getting header: {:?}", e);
            Err(worker::Error::from("Error reading header"))
        }
    }
}

#[allow(dead_code)]
pub fn is_auth_enabled(env: &Env) -> bool {
    match env.secret("API_KEY") {
        Ok(key) => !key.to_string().is_empty(),
        Err(_) => false,
    }
}
