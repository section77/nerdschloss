pub mod configuration;

// use mattermost_rust_client::{apis, models};
use tracing::{error, info};

use configuration::MatterMost;

// pub async fn mattermost(configuration: &MatterMost, state: bool) {
//     if !configuration.enable {
//         return;
//     }

//     let url = configuration.url.clone();
//     let _login_id = configuration.loginid.clone();
//     let api_token = configuration.apitoken.clone();
//     let scheme = configuration.scheme.clone();
//     let port: u16 = configuration.port;

//     // Create configuration
//     let mut config = apis::configuration::Configuration::new();
//     config.base_path = format!("{scheme}://{url}/api/v4");
//     let api_key = apis::configuration::ApiKey {
//         prefix: None,
//         key: api_token,
//     };
//     config.api_key = Some(api_key);

//     let _user = match apis::users_api::get_user(&config, "me").await {
//         Ok(user) => user,
//         Err(e) => {
//             error!("Error: {e}");
//             return;
//         }
//     };

//     // Get team by name "section77"
//     let team = match apis::teams_api::get_team_by_name(&config, "section77").await {
//         Ok(team) => team,
//         Err(e) => {
//             error!("Error: {e}");
//             return;
//         }
//     };
//     let team_id = match team.id.as_ref().ok_or("Team ID is missing") {
//         Ok(team_id) => team_id,
//         Err(e) => {
//             error!("Error: {e}");
//             return;
//         }
//     };

//     // Get channel by name "clubstatus"
//     let channel =
//         match apis::channels_api::get_channel_by_name(&config, team_id, "clubstatus", None).await {
//             Ok(channel) => channel,
//             Err(e) => {
//                 error!("Error: {e}");
//                 return;
//             }
//         };
//     let channel_id = match channel.id.as_ref().ok_or("Channel ID is missing") {
//         Ok(channel_id) => channel_id,
//         Err(e) => {
//             error!("Error: {e}");
//             return;
//         }
//     };

//     // Determine message and display name based on state
//     let (message, display_name) = match &state {
//         true => ("TEST: Die Section77 ist offen", "Status: Offen"),
//         false => ("TEST: Die Section77 ist geschlossen", "Status: Geschlossen"),
//     };

//     // Create post
//     let post_request = models::CreatePostRequest::new(channel_id.clone(), message.to_string());

//     match apis::posts_api::create_post(&config, post_request, None).await {
//         Ok(_) => {
//             info!("Post created");
//         }
//         Err(e) => {
//             error!("Error: {e}");
//             return;
//         }
//     }

//     // Update channel display name
//     let patch_channel = models::PatchChannelRequest {
//         display_name: Some(display_name.to_string()),
//         ..Default::default()
//     };

//     match apis::channels_api::patch_channel(&config, channel_id, patch_channel).await {
//         Ok(_) => {
//             info!("Channel updated");
//         }
//         Err(e) => {
//             error!("Error: {e}");
//         }
//     }

//     match apis::users_api::logout(&config).await {
//         Ok(_) => {
//             info!("Logout successful");
//         }
//         Err(e) => {
//             error!("Error: {e}");
//         }
//     }
// }

pub async fn mattermost(configuration: &MatterMost, state: bool) {
    if !configuration.enable {
        return;
    }

    let code = std::include_str!("../../../backend/python/mattermost.py");

    #[allow(unused_variables)]
    let output = match std::process::Command::new(&configuration.python)
        .args([
            "-c",
            code,
            configuration.url.as_str(),
            configuration.loginid.as_str(),
            configuration.apitoken.as_str(),
            configuration.scheme.as_str(),
            &configuration.port.to_string(),
            &state.to_string(),
        ])
        .output()
    {
        Ok(output) => {
            info!("Successfully executed MatterMost Python script");
            output
        }
        Err(e) => {
            error!("Failed to execute MatterMost Python script: {e}");
            return;
        }
    };

    #[cfg(test)]
    {
        println!("output start");
        println!("Script stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
        println!("Script exit code: {}", output.status);
        println!("output end");
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wiremock::{
        http::Method,
        matchers::{body_json, method, path},
        {Mock, MockServer, ResponseTemplate},
    };

    use crate::mattermost::configuration::MatterMost;

    use super::mattermost;

    #[tokio::test]
    async fn mattermost_disabled() {
        let mock_server = MockServer::start().await;

        Mock::given(method(Method::GET))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&mock_server)
            .await;

        let mm_config = MatterMost {
            enable: false,
            ..Default::default()
        };
        mattermost(&mm_config, true).await;
    }

    #[tokio::test]
    async fn mattermost_state_open() {
        let mock_server = MockServer::start().await;

        // Mock GET /users/me to validate token
        Mock::given(method(Method::GET))
            .and(path("/api/v4/users/me"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "user-id",
                "username": "clubstatus"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock GET team by name
        Mock::given(method(Method::GET))
            .and(path("/api/v4/teams/name/section77"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "team-id",
                "name": "section77"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock GET channel by name
        Mock::given(method(Method::GET))
            .and(path("/api/v4/teams/team-id/channels/name/clubstatus"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "channel-id",
                "name": "clubstatus"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock POST to create a post
        Mock::given(method(Method::POST))
            .and(path("/api/v4/posts"))
            .and(body_json(json!({
                "channel_id": "channel-id",
                "message": "Die Section77 ist offen"
            })))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": "post-id"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock PATCH to update channel display name
        Mock::given(method(Method::PUT))
            .and(path("/api/v4/channels/channel-id/patch"))
            .and(body_json(json!({
                "id": "channel-id",
                "display_name": "Status: Offen"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "channel-id",
                "display_name": "Status: Offen"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock logout
        Mock::given(method(Method::POST))
            .and(path("/api/v4/users/logout"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Create MatterMost configuration
        let mm_config = MatterMost {
            python: String::from("../.venv/bin/python"),
            enable: true,
            url: mock_server.address().ip().to_string(),
            loginid: String::from("clubstatus"),
            apitoken: String::from("apitoken"),
            scheme: String::from("http"),
            port: mock_server.address().port(),
        };

        // Call the function with state = true (open)
        mattermost(&mm_config, true).await;
    }

    #[tokio::test]
    async fn mattermost_state_closed() {
        let mock_server = MockServer::start().await;

        // Mock GET /users/me to validate token
        Mock::given(method(Method::GET))
            .and(path("/api/v4/users/me"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "user-id",
                "username": "clubstatus"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock GET team by name
        Mock::given(method(Method::GET))
            .and(path("/api/v4/teams/name/section77"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "team-id",
                "name": "section77"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock GET channel by name
        Mock::given(method(Method::GET))
            .and(path("/api/v4/teams/team-id/channels/name/clubstatus"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "channel-id",
                "name": "clubstatus"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock POST to create a post for closed state
        Mock::given(method(Method::POST))
            .and(path("/api/v4/posts"))
            .and(body_json(json!({
                "channel_id": "channel-id",
                "message": "Die Section77 ist geschlossen"
            })))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": "post-id"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock PATCH to update channel display name for closed state
        Mock::given(method(Method::PUT))
            .and(path("/api/v4/channels/channel-id/patch"))
            .and(body_json(json!({
                "id": "channel-id",
                "display_name": "Status: Geschlossen"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({

                "display_name": "Status: Geschlossen"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock logout
        Mock::given(method(Method::POST))
            .and(path("/api/v4/users/logout"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Create MatterMost configuration
        let mm_config = MatterMost {
            python: String::from("../.venv/bin/python"),
            enable: true,
            url: mock_server.address().ip().to_string(),
            loginid: String::from("clubstatus"),
            apitoken: String::from("apitoken"),
            scheme: String::from("http"),
            port: mock_server.address().port(),
        };

        // Call the function with state = false (closed)
        mattermost(&mm_config, false).await;
    }

    #[tokio::test]
    async fn mattermost_api_error_handling() {
        let mock_server = MockServer::start().await;

        // Mock GET /users/me to return 401 Unauthorized
        Mock::given(method(Method::GET))
            .and(path("/api/v4/users/me"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "message": "Invalid token"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let mm_config = MatterMost {
            python: String::from("../.venv/bin/python"),
            enable: true,
            url: mock_server.address().ip().to_string(),
            loginid: String::from("clubstatus"),
            apitoken: String::from("invalid-token"),
            scheme: String::from("http"),
            port: mock_server.address().port(),
        };

        // This should handle the error gracefully
        // Note: The current Python script may not have sophisticated error handling,
        // but this test documents the expected behavior
        mattermost(&mm_config, true).await;
    }

    #[tokio::test]
    async fn mattermost_team_not_found() {
        let mock_server = MockServer::start().await;

        // Mock GET /users/me to validate token
        Mock::given(method(Method::GET))
            .and(path("/api/v4/users/me"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "user-id",
                "username": "clubstatus"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock GET team by name to return 404
        Mock::given(method(Method::GET))
            .and(path("/api/v4/teams/name/section77"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "message": "Team not found"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let mm_config = MatterMost {
            python: String::from("../.venv/bin/python"),
            enable: true,
            url: mock_server.address().ip().to_string(),
            loginid: String::from("clubstatus"),
            apitoken: String::from("apitoken"),
            scheme: String::from("http"),
            port: mock_server.address().port(),
        };

        // This should handle the team not found error
        mattermost(&mm_config, true).await;
    }

    #[tokio::test]
    async fn mattermost_channel_not_found() {
        let mock_server = MockServer::start().await;

        // Mock GET /users/me to validate token
        Mock::given(method(Method::GET))
            .and(path("/api/v4/users/me"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "user-id",
                "username": "clubstatus"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock GET team by name
        Mock::given(method(Method::GET))
            .and(path("/api/v4/teams/name/section77"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "team-id",
                "name": "section77"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Mock GET channel by name to return 404
        Mock::given(method(Method::GET))
            .and(path("/api/v4/teams/team-id/channels/name/clubstatus"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "message": "Channel not found"
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let mm_config = MatterMost {
            python: String::from("../.venv/bin/python"),
            enable: true,
            url: mock_server.address().ip().to_string(),
            loginid: String::from("clubstatus"),
            apitoken: String::from("apitoken"),
            scheme: String::from("http"),
            port: mock_server.address().port(),
        };

        // This should handle the channel not found error
        mattermost(&mm_config, true).await;
    }
}
