use log::info;
use r2d2_postgres::{postgres, PostgresConnectionManager};
use rouille::{Request, Response};
use serde::{Deserialize, Serialize};

use crate::{library};

#[derive(Serialize, Deserialize)]
struct UserObject {
    name: String,
    email: String,
    organization: String,
    phone: String,
    location: String,
    level: i32,
}

#[derive(Serialize, Deserialize)]
struct CreateUser {
    user: UserObject,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    result: String,
}

#[derive(Serialize, Deserialize)]
struct GetUserResponse {
    user: UserObject,
}

#[derive(Serialize, Deserialize)]
struct GetUserRequest {
    email: String,
}

#[derive(Serialize, Deserialize)]
struct UserAuthRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct UserAuthResponse {
    token: String,
    level: i32,
    email: String,
}

pub(crate) fn on_user_create(
    request: &Request,
    connection_pool: r2d2::Pool<PostgresConnectionManager<postgres::NoTls>>,
) -> Response {
    let data = library::utils::request_to_bytes(&request);
    let user_object: CreateUser;
    match serde_json::from_slice(&data) {
        Ok(user_data) => {
            user_object = user_data;
        }
        Err(_) => {
            return Response::json(&CreateUserResponse {
                result: library::enums::ERROR_RESPONSE_INVALID_JSON.to_string(),
            });
        }
    };
    let level = library::utils::get_user_level(&request);
    if user_object.user.level > 1 && level != 3 {
        return Response::json(&CreateUserResponse {
            result: library::enums::ERROR_RESPONSE_ACCESS_ERROR.to_string(),
        });
    }

    let mut conn = connection_pool.get().unwrap();
    match  conn.execute(
        "INSERT INTO platform_users (name, password, email, organization, phone, location, level, version) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[&user_object.user.name, &user_object.password, &user_object.user.email, &user_object.user.organization, &user_object.user.phone, &user_object.user.location, &user_object.user.level, &library::enums::VERSION]
    ) {
        Ok(_) => {
            Response::json(&CreateUserResponse{result: library::enums::ERROR_RESPONSE_OK.to_string()})
        },
        Err(err) => {
            log::error!("Error creating user: {}", err);
            Response::json(&CreateUserResponse{result: library::enums::ERROR_RESPONSE_ALREADY_EXISTS.to_string()})
        }
    }
}

pub(crate) fn on_user_get(
    request: &Request,
    connection_pool: r2d2::Pool<PostgresConnectionManager<postgres::NoTls>>,
) -> Response {
    let level = library::utils::get_user_level(&request);
    if level == 0 {
        return Response::json(&CreateUserResponse {
            result: library::enums::ERROR_RESPONSE_ACCESS_ERROR.to_string(),
        });
    }
    let data = library::utils::request_to_bytes(&request);
    let get_user: GetUserRequest = match serde_json::from_slice(&data) {
        Ok(user_data) => user_data,
        Err(_) => {
            return Response::json(&CreateUserResponse {
                result: library::enums::ERROR_RESPONSE_INVALID_JSON.to_string(),
            });
        }
    };
    let mut conn = connection_pool.get().unwrap();
    let result = conn.query(
        "SELECT name, email, organization, phone, location, level FROM platform_users WHERE email = $1",
        &[&get_user.email]
    );
    match result {
        Ok(rows) => {
            if rows.len() == 0 {
                info!("User not found");
                return Response::json(&CreateUserResponse {
                    result: library::enums::ERROR_RESPONSE_DOESNT_EXISTS.to_string(),
                });
            }
            if let Some(row) = rows.get(0) {
                let user = UserObject {
                    name: row.get(0),
                    email: row.get(1),
                    organization: row.get(2),
                    phone: row.get(3),
                    location: row.get(4),
                    level: row.get(5),
                };
                return Response::json(&GetUserResponse { user });
            } else {
                return Response::json(&CreateUserResponse {
                    result: library::enums::ERROR_RESPONSE_DOESNT_EXISTS.to_string(),
                });
            }
        }
        Err(err) => {
            log::error!("Error getting user: {}", err);
            Response::json(&CreateUserResponse {
                result: library::enums::ERROR_RESPONSE_DOESNT_EXISTS.to_string(),
            })
        }
    }
}

pub(crate) fn on_user_auth(
    request: &Request,
    connection_pool: r2d2::Pool<PostgresConnectionManager<postgres::NoTls>>,
) -> Response {
    let data = library::utils::request_to_bytes(request);
    let user_auth: UserAuthRequest = match serde_json::from_slice(&data) {
        Ok(user_data) => user_data,
        Err(_) => {
            return Response::json(&CreateUserResponse {
                result: library::enums::ERROR_RESPONSE_INVALID_JSON.to_string(),
            });
        }
    };
    let mut conn = connection_pool.get().unwrap();
    let result = conn.query(
        "SELECT level FROM platform_users WHERE email = $1 AND password = $2",
        &[&user_auth.email, &user_auth.password],
    );
    match result {
        Ok(rows) => {
            if rows.len() == 0 {
                info!("User not found");
                return Response::json(&CreateUserResponse {
                    result: library::enums::ERROR_RESPONSE_DOESNT_EXISTS.to_string(),
                });
            }
            if let Some(row) = rows.get(0) {
                let token = library::utils::generate_token();
                return Response::json(&UserAuthResponse {
                    token,
                    level: row.get(0),
                    email: user_auth.email,
                });
            } else {
                return Response::json(&CreateUserResponse {
                    result: library::enums::ERROR_RESPONSE_DOESNT_EXISTS.to_string(),
                });
            }
        }
        Err(err) => {
            log::error!("Error authenticating user: {}", err);
            Response::json(&CreateUserResponse {
                result: library::enums::ERROR_RESPONSE_DOESNT_EXISTS.to_string(),
            })
        }
    }
}
