#![allow(unused_imports)]

use crate::common;
use httpmock::{MockServer, Method::{GET, POST, PUT, DELETE}};
use bitbucket_rs::{
    models::{
        get::BitbucketErrors,
        post,
    },
};

fn get_repo() -> &'static str {
    r#"
    {
        "slug": "my-repo",
        "id": 1,
        "name": "My repo",
        "scmId": "git",
        "state": "AVAILABLE",
        "statusMessage": "Available",
        "forkable": true,
        "project": {
            "key": "PRJ",
            "id": 1,
            "name": "My Cool Project",
            "description": "The description for my cool project.",
            "public": true,
            "type": "NORMAL",
            "link": {
                "url": "http://link/to/project",
                "rel": "self"
            },
            "links": {
                "self": [
                    {
                        "href": "http://link/to/project"
                    }
                ]
            }
        },
        "public": true,
        "cloneUrl": "https://<baseURL>/scm/PRJ/my-repo.git",
        "link": {
            "url": "http://link/to/repository",
            "rel": "self"
        },
        "links": {
            "clone": [
                {
                    "href": "https://<baseURL>/scm/PRJ/my-repo.git",
                    "name": "http"
                },
                {
                    "href": "ssh://git@<baseURL>/PRJ/my-repo.git",
                    "name": "ssh"
                }
            ],
            "self": [
                {
                    "href": "http://link/to/repository"
                }
            ]
        }
    }"#
}

#[tokio::test]
async fn get_all_repositories_works() -> common::Result {
    let ctx = context!(RepositoryResource, "PRJ");

    let json_repo = get_repo();
    let json_page = format!(r#"
    {{
        "size": 1,
        "limit": 25,
        "isLastPage": true,
        "values": [
            {}
        ],
        "start": 0
    }}"#, json_repo);

    let expected_repo = serde_json::from_str(json_repo)?;

    let path = common::format_path("projects/PRJ/repos");

    ctx.server().mock(|when, then| {
        when.method(GET)
            .path(&path);
        then.status(200)
            .body(&json_page);
    });

    let repos = ctx.resource().get_all_repositories().await?;
    assert_eq!(repos.len(), 1);
    assert_eq!(repos[0], expected_repo);

    Ok(())
}


#[tokio::test]
async fn get_existing_repo_works() -> common::Result {
    let ctx = context!(RepositoryResource, "PRJ");
    let json_repo = get_repo();
    let expected_repo = serde_json::from_str(json_repo)?;
    let path = common::format_path("projects/PRJ/repos/test");

    ctx.server().mock(|when, then| {
        when.method(GET)
            .path(&path);
        then.status(200)
            .body(json_repo);
    });

    let repo = ctx.resource().get_repository("test").await?;
    assert_eq!(repo, expected_repo);

    Ok(())
}