// Pre-generation script for cargo-generate
// This runs before the template is processed

fn init() {
    // Set default values if not provided
    if variable::get("crate_type") == "" {
        variable::set("crate_type", "bin");
    }
    
    // Derive the crate name from project name
    let project_name = variable::get("project-name");
    let crate_name = project_name.to_lower().replace("-", "_");
    variable::set("crate_name", crate_name);
    
    // Set the current year for copyright
    let current_year = "2024"; // In a real scenario, this would be dynamic
    variable::set("copyright_year", current_year);
    
    // Log some information
    print("Setting up MCP Server template for: " + project_name);
    print("Crate name will be: " + crate_name);
}