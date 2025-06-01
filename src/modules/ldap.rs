use crate::cli::*;

use std::sync::{Arc, Mutex};

struct LDAPModule {
    conn: Arc<Mutex<ldap3::LdapConn>>,
}

impl LDAPModule {
    fn new(conn: Arc<Mutex<ldap3::LdapConn>>) -> Self {
        LDAPModule { conn }
    }
}

impl Module for LDAPModule {
    const MODULE_NAME: &'static str = "LDAP Management Module";
    const TOOLS: &'static [Box<dyn SafeTool>] = &[];
    type Output = ();
    type Error = ldap3::LdapError;
    fn init_module() -> Result<Self, Self::Error> where Self: Sized {
        todo!()
    }
    fn run_module(&self) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}
