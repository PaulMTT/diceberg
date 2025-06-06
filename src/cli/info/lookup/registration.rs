use crate::api::management::client::DiciManagementClient;
use anyhow::Context;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum RegistrationLookupType {
    All,
    Path(PathArgs),
}

#[derive(Args)]
pub struct PathArgs {
    path: String,
}

pub async fn handle_lookup_registration(
    registration_lookup_type: RegistrationLookupType,
) -> anyhow::Result<()> {
    match registration_lookup_type {
        RegistrationLookupType::All => {
            let dici_management_client = DiciManagementClient::default();
            let registrations = dici_management_client.fetch_registrations().await?;
            serde_json::to_writer_pretty(std::io::stdout(), &registrations)
                .context("failed to serialize registrations")
        }
        RegistrationLookupType::Path(args) => {
            let dici_management_client = DiciManagementClient::default();
            let registrations = dici_management_client.fetch_registrations_by_path(args.path).await?;
            serde_json::to_writer_pretty(std::io::stdout(), &registrations)
                .context("failed to serialize registrations")
        }
    }
}
