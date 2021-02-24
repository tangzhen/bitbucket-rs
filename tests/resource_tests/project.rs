use crate::common;
use httpmock::{MockServer, Method::{GET, POST, PUT, DELETE}};
use bitbucket_rs::{
    resources::ProjectResource,
    models::{
        get::BitbucketErrors,
        post,
    },
};
use serde_json::json;

#[tokio::test]
async fn get_existing_project_works() -> common::Result {
    let server = MockServer::start_async().await;
    let client = common::make_client(&server);
    let resource = ProjectResource::new(&client);
    let project = "test_project";

    let json_project = r#"
    {
        "key": "test_project",
        "id": 0,
        "name": "The cool test project",
        "description": "The test project",
        "public": true,
        "type": "NORMAL",
        "links": {
            "self": [
                {
                    "href": "http://stash.test.com/projects/test_project"
                }
            ]
        }
    }"#;

    let expected_project = serde_json::from_str(json_project)?;

    server.mock(|when, then| {
        let path = format!("projects/{}", &project);

        when.method(GET)
            .path(common::format_path(&path));
        then.status(200)
            .header(common::CONTENT_TYPE, common::CONTENT_TYPE_JSON)
            .body(json_project);
    });

    let project = resource.get_project(&project).await?;
    assert_eq!(project, expected_project);

    Ok(())
}

#[tokio::test]
async fn get_all_projects_works() -> common::Result {
    let server = MockServer::start_async().await;
    let client = common::make_client(&server);
    let resource = ProjectResource::new(&client);

    let json_first_project = r#"
    {
        "key": "Project1",
        "id": 1,
        "name": "The first project",
        "public": true,
        "type": "NORMAL",
        "links": {
            "self": [
                {
                    "href": "http://stash.test.com/project_1"
                }
            ]
        }
    }"#;

    let json_first_page = format!(r#"
    {{
        "size": 1,
        "limit": 25,
        "isLastPage": false,
        "values": [
            {}
        ],
        "start": 0,
        "nextPageStart": 1
    }}"#, json_first_project);

    let json_second_project = r#"{
        "key": "Project2",
        "id": 2,
        "name": "The second project",
        "public": false,
        "type": "NORMAL",
        "links": {
            "self": [
                {
                    "href": "http://stash.test.com/project_2"
                }
            ]
        }
    }"#;

    let json_second_page = format!(r#"
    {{
        "size": 1,
        "limit": 25,
        "isLastPage": true,
        "values": [
            {}
        ],
        "start": 1
    }}"#, json_second_project);

    let path = common::format_path("projects");

    server.mock(|when, then| {
        when.method(GET)
            .path(&path)
            .query_param_exists("start");
        then.status(200)
            .body(json_second_page);
    });

    server.mock(|when, then| {
        when.method(GET)
            .path(&path);
        then.status(200)
            .body(json_first_page);
    });

    let expected_first_project = serde_json::from_str(&json_first_project)?;
    let expected_second_project = serde_json::from_str(&json_second_project)?;

    let projects = resource.get_all_projects().await?;

    assert_eq!(projects[0], expected_first_project);
    assert_eq!(projects[1], expected_second_project);

    Ok(())
}

#[tokio::test]
async fn create_project_works() -> common::Result {
    let server = MockServer::start_async().await;
    let client = common::make_client(&server);
    let resource = ProjectResource::new(&client);

    let project = post::Project {
        key: "PTP".to_owned(),
        name: "Post-Test-Project".to_owned(),
        description: Some("This is a test project, please ignore".to_owned()),
        avatar: None,
    };

    let expected_json_project = r#"
    {
        "key": "PTP",
        "id": 1,
        "name": "Post-Test-Project",
        "description": "This is a test project, please ignore",
        "public": true,
        "type": "NORMAL",
        "links": {
            "self": [
                {
                    "href": "http://stash.test.com/projects/Post-Test-Project"
                }
            ]
        }
    }"#;

    server.mock(|when, then| {
        when.method(POST)
            .path(common::format_path("projects"));
        then.status(201)
            .body(expected_json_project);
    });

    let expected_project = serde_json::from_str(expected_json_project)?;
    let new_project = resource.create_project(&project).await?;

    assert_eq!(new_project, expected_project);

    Ok(())
}

#[tokio::test]
async fn get_non_existent_project_returns_error() -> common::Result {
    let server = MockServer::start_async().await;
    let client = common::make_client(&server);
    let resource = ProjectResource::new(&client);

    let json_errors = r#"
    {
        "errors": [
            {
                "context": null,
                "message": "Project non_existent was not found",
                "exceptionName": null
            }
        ]
    }
    "#;

    server.mock(|when, then| {
        when.method(GET)
            .path(common::format_path("projects/non_existent"));
        then.status(404)
            .body(json_errors);
    });

    let expected_errors: BitbucketErrors = serde_json::from_str(json_errors)?;
    let resp = resource.get_project("non_existent").await;
    assert!(resp.is_err());

    let errors = resp.as_ref()
        .unwrap_err()
        .downcast_ref::<BitbucketErrors>()
        .unwrap();

    assert_eq!(expected_errors, *errors);

    Ok(())
}

#[tokio::test]
async fn update_project_works() -> common::Result {
    let server = MockServer::start_async().await;
    let client = common::make_client(&server);
    let resource = ProjectResource::new(&client);
    let project = "test";
    let path = common::format_path(&format!("projects/{}", project));

    let mut json_project = json!(
    {
        "key": "EP",
        "id": 0,
        "name": "Existent project",
        "description": "The project to update",
        "public": true,
        "type": "NORMAL",
        "links": {
            "self": [
                {
                    "href": "http://stash.test.com/projects/test_project"
                }
            ]
        }
    });

    let existing_json_project = json_project.to_string();
    let existing_expected_project = serde_json::from_str(&existing_json_project)?;

    server.mock(|when, then| {
        when.method(GET)
            .path(&path);
        then.status(200)
            .body(existing_json_project);
    });

    *json_project.get_mut("key").unwrap() = json!("EPN");
    let updated_json_project = json_project.to_string();
    let updated_expected_project = serde_json::from_str(&updated_json_project)?;

    server.mock(|when, then| {
        when.method(PUT)
            .path(&path);
        then.status(200)
            .body(updated_json_project);
    });

    let existing_project = resource.get_project(project).await?;
    assert_eq!(existing_project, existing_expected_project);

    let new_project = post::Project {
        key: "EPN".to_string(),
        name: existing_project.name,
        description: existing_project.description,
        avatar: None,
    };

    let updated_project = resource.update_project(project, &new_project).await?;
    assert_eq!(updated_project, updated_expected_project);

    Ok(())
}

#[tokio::test]
async fn delete_project_works() -> common::Result {
    let server = MockServer::start_async().await;
    let client = common::make_client(&server);
    let resource = ProjectResource::new(&client);

    server.mock(|when, then| {
        when.method(DELETE)
            .path(common::format_path("projects/test"));
        then.status(204);
    });

    let res = resource.delete_project("test").await;
    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[tokio::test]
async fn failed_to_delete_returns_error() -> common::Result {
    let server = MockServer::start_async().await;
    let client = common::make_client(&server);
    let resource = ProjectResource::new(&client);

    let errors = r#"
    {
        "errors": [
            {
                "context": null,
                "message": "You don't have permission to delete this project",
                "exceptionName": null
            }
        ]
    }
    "#;

    server.mock(|when, then| {
        when.method(DELETE)
            .path(common::format_path("projects/test"));
        then.status(401)
            .body(errors);
    });

    let expected_errors = serde_json::from_str(errors)?;
    let resp = resource.delete_project("test").await;
    assert!(resp.is_err());

    let errors = resp
        .as_ref()
        .unwrap_err()
        .downcast_ref::<BitbucketErrors>()
        .unwrap();

    assert_eq!(*errors, expected_errors);

    Ok(())
}