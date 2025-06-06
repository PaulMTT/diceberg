use crate::api::management::client::DiciManagementClient;
use anyhow::Context;
use clap::{Args, Subcommand};
use std::collections::HashMap;

#[derive(Subcommand)]
pub enum RegistrationLookupType {
    All,
    Path(PathArgs),
    Filtered(MetadataArgs),
}
#[derive(Args)]
pub struct MetadataArgs {
    path: String,
    pairs: Vec<String>,
}

#[derive(Args)]
pub struct PathArgs {
    path: String,
}

pub async fn handle_lookup_registration(
    registration_lookup_type: RegistrationLookupType,
) -> anyhow::Result<()> {
    let dici_management_client = DiciManagementClient::default();
    let registrations = match registration_lookup_type {
        RegistrationLookupType::All => dici_management_client.fetch_registrations().await?,
        RegistrationLookupType::Path(args) => {
            dici_management_client
                .fetch_registrations_by_path(args.path)
                .await?
        }
        RegistrationLookupType::Filtered(MetadataArgs { path, pairs }) => {
            let metadata: HashMap<String, String> = pairs
                .chunks_exact(2)
                .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
                .collect();
            dici_management_client
                .fetch_registrations_by_path_and_metadata(path, metadata)
                .await?
        }
    };
    serde_json::to_writer_pretty(std::io::stdout(), &registrations)
        .context("failed to serialize registrations")
}
