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
mod cli;
mod modules;
mod io;
mod models;

use dotenv::dotenv;
use modules::email::ListEmailUsers;
use modules::ldap::*;
use cli::Module;
use cli::*;

fn main() {
    // Read environment variables from .env file
    dotenv().ok();
    let tool = FetchUsersTool;
    let email = ListEmailUsers;
    // Initialize the LDAP module with all tools
    let ldap_module = LDAPModule::init_module(vec![Box::new(tool), Box::new(email)]);

    // Create a new CLI with our modules
    let mut cli = CLI::new(vec![Box::new(ldap_module)]);
    cli.run_loop();
}
