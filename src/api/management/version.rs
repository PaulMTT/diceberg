use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitConfig {
    pub branch: String,
    pub build: Build,
    pub closest: Closest,
    pub commit: Commit,
    pub dirty: String,
    pub local: Local,
    pub remote: Remote,
    pub tag: Option<String>,
    pub tags: Option<String>,
    pub total: Total,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub host: String,
    pub time: String,
    pub user: BuildUser,
    pub version: String,
    pub number: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildUser {
    pub email: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Closest {
    pub tag: Tag,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub commit: TagCommit,
    pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagCommit {
    pub count: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub author: Author,
    pub committer: Committer,
    pub id: CommitId,
    pub message: Message,
    pub time: String,
    pub user: CommitUser,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub time: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Committer {
    pub time: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommitId {
    pub abbrev: String,
    pub describe: String,
    pub describe_short: String,
    pub full: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub full: String,
    pub short: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommitUser {
    pub email: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Local {
    pub branch: LocalBranch,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalBranch {
    pub ahead: String,
    pub behind: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Remote {
    pub origin: RemoteOrigin,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoteOrigin {
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Total {
    pub commit: TotalCommit,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TotalCommit {
    pub count: String,
}
