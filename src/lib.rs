extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, parse_macro_input};
use syn::{ItemFn, LitStr, Result, Token, parse::Parse, parse::ParseStream};

/// Derives the `Module` trait for unit structs.
///
/// This macro adds implementation of the `Module` trait for the annotated struct.
/// It uses a companion struct to store the module state and tools.
///
/// # Attributes
/// - `#[module(name = "Module Name", desc = "Module Description")]`
///
/// # Example
/// ```
/// #[derive(Module)]
/// #[module(name = "Test Module", desc = "A test module")]
/// struct TestModule;
/// ```
#[proc_macro_derive(Module, attributes(module))]
pub fn derive_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract struct name and check if it's a unit struct
    let struct_name = &input.ident;
    let state_struct_name = format_ident!("{}State", struct_name);

    let is_unit_struct = match &input.data {
        Data::Struct(data) => matches!(data.fields, Fields::Unit),
        _ => false,
    };

    if !is_unit_struct {
        return TokenStream::from(
            syn::Error::new(
                input.ident.span(),
                "Module can only be derived for unit structs",
            )
            .to_compile_error(),
        );
    }

    // Extract module name and description from attributes
    let mut module_name = None;
    let mut module_desc = None;

    for attr in &input.attrs {
        if attr.path().is_ident("module") {
            match &attr.meta {
                Meta::List(meta_list) => {
                    let nested_meta = meta_list.parse_args_with(
                        syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                    );

                    if let Ok(nested) = nested_meta {
                        for meta in nested {
                            if let Meta::NameValue(name_value) = meta {
                                if name_value.path.is_ident("name") {
                                    if let Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit_str),
                                        ..
                                    }) = &name_value.value
                                    {
                                        module_name = Some(lit_str.value());
                                    }
                                }
                                if name_value.path.is_ident("desc") {
                                    if let Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit_str),
                                        ..
                                    }) = &name_value.value
                                    {
                                        module_desc = Some(lit_str.value());
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Default values if not provided
    let module_name = module_name.unwrap_or_else(|| format!("{} Module", struct_name));
    let module_desc = module_desc.unwrap_or_else(|| format!("Description for {}", struct_name));

    // Generate the implementation
    let expanded = quote! {
        // Create a state struct to hold the required fields
        struct #state_struct_name {
            tools: Vec<Box<dyn crate::cli::SafeTool>>,
            state: crate::cli::ModuleState,
        }

        // Thread-local storage to keep module state
        thread_local! {
            static MODULE_STATE: std::sync::Mutex<Option<#state_struct_name>> = std::sync::Mutex::new(None);
        }

        // Implementation of the Module trait
        impl crate::cli::Module for #struct_name {
            const MODULE_NAME: &'static str = #module_name;
            const MODULE_DESC: &'static str = #module_desc;
            type Output = ();
            type Error = std::io::Error; // Using a dummy error type

            fn init_module(tools: Vec<Box<dyn crate::cli::SafeTool>>) -> Self
            where
                Self: Sized,
            {
                // Initialize the module state
                MODULE_STATE.with(|cell| {
                    let mut state_guard = cell.lock().unwrap();
                    *state_guard = Some(#state_struct_name {
                        tools,
                        state: crate::cli::ModuleState::ToolSelect
                    });
                });

                Self {}
            }

            fn run_loop(&mut self) {
                loop {
                    // Get the current state (type and index if in a tool)
                    let current_state = MODULE_STATE.with(|cell| {
                        let state_guard = cell.lock().unwrap();
                        let state = state_guard.as_ref().expect("Module state not initialized");
                        state.state.clone() // Clone the state enum to avoid holding the lock
                    });

                    match current_state {
                        crate::cli::ModuleState::ToolSelect => {
                            Self::print_tools();
                            Self::select_tool();
                        }
                        crate::cli::ModuleState::InTool(tool_index) => {
                            // Important: Do all operations inside one with() call
                            MODULE_STATE.with(|cell| {
                                let mut state_guard = cell.lock().unwrap();
                                let state = state_guard.as_mut().expect("Module state not initialized");

                                // Access and run tool while holding the lock
                                let tool = &mut state.tools[tool_index];
                                tool.run_tool();

                                // Update state after tool execution
                                state.state = crate::cli::ModuleState::ToolSelect;
                            });
                        }
                        crate::cli::ModuleState::Quit => {
                            println!("Exiting Module");
                            break;
                        }
                    }
                }
            }

            fn print_tools(&self) {
                Self::print_tools();
            }

            fn select_tool(&mut self) {
                Self::select_tool();
            }
        }

        // Static methods to handle module state
        impl #struct_name {
            fn print_tools() {
                MODULE_STATE.with(|cell| {
                    let state_guard = cell.lock().unwrap();
                    let state = state_guard.as_ref().expect("Module state not initialized");

                    println!("Available Tools:");
                    println!("Input Q to quit");
                    for (index, tool) in state.tools.iter().enumerate() {
                        println!("{}. {}", index + 1, tool.name());
                        println!("  -{}", tool.desc());
                    }
                });
            }

            fn select_tool() {
                // Get input outside of any lock
                let input = crate::io::get_input("Select a tool");

                if crate::io::is_quit(&input) {
                    MODULE_STATE.with(|cell| {
                        let mut state_guard = cell.lock().unwrap();
                        let state = state_guard.as_mut().expect("Module state not initialized");
                        state.state = crate::cli::ModuleState::Quit;
                    });
                    return;
                }

                // First get the tool count
                let tool_count = MODULE_STATE.with(|cell| {
                    let state_guard = cell.lock().unwrap();
                    let state = state_guard.as_ref().expect("Module state not initialized");
                    state.tools.len()
                });

                // Then update the state based on selection
                match crate::io::select_index(&input, tool_count) {
                    Some(index) => {
                        MODULE_STATE.with(|cell| {
                            let mut state_guard = cell.lock().unwrap();
                            let state = state_guard.as_mut().expect("Module state not initialized");
                            state.state = crate::cli::ModuleState::InTool(index - 1);
                        });
                    }
                    None => println!("Invalid selection"),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

// Custom struct to parse the attribute arguments
struct ToolArgs {
    id: String,
    name: Option<String>,
    desc: Option<String>,
}

impl Parse for ToolArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut id = None;
        let mut name = None;
        let mut desc = None;

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            let _: Token![=] = input.parse()?;

            if key == "id" {
                let value: LitStr = input.parse()?;
                id = Some(value.value());
            } else if key == "name" {
                let value: LitStr = input.parse()?;
                name = Some(value.value());
            } else if key == "desc" {
                let value: LitStr = input.parse()?;
                desc = Some(value.value());
            } else {
                return Err(syn::Error::new(key.span(), "Unknown attribute"));
            }

            if !input.is_empty() {
                let _: Token![,] = input.parse()?;
            }
        }

        match id {
            Some(id_value) => Ok(ToolArgs {
                id: id_value,
                name,
                desc,
            }),
            None => Err(syn::Error::new(
                input.span(),
                "Missing required 'id' attribute",
            )),
        }
    }
}

/// Function attribute macro that creates a Tool struct and implements the Tool trait.
///
/// This macro should be applied to a function and requires an `id` attribute that specifies
/// the name of the struct to create. It also accepts optional `name` and `desc` attributes
/// for customizing the tool's name and description.
///
/// # Example
/// ```
/// #[derive_tool(id = "ListUsersTool", name = "List Users", desc = "Lists all users in the system")]
/// fn list_users() {
///     // Function implementation here
/// }
/// ```
///
/// The macro also supports functions with parameters:
/// ```
/// #[derive_tool(id = "GreetTool", name = "Greeting Tool")]
/// fn greet(name: &str, formal: bool) {
///     if formal {
///         println!("Good day, {}.", name);
///     } else {
///         println!("Hey, {}!", name);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn derive_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let args = parse_macro_input!(attr as ToolArgs);

    // Parse the function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function name and visibility
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;

    // Check if function has parameters and return an error if it does
    if !fn_inputs.is_empty() {
        return TokenStream::from(
            syn::Error::new(
                input_fn.sig.span(),
                "derive_tool can only be applied to functions with no parameters",
            )
            .to_compile_error(),
        );
    }

    // Create the struct identifier from the ID
    let struct_id = format_ident!("{}", args.id);

    // Default values if not provided
    let tool_name = args.name.unwrap_or_else(|| format!("{} Tool", fn_name));
    let tool_desc = args.desc.unwrap_or_else(|| format!("Tool to {}", fn_name));

    // Generate implementation for functions without parameters
    let expanded = quote! {
        // Keep the original function
        #input_fn

        // Create a new struct with the specified ID
        #fn_vis struct #struct_id;

        // Implement the Tool trait for the new struct
        impl crate::cli::Tool for #struct_id {
            const TOOL_NAME: &'static str = #tool_name;
            const TOOL_DESC: &'static str = #tool_desc;

            fn run_tool(&self) {
                // Call the original function
                #fn_name();
            }
        }
    };

    TokenStream::from(expanded)
}
