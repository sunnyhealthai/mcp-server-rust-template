// Post-generation script for cargo-generate
// This runs after the template is processed

fn init() {
    // cargo-generate provides the project-name automatically
    let project_name = variable::get("crate_name");
    let server_name = variable::get("server_name");
    
    print("====================================");
    print("MCP Server template generated successfully!");
    print("====================================");
    print("");
    print("Project: " + project_name);
    print("Server Name: " + server_name);
    print("");
    print("Next steps:");
    print("1. cd " + project_name);
    print("2. cargo make install-deps  # Install dependencies");
    print("3. cargo make dev          # Start development server");
    print("4. cargo make test         # Run tests");
    print("");
    print("For more tasks, run: cargo make --list-all-steps");
    print("");
    print("Check TEMPLATE_CONFIG.md for customization guide");
}