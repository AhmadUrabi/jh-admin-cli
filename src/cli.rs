use std::io::{stdin, stdout, Write};

pub struct CLI {
    modules: Vec<Box<dyn SafeModule>>,
    selected_module: Option<&Box<dyn SafeModule>>,
    state: CLIState,
}

enum CLIState {
    ModSelect,
    InModule,
    Quit
}

pub enum ModuleState {
    ToolSelect,
    InTool,
    Quit
}

fn get_input(message: &str) -> String {
    print!("{}: ", message);
    let mut buffer = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}

impl CLI {
    pub fn new(modules: Vec<Box<dyn SafeModule>>) -> Self {
        println!("Initializing CLI...");
        Self {
            modules,
            selected_module: None,
            state: CLIState::ModSelect
        }
    }

    pub fn run_loop(&mut self) {
        loop {
            match self.state {
                CLIState::ModSelect => {
                    self.list_modules();
                },
                CLIState::InModule => {
                    self.selected_module.as_ref().unwrap().run_module();
                    self.state = CLIState::ModSelect;
                },
                CLIState::Quit => {
                    println!("Exiting CLI...");
                    break;
                }
            }
        }
    }

    pub fn list_modules(&mut self) {
        println!("Available Modules:");
        println!("Input Q to quit");
        for (index, module) in self.modules.iter().enumerate() {
            println!("{}. {}", index + 1, module.name());
            println!("  -{}", module.desc());
        }
        let input = get_input("Select a module");
        if input == "q" || input == "Q" {
            self.state = CLIState::Quit;
            return;
        }
        match input.parse::<usize>() {
            Ok(index) => {
                if index > 0 && index <= self.modules.len() {
                    let selected_module = self.modules.get(index - 1);
                    self.selected_module = selected_module;
                    self.state = CLIState::InModule;
                } else {
                    println!("Invalid selection");
                }
            },
            Err(_) => println!("Invalid input")
        }
    }
}

pub trait SafeModule {
    fn name(&self) -> &'static str;
    fn desc(&self) -> &'static str;
    fn run_module(&self);
}

impl<T: ?Sized + Module> SafeModule for T {
    fn name(&self) -> &'static str {
        <T as Module>::MODULE_NAME
    }
    fn desc(&self) -> &'static str {
        <T as Module>::MODULE_DESC
    }
    fn run_module(&self) {
        let _ = <T as Module>::run_module(&self);
    }
}

pub trait Module: SafeModule {
    const MODULE_NAME: &'static str;
    const MODULE_DESC: &'static str;
    type Output;
    type Error;

    fn init_module() -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn run_module(&self) -> Result<Self::Output, Self::Error>;
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
