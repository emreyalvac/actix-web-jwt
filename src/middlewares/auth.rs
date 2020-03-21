use actix_web::{FromRequest, Error, HttpRequest, dev};
use futures::future::{ok, err, Ready};
use actix_web::error::ErrorUnauthorized;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::config::{Config, IConfig};
use crate::models::user::Claims;


pub struct AuthorizationService;

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let _config: Config = Config {};
                let _var = _config.get_config_with_key("SECRET_KEY");
                let key = _var.as_bytes();
                match decode::<Claims>(token.as_ref(), &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS256)) {
                    Ok(token) => {
                        ok(AuthorizationService)
                    }
                    Err(e) => {
                        err(ErrorUnauthorized("invalid token!"))
                    }
                }
            }
            None => {
                err(ErrorUnauthorized("blocked!"))
            }
        }
    }
}
