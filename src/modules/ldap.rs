use crate::cli::*;
use std::sync::{Arc, Mutex};

pub struct LDAPModule {
    conn: Arc<Mutex<ldap3::LdapConn>>,
    tools: Vec<Box<dyn SafeTool>>,
    selected_tool: Option<&Box<dyn SafeTool>>,
    state: ModuleState
}

impl LDAPModule {
    fn new(conn: Arc<Mutex<ldap3::LdapConn>>, tools: Vec<Box<dyn SafeTool>>) -> Self {
        LDAPModule { conn, tools, selected_tool: None, state: ModuleState::ToolSelect }
    }
    fn module_loop(&mut self) {
        loop {
            match self.state {
                ModuleState::ToolSelect => {
                    self.list_tools();
                },
                ModuleState::InTool => {},
                ModuleState::Quit => {
                    println!("Exiting {}", self.name());
                    break;
                }
            }

        }
    }
    fn list_tools(&self) {
        println!("Available Tools:");
        for (index, tool) in self.tools.iter().enumerate() {
            println!("{}. {}", index + 1, tool.name());
            println!("  -{}", tool.desc());
        }
        let input = get_input("Select a tool");

        match input.parse::<usize>() {
            Ok(index) => {
                if index > 0 && index <= self.tools.len() {
                    let selected_tool = self.tools.get(index - 1);
                    self.selected_tool = selected_tool;
                    self.state = ModuleState::InTool;
                } else {
                    println!("Invalid selection");
                }
            },
            Err(_) => println!("Invalid input")
        }
    }
}

impl Module for LDAPModule {
    const MODULE_NAME: &'static str = "LDAP Management Module";
    const MODULE_DESC: &'static str = "Manage LDAP users and groups";
    type Output = ();
    type Error = ldap3::LdapError;
    fn init_module() -> Result<Self, Self::Error> where Self: Sized {
        let addr = std::env::var("LDAP_ADDR").unwrap_or_else(|_| "ldap://localhost".to_string());
        let conn = ldap3::LdapConn::new(&addr)?;
        let conn_am = Arc::new(Mutex::new(conn));
        let list_users_tool = ListUsersTool::new(conn_am.clone());
        Ok(LDAPModule::new(conn_am, vec![Box::new(list_users_tool)]))
    }
    fn run_module(&self) -> Result<Self::Output, Self::Error> {
        println!("Running {}", Self::MODULE_NAME);
        Ok(())
    }
}

pub(crate) struct ListUsersTool(Arc<Mutex<ldap3::LdapConn>>);

impl ListUsersTool {
    fn new(conn: Arc<Mutex<ldap3::LdapConn>>) -> Self {
        ListUsersTool(conn)
    }
}

impl Tool for ListUsersTool {
    const TOOL_NAME: &'static str = "List Users Tool";
    const TOOL_DESC: &'static str = "Tool to list all LDAP Users";
    fn run_tool(&self) {
        println!("Running List Users Tool");
    }
}
