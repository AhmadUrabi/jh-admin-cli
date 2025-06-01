use crate::cli::{Module, SafeTool};

pub struct UserModule;

impl Module for UserModule {
    const MODULE_NAME: &'static str = "User Management Module";
    const MODULE_DESC: &'static str = "Manage Users";
    // const TOOLS: &'static [Box<dyn SafeTool>] = &[];
    type Output = ();
    type Error = ();

    fn init_module() -> Result<Self, Self::Error>
    where
        Self: Sized {
            Ok(UserModule)
    }

    fn run_module(&self) -> Result<Self::Output, Self::Error> {
        println!("Running {}", Self::MODULE_NAME);
        Ok(())
    }
}
