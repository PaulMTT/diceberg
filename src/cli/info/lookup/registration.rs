use crate::api::management::client::ManagementClient;
use anyhow::Context;
use clap::{Args, Subcommand};
use std::collections::HashMap;

#[derive(Subcommand, Clone)]
pub enum RegistrationLookupCommand {
    /// All registrations
    All,
    /// Registrations that are within a path
    Path(PathArgs),
    /// Registrations that are within a path and filtered by metadata key-value
    Filtered(MetadataArgs),
}
#[derive(Args, Clone)]
pub struct MetadataArgs {
    /// The s3 path of the registration
    path: String,
    /// Pairs of key-value metadata to filter registrations by
    pairs: Vec<String>,
}

#[derive(Args, Clone)]
pub struct PathArgs {
    /// The s3 path of the registration
    path: String,
}

pub async fn handle_lookup_registration(
    registration_lookup_command: RegistrationLookupCommand,
) -> anyhow::Result<()> {
    let dici_management_client = ManagementClient::default();
    let registrations = match registration_lookup_command {
        RegistrationLookupCommand::All => dici_management_client.fetch_registrations().await?,
        RegistrationLookupCommand::Path(args) => {
            dici_management_client
                .fetch_registrations_by_path(args.path)
                .await?
        }
        RegistrationLookupCommand::Filtered(MetadataArgs { path, pairs }) => {
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
