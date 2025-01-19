use std::path::Path;

use reqwest::{header::AUTHORIZATION, Client, StatusCode};

use crate::tauface::{get_meta_internal, Error, LoginResponse, Meta};

pub async fn make_get_request<T>(
    url_path: &str,
    data_dir: &Path,
    query: Option<&[(&str, &str)]>,
) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let meta = get_meta_internal(data_dir).await?;

    let client = Client::new();
    let response = client
        .get(format!("http://{}{}", meta.server_ip, url_path))
        .header(AUTHORIZATION, format!("Bearer {}", meta.access_token));

    let response = match query {
        Some(query) => response.query(query),
        None => response,
    };
    let response = response
        .send()
        .await
        .map_err(|e| Error::Server(e.to_string()))?;

    match response.status() {
        StatusCode::OK => {
            let response = response
                .json::<T>()
                .await
                .map_err(|e| Error::Client(e.to_string()))?;
            Ok(response)
        }
        StatusCode::UNAUTHORIZED => Err(Error::Server("Unauthorized".to_string())),
        StatusCode::NETWORK_AUTHENTICATION_REQUIRED => {
            let refresh_token = meta.refresh_token;
            let new_tokens = client
                .post(format!("http://{}/auth/refresh", meta.server_ip))
                .body(refresh_token)
                .send()
                .await
                .map_err(|e| Error::Server(e.to_string()))?;

            let new_tokens = match new_tokens.status() {
                StatusCode::OK => new_tokens
                    .json::<LoginResponse>()
                    .await
                    .map_err(|e| Error::Client(e.to_string()))?,
                StatusCode::UNAUTHORIZED => {
                    return Err(Error::Server("Unauthorized".to_string()));
                }
                StatusCode::NETWORK_AUTHENTICATION_REQUIRED => {
                    return Err(Error::LoginExpired);
                }
                _ => {
                    return Err(Error::Client(format!(
                        "Unknown error: {}",
                        response.status()
                    )));
                }
            };

            let meta = Meta {
                username: meta.username,
                server_ip: meta.server_ip,
                access_token: new_tokens.access_token,
                refresh_token: new_tokens.refresh_token,
            };

            let meta_json =
                serde_json::to_string(&meta).map_err(|e| Error::Client(e.to_string()))?;
            let meta_path = data_dir.join("meta.json");
            std::fs::write(meta_path, meta_json).map_err(|e| Error::Client(e.to_string()))?;

            let response = client
                .get(format!("http://{}/{}", meta.server_ip, url_path))
                .header(AUTHORIZATION, format!("Bearer {}", meta.access_token))
                .send()
                .await
                .map_err(|e| Error::Server(e.to_string()))?
                .json::<T>()
                .await
                .map_err(|e| Error::Client(e.to_string()))?;

            Ok(response)
        }
        _ => Err(Error::Client(format!(
            "Unknown error: {}",
            response.status()
        ))),
    }
}

pub async fn make_post_request<T>(url_path: &str, data_dir: &Path, data: &T) -> Result<(), Error>
where
    T: serde::Serialize,
{
    let meta = get_meta_internal(data_dir).await?;

    let client = Client::new();
    let response = client
        .post(format!("http://{}{}", meta.server_ip, url_path))
        .json(data)
        .header(AUTHORIZATION, format!("Bearer {}", meta.access_token));
    let response = response
        .send()
        .await
        .map_err(|e| Error::Server(e.to_string()))?;

    match response.status() {
        StatusCode::OK => Ok(()),
        StatusCode::UNAUTHORIZED => Err(Error::Server("Unauthorized".to_string())),
        StatusCode::NETWORK_AUTHENTICATION_REQUIRED => {
            let refresh_token = meta.refresh_token;
            let new_tokens = client
                .post(format!("http://{}/auth/refresh", meta.server_ip))
                .body(refresh_token)
                .send()
                .await
                .map_err(|e| Error::Server(e.to_string()))?;

            let new_tokens = match new_tokens.status() {
                StatusCode::OK => new_tokens
                    .json::<LoginResponse>()
                    .await
                    .map_err(|e| Error::Client(e.to_string()))?,
                StatusCode::UNAUTHORIZED => {
                    return Err(Error::Server("Unauthorized".to_string()));
                }
                StatusCode::NETWORK_AUTHENTICATION_REQUIRED => {
                    return Err(Error::LoginExpired);
                }
                _ => {
                    return Err(Error::Client(format!(
                        "Unknown error: {}",
                        response.status()
                    )));
                }
            };

            let meta = Meta {
                username: meta.username,
                server_ip: meta.server_ip,
                access_token: new_tokens.access_token,
                refresh_token: new_tokens.refresh_token,
            };

            let meta_json =
                serde_json::to_string(&meta).map_err(|e| Error::Client(e.to_string()))?;
            let meta_path = data_dir.join("meta.json");
            std::fs::write(meta_path, meta_json).map_err(|e| Error::Client(e.to_string()))?;

            let response = client
                .post(format!("http://{}/{}", meta.server_ip, url_path))
                .header(AUTHORIZATION, format!("Bearer {}", meta.access_token))
                .json(data)
                .send()
                .await
                .map_err(|e| Error::Server(e.to_string()))?;
            match response.status() {
                StatusCode::OK => Ok(()),
                _ => Err(Error::Server("Unknown error".to_string())),
            }
        }
        _ => Err(Error::Client(format!(
            "Unknown error: {}",
            response.status()
        ))),
    }
}
