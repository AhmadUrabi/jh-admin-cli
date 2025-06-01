/// JH Admin CLI
///
/// Author: Ahmad Urabi
///
/// The purpose of this tool is to provide a command-line interface for managing JH Admin tasks.
///
/// Tasks include: Monitoring Systems, Reading Logs, Configuration Management, User Management, and more.
///
/// Modules:
/// - User Management:
///     - Manage LDAP Users
///     - Manage JHApp Users
/// - Email Management:
///     - Create Signature
///     - Add Email User
/// - Monitoring:
///     - VM Status
///     - Server Status
/// - Cisco:
///     - Phone Provisioning
///     - Reliability Testing
use dotenv::dotenv;
mod cli;
mod modules;
fn main() {
    // Read environment variables from .env file
    dotenv().ok();
}
