#[cfg(test)]
mod tests {
    use crate::{
        organization::models::OrganizationResponse,
        tests::utils::{create_organization, send_request, setup_test_app, setup_test_auth},
    };
    use serde_json::from_slice;
    use serde_json::json;
    use warp::{http::StatusCode, hyper::body::to_bytes};

    const ORG_NAME: &str = "kudos";

    #[tokio::test]
    async fn test_organization_crud_operations() {
        let app = setup_test_app().await;
        let auth_header = setup_test_auth().await;

        
        // 1. Create an organization
        let org_payload = json!({"name": ORG_NAME}).to_string();
        let org_id = create_organization(&app, ORG_NAME)
            .await
            .expect("Failed to create organization");


        // 2. Attempt to create the same organization again and expect an error
        let duplicate_org_resp = send_request(
            &app,
            "POST",
            "/organizations",
            Some(org_payload.into_bytes()),
            Some(&auth_header),
        )
        .await;
        assert_eq!(
            duplicate_org_resp.status(),
            StatusCode::CONFLICT,
            "Expected conflict error for duplicate organization"
        );


        // 3. Retrieve the organization by ID and verify details
        let get_org_resp = send_request(
            &app,
            "GET",
            &format!("/organizations/{}", org_id),
            None,
            None,
        )
        .await;
        assert_eq!(
            get_org_resp.status(),
            StatusCode::OK,
            "Failed to retrieve organization"
        );
        let body_bytes = to_bytes(get_org_resp.into_body())
            .await
            .expect("Failed to read response body");
        let get_org_response: OrganizationResponse =
            from_slice(&body_bytes).expect("Failed to deserialize response");
        let expected_org_response = OrganizationResponse {
            id: org_id,
            name: ORG_NAME.to_string(),
        };
        assert!(get_org_response.eq(&expected_org_response));


        // 4. Retrieve all organizations and verify the list
        let query_string = format!("?name={}", ORG_NAME);
        let full_path = format!("/organizations{}", query_string);
        let get_all_resp = send_request(&app, "GET", &full_path, None, None).await;
        assert_eq!(
            get_all_resp.status(),
            StatusCode::OK,
            "Failed to retrieve organizations list"
        );
        let body_bytes = to_bytes(get_all_resp.into_body())
            .await
            .expect("Failed to read response body");
        let get_all_response: Vec<OrganizationResponse> =
            from_slice(&body_bytes).expect("Failed to deserialize response");
        let expected_vector = vec![expected_org_response];
        assert_eq!(
            get_all_response, expected_vector,
            "The retrieved organization list does not match the expected one."
        );


        // 5. Attempt to retrieve a non-existent organization
        let get_nonexistent_resp =
            send_request(&app, "GET", "/organizations/9999", None, None).await;
        assert_eq!(
            get_nonexistent_resp.status(),
            StatusCode::NOT_FOUND,
            "Expected not found error for non-existent organization"
        );


        // 6. Delete the organization and verify success
        let delete_resp = send_request(
            &app,
            "DELETE",
            &format!("/organizations/{}", org_id),
            None,
            Some(&auth_header),
        )
        .await;
        assert_eq!(
            delete_resp.status(),
            StatusCode::OK,
            "Failed to delete organization"
        );


        // 7. Attempt to delete the non-existent organization again and verify error
        let delete_again_resp = send_request(
            &app,
            "DELETE",
            &format!("/organizations/{}", org_id),
            None,
            Some(&auth_header),
        )
        .await;
        assert_eq!(
            delete_again_resp.status(),
            StatusCode::NOT_FOUND,
            "Expected not found error for deleting non-existent organization"
        );
    }
}
