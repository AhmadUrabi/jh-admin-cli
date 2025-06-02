#![allow(non_snake_case, dead_code)]
use jh_admin_cli_macros::Module;
use jh_admin_cli_macros::derive_tool;

use ldap3::LdapConn;

use ldap3::LdapConnSettings;
use ldap3::LdapError;
use ldap3::{Scope, SearchEntry};

#[derive(Module)]
#[module(name = "LDAP Management Module", desc = "Manage LDAP users and groups")]
pub struct LDAPModule;

pub fn create_ldap_connection() -> Result<LdapConn, LdapError> {
    let username = std::env::var("LDAP_USERNAME").expect("LDAP_USERNAME not set");
    let password = std::env::var("LDAP_PASSWORD").expect("LDAP_PASSWORD not set");
    let ldap_server = std::env::var("LDAP_SERVER").expect("LDAP_SERVER not set");
    // Establish a connection with the LDAP server
    let ldap_settings = LdapConnSettings::new();
    let mut conn = LdapConn::with_settings(ldap_settings, ldap_server.as_str()).unwrap();
    conn.simple_bind(username.as_str(), password.as_str())?;
    Ok(conn)
}

#[derive_tool(
    id = "FetchUsersTool",
    name = "Fetch Users",
    desc = "Lists all users in the system"
)]
pub fn fetch_all_users() {
    let mut ldap = create_ldap_connection().unwrap();
    let base_dn_string = std::env::var("BASE_DN").unwrap();
    let base_dn = base_dn_string.as_str();
    // Perform a search
    let (rs, _res) = ldap
        .search(
            base_dn,
            Scope::Subtree,
            "(objectClass=user)",
            vec!["*", "+"],
        )
        .unwrap()
        .success()
        .unwrap();

    // Iterate through search results and print them
    for entry in rs {
        let entry = SearchEntry::construct(entry);
        println!("{:?} - {:?}", entry.attrs.get("sAMAccountName"), entry.attrs.get("name"));
    }
}
