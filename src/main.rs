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

use dotenv::dotenv;
use modules::ldap::*;
use cli::Module;
use cli::*;
use modules::user::UserModule;

fn main() {
    // Read environment variables from .env file
    dotenv().ok();
    // This will later be replaced with a function to initialize all modules
    let ldap_module = LDAPModule::init_module().unwrap();
    let users_module = UserModule::init_module().unwrap();
    let mut cli = CLI::new(vec![Box::new(ldap_module), Box::new(users_module)]);
    cli.run_loop();
}
