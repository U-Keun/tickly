fn main() {
    // Load .env file from project root (parent of src-tauri)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = std::path::Path::new(&manifest_dir).parent().unwrap();
    let env_path = project_root.join(".env");

    if env_path.exists() {
        dotenvy::from_path(&env_path).ok();
    }

    // Pass Supabase config as compile-time environment variables
    if let Ok(url) = std::env::var("SUPABASE_URL") {
        println!("cargo:rustc-env=SUPABASE_URL={}", url);
    }
    if let Ok(key) = std::env::var("SUPABASE_ANON_KEY") {
        println!("cargo:rustc-env=SUPABASE_ANON_KEY={}", key);
    }

    // Rebuild if .env changes
    println!("cargo:rerun-if-changed=../.env");

    tauri_build::build()
}
