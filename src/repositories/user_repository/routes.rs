use crate::db::db::{Connection, IConnection};
use crate::middlewares::auth::AuthorizationService;
use crate::models::user::{Login, Register};
use crate::repositories::user_repository::{IUserRepository, UserRepository};
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpRequest, HttpResponse};

#[post("/login")]
async fn login(user: web::Json<Login>) -> HttpResponse {
    let _connection: Connection = Connection {};
    let _repository: UserRepository = UserRepository {
        connection: _connection.init(),
    };
    let proc = _repository.login(user.into_inner());

    match proc {
        Ok(_) => HttpResponse::Ok().json(proc.unwrap()),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::from_u16(401).unwrap())
            .json(proc.unwrap_err()),
    }
}

#[post("/register")]
async fn register(user: web::Json<Register>) -> HttpResponse {
    let _connection: Connection = Connection {};
    let _repository: UserRepository = UserRepository {
        connection: _connection.init(),
    };
    HttpResponse::Ok().json(_repository.register(user.into_inner()))
}

#[post("/userInformations")]
async fn user_informations(_req: HttpRequest) -> HttpResponse {
    let _auth = _req.headers().get("Authorization");
    let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
    let token = _split[1].trim();
    let _connection: Connection = Connection {};
    let _repository: UserRepository = UserRepository {
        connection: _connection.init(),
    };
    match _repository.user_informations(token) {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(err) => HttpResponse::Ok().json(err),
    }
}

#[post("/protectedRoute")]
async fn protected(_: AuthorizationService) -> HttpResponse {
    let _connection: Connection = Connection {};
    let _repository: UserRepository = UserRepository {
        connection: _connection.init(),
    };
    HttpResponse::Ok().json(_repository.protected_function())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(user_informations);
    cfg.service(protected);
}
