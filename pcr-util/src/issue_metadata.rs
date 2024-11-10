// Each issue metadata returned from `gh` cli looks like:
/*
{
    "assignees": [
        {
            "id": "MDQ6VXNlcjM2NzQzMTQ=",
            "login": "compiler-errors",
            "name": "Michael Goulet"
        }
    ],
    "author": {
        "id": "MDQ6VXNlcjUyOTYyNTM0",
        "is_bot": false,
        "login": "wxie7",
        "name": ""
    },
    "createdAt": "2024-11-06T02:24:31Z",
    "labels": [
        {
            "id": "MDU6TGFiZWwyMzU3OTE=",
            "name": "A-diagnostics",
            "description": "Area: Messages for errors, warnings, and lints",
            "color": "f7e101"
        },
        {
            "id": "MDU6TGFiZWwyMDM0MjkyMDA=",
            "name": "P-high",
            "description": "High priority",
            "color": "eb6420"
        },
        {
            "id": "MDU6TGFiZWwyMTE2NjgxMDA=",
            "name": "T-compiler",
            "description": "Relevant to the compiler team, which will review and decide on the PR/issue.",
            "color": "bfd4f2"
        },
        {
            "id": "MDU6TGFiZWwyNjIyNTI4NDA=",
            "name": "regression-from-stable-to-stable",
            "description": "Performance or correctness regression from one stable version to another.",
            "color": "e4008a"
        },
        {
            "id": "MDU6TGFiZWw2NTA3MzE2NjM=",
            "name": "C-bug",
            "description": "Category: This is a bug.",
            "color": "f5f1fd"
        },
        {
            "id": "MDU6TGFiZWwxMTY4MDI5NTU1",
            "name": "I-hang",
            "description": "Issue: The compiler never terminates, due to infinite loops, deadlock, livelock, etc.",
            "color": "e10c02"
        },
        {
            "id": "LA_kwDOAAsO6M7pwtW9",
            "name": "S-has-mcve",
            "description": "Status: A Minimal Complete and Verifiable Example has been found for this issue",
            "color": "862eff"
        }
    ],
    "number": 132673,
    "title": "Hang after encountering overflow errors for huge types ",
    "updatedAt": "2024-11-08T05:00:16Z",
    "url": "https://github.com/rust-lang/rust/issues/132673"
},
*/

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Response format.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueMetadataRepr {
    pub assignees: BTreeSet<Assignee>,
    pub author: Author,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub labels: BTreeSet<Label>,
    pub number: u64,
    pub title: String,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Assignee {
    pub login: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Author {
    pub login: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label {
    pub name: String,
}

/// Our convenient format.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IssueMetadata {
    pub assignees: Vec<String>,
    pub author: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub labels: Vec<String>,
    pub number: u64,
    pub title: String,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub url: String,
}
