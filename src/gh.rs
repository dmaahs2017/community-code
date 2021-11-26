use reqwest::blocking as reqwest;

const GH_BASE_URL: &'static str = "https://github.com";
const API_BASE_URL: &'static str = "https://api.github.com";
const OATH_ENDPOINT: &'static str = "/login/oauth/access_token";
const DEV_CODE_ENDPOINT: &'static str = "/login/device/code";
const GET_USER_ENDPIONT: &'static str = "/user";

const ACCEPT_JSON_HEADER: (&'static str, &'static str) = ("Accept", "application/json");

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DeviceResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: i32,
    interval: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OathTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct UserResponse {
      login: String,
      id: i32,
      node_id: String,
      avatar_url: String,
      gravatar_id: String,
      url: String,
      html_url: String,
      followers_url: String,
      following_url: String,
      gists_url: String,
      starred_url: String,
      subscriptions_url: String,
      organizations_url: String,
      repos_url: String,
      events_url: String,
      received_events_url: String,
      r#type: String,
      site_admin: bool,
      name: String,
      company: Option<String>,
      blog: String,
      location: String,
      email: String,
      hireable: Option<bool>,
      bio: String,
      twitter_username: Option<String>,
      public_repos: i32,
      public_gists: i32,
      followers: i32,
      following: i32,
      created_at: String,
      updated_at: String,
}

pub fn post_device_code(client_id: &str) -> DeviceResponse {
    reqwest::Client::new()
        .post(format!("{}{}", GH_BASE_URL, DEV_CODE_ENDPOINT))
        .query(&[("client_id", client_id)])
        .header("Accept", "application/json")
        .send()
        .unwrap()
        .json::<DeviceResponse>()
        .unwrap()
}


// TODO fallible. Before user enters the user_code the call will fail. Need to be able to handle
// this for retries.
pub fn get_oath_token(client_id: &str, dev_response: &DeviceResponse) -> OathTokenResponse {
    reqwest::Client::new()
        .get(format!("{}{}", GH_BASE_URL, OATH_ENDPOINT))
        .query(&[
            ("client_id", client_id),
            ("device_code", &dev_response.device_code),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ])
        .header(ACCEPT_JSON_HEADER.0, ACCEPT_JSON_HEADER.1)
        .send()
        .unwrap()
        .json()
        .unwrap()
}

pub fn get_user_info(access_token: &str) -> UserResponse {
    reqwest::Client::new()
        .get(format!("{}{}", API_BASE_URL, GET_USER_ENDPIONT))
        .header(ACCEPT_JSON_HEADER.0, ACCEPT_JSON_HEADER.1)
        .header("Authorization", format!("token {}", access_token))
        .header("User-Agent", "")
        .send()
        .expect("Failed 1")
        .json()
        .expect("Failed 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn post_device_code_works() {
        let response = post_device_code("ed4d31187fb410dc562d");

        dbg!(response);
    }

    #[test]
    #[ignore]
    fn get_oath_works() {
        let response = get_oath_token("ed4d31187fb410dc562d", &DeviceResponse {
            device_code: "ff77c553fc2ef2a9fed805b671bc6216dfb41498".to_string(),
            user_code: "8677-D1EE".to_string(),
            ..Default::default()
        });

        dbg!(response);
    }

    #[test]
    #[ignore]
    fn get_user_info_works() {
        let response = get_user_info("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");
        dbg!(response);
    }
}
