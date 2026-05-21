use std::panic;

// ======================================
// TABFORGE ENTRY
// ======================================

fn main() {
    panic::set_hook(Box::new(|info| {
        println!();

        println!("========== PANIC ==========");

        println!("[FATAL] {}", info);

        println!("===========================");

        println!();
    }));

    println!();

    println!("================================");

    println!("[TABFORGE] Desktop Runtime");

    println!("[TABFORGE] Starting...");

    println!("================================");

    println!();

    desktop_app_lib::run();

    println!("[TABFORGE] Shutdown");
}
