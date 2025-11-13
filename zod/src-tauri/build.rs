use tauri_typegen::BuildSystem;

fn main() {
    // Tell cargo to rerun this build script if any Rust source files change
    println!("cargo:rerun-if-changed=src");

    // Generate TypeScript bindings from Tauri commands
    BuildSystem::generate_at_build_time().expect("Failed to generate TypeScript bindings");

    tauri_build::build()
}
