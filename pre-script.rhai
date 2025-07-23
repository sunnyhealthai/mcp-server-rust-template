// Pre-generation script for cargo-generate
// This runs before the template is processed

fn init() {
    // Set default values if not provided
    if variable::get("crate_type") == "" {
        variable::set("crate_type", "bin");
    }
    
    // Set the current year for copyright
    let current_year = "2024"; // In a real scenario, this would be dynamic
    variable::set("copyright_year", current_year);
    
    // Log some information
    let server_name = variable::get("server_name");
    print("Setting up MCP Server template");
    print("Server name: " + server_name);
}