struct CLI(Vec<Box<dyn SafeModule>>);

pub trait SafeModule {
    fn name(&self) -> &'static str;
    fn tools(&self) -> &'static [Box<dyn SafeTool>];
}

impl<T: ?Sized + Module> SafeModule for T {
    fn name(&self) -> &'static str {
        <T as Module>::MODULE_NAME
    }
    fn tools(&self) -> &'static [Box<dyn SafeTool>] {
        <T as Module>::TOOLS
    }
}

pub trait Module: SafeModule {
    const MODULE_NAME: &'static str;
    const TOOLS: &'static [Box<dyn SafeTool>];
    type Output;
    type Error;
    fn init_module() -> Result<Self, Self::Error> where Self: Sized;
    fn run_module(&self) -> Result<Self::Output, Self::Error>;
}

pub trait SafeTool {
    fn name(&self) -> &'static str;
}

impl<T: ?Sized + Tool> SafeTool for T {
    fn name(&self) -> &'static str {
        <T as Tool>::TOOL_NAME
    }
}

pub trait Tool: SafeTool {
    const TOOL_NAME: &'static str;
}
