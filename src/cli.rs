use crate::io::{get_input, is_quit, select_index};

pub struct CLI {
    modules: Vec<Box<dyn SafeModule>>,
    state: CLIState,
}

enum CLIState {
    ModSelect,
    InModule(usize), // Stores the index of the selected module
    Quit,
}

#[derive(Clone)]
pub enum ModuleState {
    ToolSelect,
    InTool(usize), // Stores the index of the selected tool
    Quit,
}

impl CLI {
    pub fn new(modules: Vec<Box<dyn SafeModule>>) -> Self {
        println!("Initializing CLI...");
        Self {
            modules,
            state: CLIState::ModSelect,
        }
    }

    pub fn run_loop(&mut self) {
        loop {
            print!("{}[2J", 27 as char);
            match self.state {
                CLIState::ModSelect => {
                    self.print_modules();
                    self.select_module();
                }
                CLIState::InModule(module_index) => {
                    let module = &mut self.modules[module_index];
                    module.run_module();
                    self.state = CLIState::ModSelect;
                }
                CLIState::Quit => {
                    println!("Exiting CLI...");
                    break;
                }
            }
        }
    }

    pub fn print_modules(&self) {
        println!("Available Modules:");
        println!("Input Q to quit");
        for (index, module) in self.modules.iter().enumerate() {
            println!("{}. {}", index + 1, module.name());
            println!("  -{}", module.desc());
        }
    }

    pub fn select_module(&mut self) {
        let input = get_input("Select a module");
        if is_quit(&input) {
            self.state = CLIState::Quit;
            return;
        }

        match select_index(&input, self.modules.len()) {
            Some(index) => {
                self.state = CLIState::InModule(index - 1);
            }
            None => println!("Invalid selection"),
        }
    }
}

pub trait SafeModule {
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn run_module(&mut self);
}

impl<T: ?Sized + Module> SafeModule for T {
    fn name(&self) -> &'static str {
        <T as Module>::MODULE_NAME
    }
    fn desc(&self) -> &'static str {
        <T as Module>::MODULE_DESC
    }
    fn run_module(&mut self) {
        let _ = <T as Module>::run_loop(self);
    }
}

pub trait Module: SafeModule {
    const MODULE_NAME: &'static str;
    const MODULE_DESC: &'static str;
    type Output;
    type Error;

    // TODO: Create derive macros
    fn init_module(tools: Vec<Box<dyn SafeTool>>) -> Self
    where
        Self: Sized;
    fn run_loop(&mut self);
    fn print_tools(&self);
    fn select_tool(&mut self);
}

pub trait SafeTool {
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn run_tool(&self);
}

impl<T: ?Sized + Tool> SafeTool for T {
    fn name(&self) -> &'static str {
        <T as Tool>::TOOL_NAME
    }
    fn desc(&self) -> &'static str {
        <T as Tool>::TOOL_DESC
    }
    fn run_tool(&self) {
        <T as Tool>::run_tool(&self);
    }
}

pub trait Tool: SafeTool {
    const TOOL_NAME: &'static str;
    const TOOL_DESC: &'static str;
    fn run_tool(&self);
}
