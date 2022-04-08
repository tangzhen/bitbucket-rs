use crate::common;
use bitbucket_rs::models::post;
use httpmock::{Method::POST, MockServer};

#[tokio::test]
async fn create_pull_request_works() -> common::Result {
    let ctx = context!(PullRequestResource, "my-project", "my-repo");

    let pull_request = post::PullRequest {
        title: "PR-title".to_owned(),
        description: Some("PR-description".to_owned()),
        from_ref: post::PullRequestRef {
            id: "featureBranch".to_owned(),
            repository: post::PullRequestRefRepo {
                slug: "my-repo".to_owned(),
                name: None,
                project: post::PullRequestRefRepoProject {
                    key: "my-project".to_owned(),
                },
            },
        },
        to_ref: post::PullRequestRef {
            id: "master".to_owned(),
            repository: post::PullRequestRefRepo {
                slug: "my-repo".to_owned(),
                name: None,
                project: post::PullRequestRefRepoProject {
                    key: "my-project".to_owned(),
                },
            },
        },
        close_source_branch: false,
        reviewers: Vec::new(),
    };

    let expected_json_pull_request = r#"
    {
        "id": 1,
        "version": 2,
        "title": "PR-title",
        "description": "PR-description",
        "createdDate": 1649346771,
        "updatedDate": 1649346776,
        "reviewers": [],
        "participants": [],
        "fromRef": {
            "id": "featureBranch",
            "repository": {
                "slug": "my-repo",
                "project": {
                    "key": "my-project"
                }
            }
        },
        "toRef": {
            "id": "master",
            "repository": {
                "slug": "my-repo",
                "project": {
                    "key": "my-project"
                }
            }
        },
        "links": {
            "self": [
                {
                    "href": "http://stash.test.com/projects/Post-Test-Project"
                }
            ]
        }
    }"#;

    let path = common::format_path("projects/my-project/repos/my-repo/pull-requests");

    ctx.server().mock(|when, then| {
        when.method(POST).path(&path);
        then.status(201).body(expected_json_pull_request);
    });

    let expected_pull_request = serde_json::from_str(expected_json_pull_request)?;
    let new_pull_request = ctx.resource().create_pull_request(&pull_request).await?;

    assert_eq!(new_pull_request, expected_pull_request);

    Ok(())
}
