use log::error;
use reqwest;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    pub usuario: String,
    pub senha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub token_type: String,
    pub access_token: String,
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Status error")]
    StatusError,
    #[error("Request Error")]
    InvalidRequest(#[from] reqwest::Error),
}

// send get request using reqwest
async fn get_request<T>(url: &str, header: &HeaderMap) -> Result<T, RequestError>
where
    T: Serialize + DeserializeOwned,
{
    let response = reqwest::Client::new()
        .get(url)
        .headers(header.clone())
        .send()
        .await?;

    if !response.status().is_success() {
        error!("Status Code: {:?}", response.status());
        error!("Text {:?}", response.text().await?);
        return Err(RequestError::StatusError);
    }

    let response_data = response.json::<T>().await?;
    Ok(response_data)
}

// send post request using reqwest
async fn post_request<T, N>(url: &str, header: &HeaderMap, data: &T) -> Result<N, RequestError>
where
    T: Serialize + DeserializeOwned,
    N: Serialize + DeserializeOwned,
{
    let response = reqwest::Client::new()
        .post(url)
        .headers(header.clone())
        .json(data)
        .send()
        .await?;

    if !response.status().is_success() {
        error!("Status Code: {:?}", response.status());
        error!("Text {:?}", response.text().await?);
        return Err(RequestError::StatusError);
    }

    let response_data = response.json::<N>().await?;
    Ok(response_data)
}

pub async fn get_auth_token(
    url: &str,
    username: &str,
    password: &str,
) -> Result<String, RequestError> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let user_credentias = UserCredentials {
        usuario: username.to_string(),
        senha: password.to_string(),
    };

    let auth: AuthToken = post_request(url, &headers, &user_credentias).await?;

    Ok(format!("{} {}", auth.token_type, auth.access_token))
}

pub async fn get_measures<T>(url: &str, token: &str) -> Result<Vec<T>, RequestError>
where
    T: Serialize + DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, token.parse().unwrap());

    let response: Vec<T> = get_request(url, &headers).await?;

    Ok(response)
}
