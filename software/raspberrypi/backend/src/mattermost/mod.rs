pub mod configuration;

mod client;
mod types;

use std::process::Command;

use configuration::MatterMost;

pub fn mattermost(configuration: &MatterMost, state: bool) {
    if configuration.enable {
        let code = std::include_str!("../../../backend/python/mattermost.py");
        let output = Command::new(&configuration.python)
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
            .expect("Error: Failed to set MatterMost message");

        println!("output start");
        use std::io::{self, Write};
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        println!("output end");
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        http::Method,
        matchers::{body_json, header, method, path},
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
        mattermost(&mm_config, true);
    }

    #[tokio::test]
    async fn send_mattermost_message() {
        let mock_server = MockServer::start().await;

        use super::types::users::{NotifyProps, Props, Timezone, UserResponse};
        let user_response = UserResponse {
            id: "user123".to_string(),
            create_at: 1612345678,
            update_at: 1612345678,
            delete_at: 0,
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            nickname: "tester".to_string(),
            email: "test@example.com".to_string(),
            email_verified: true,
            auth_service: "email".to_string(),
            roles: "system_user".to_string(),
            locale: "en".to_string(),
            notify_props: NotifyProps {
                email: "true".to_string(),
                push: "mention".to_string(),
                desktop: "all".to_string(),
                desktop_sound: "true".to_string(),
                mention_keys: "test,user".to_string(),
                channel: "true".to_string(),
                first_name: "false".to_string(),
            },
            props: Props::default(),
            last_password_update: 1612345678,
            last_picture_update: 1612345678,
            failed_attempts: 0,
            mfa_active: true,
            timezone: Timezone {
                use_automatic_timezone: true,
                manual_timezone: "".to_string(),
                automatic_timezone: "America/New_York".to_string(),
            },
            terms_of_service_id: "tos123".to_string(),
            terms_of_service_create_at: 1612345678,
        };

        Mock::given(method("GET"))
            .and(path("/api/v4/users/me"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&user_response))
            .expect(1)
            .mount(&mock_server)
            .await;

        use super::types::teams::TeamResponse;
        let team_response = TeamResponse {
            id: "section77".to_string(),
            create_at: 1612345678,
            update_at: 1612345678,
            delete_at: 0,
            display_name: "section77".to_string(),
            name: "section77".to_string(),
            description: "This is a test team".to_string(),
            email: "team@example.com".to_string(),
            r#type: "O".to_string(), // 'O' for Open, 'I' for Invite only
            allowed_domains: "example.com".to_string(),
            invite_id: "invite123".to_string(),
            allow_open_invite: true,
            policy_id: "policy123".to_string(),
        };
        Mock::given(method(Method::GET))
            .and(path("api/v4/teams/name/section77"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&team_response))
            .expect(1)
            .mount(&mock_server)
            .await;

        Mock::given(method(Method::GET))
            .and(path("api/v4/teams/section77/channels/name/clubstatus"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&team_response))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Create a sample post response
        let post_response = super::types::posts::PostResponse {
            id: "post123".to_string(),
            create_at: 1612345678,
            update_at: 1612345678,
            delete_at: 0,
            edit_at: 0,
            user_id: "user123".to_string(),
            channel_id: "clubstatus".to_string(),
            root_id: "".to_string(),
            original_id: "".to_string(),
            message: "Die Section77 ist offen".to_string(),
            type_field: "".to_string(),
            props: super::types::posts::Props::default(),
            hashtag: "".to_string(),
            file_ids: vec!["file123".to_string()],
            pending_post_id: "".to_string(),
            metadata: super::types::posts::Metadata::default(),
        };

        Mock::given(method(Method::POST))
            .and(path("api/v4/posts"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&post_response))
            .mount(&mock_server)
            .await;

        // Expected request body
        let expected_request = super::types::channels::ChannelPatchRequest {
            name: "updated-channel".to_string(),
            display_name: "Updated Channel".to_string(),
            purpose: "This is an updated purpose".to_string(),
            header: "Updated channel header".to_string(),
        };

        // Create a sample channel response
        let channel_response = super::types::channels::ChannelResponse {
            id: "clubstatus".to_string(),
            create_at: 1612345678,
            update_at: 1612345679, // Updated timestamp
            delete_at: 0,
            team_id: "section77".to_string(),
            name: "clubstatus".to_string(),
            display_name: "Status: Offen".to_string(),
            purpose: "".to_string(),
            header: "".to_string(),
            type_field: "".to_string(),
            creator_id: "user123".to_string(),
            last_post_at: 1612345678,
            total_msg_count: 42,
            extra_update_at: 0,
        };
         Mock::given(method(Method::PUT))
            .and(path(r"api/v4/channels/clubstatus/patch"))
            .and(body_json(&expected_request))
            .respond_with(ResponseTemplate::new(200).set_body_json(&channel_response))
            .mount(&mock_server)
            .await;

        // Call funtion to set channel status
        let mm_config = MatterMost {
            python: String::from("../ve/bin/python"),
            enable: true,
            url: mock_server.address().ip().to_string(),
            loginid: String::from("clubstatus"),
            apitoken: String::from("apitoken"),
            scheme: String::from("http"),
            port: mock_server.address().port(),
        };
        mattermost(&mm_config, true);
    }
}
