// Prevents an extra console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // MCP clients that only speak stdio (Claude Desktop, Codex, ...) launch this
    // binary like a plain command; no window, just a relay to the running app.
    if std::env::args().any(|a| a == "--mcp-stdio") {
        #[cfg(target_os = "windows")]
        unsafe {
            // In release mode on Windows, windows_subsystem = "windows" detaches standard console handles.
            // Attach to the parent console (calling process) so stdin/stdout work properly.
            extern "system" {
                fn AttachConsole(dwProcessId: u32) -> i32;
            }
            const ATTACH_PARENT_PROCESS: u32 = 0xFFFFFFFF;
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
        std::process::exit(spritsit_lib::mcp_stdio_bridge());
    }
    spritsit_lib::run()
}
