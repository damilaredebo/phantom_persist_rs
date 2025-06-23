use phantom_persist_rs;

fn main() {
    /* C example
    	if (FAILED(RegisterApplicationRestart(NULL, 0))) {
		printf("Failed to register application restart\n");
		return 1;
	} */
    phantom_persist_rs::register_application_restart();
    println!("[+] Registered application restart");
    println!("[+] Sleeping 60 seconds to ensure registration");
    std::thread::sleep(std::time::Duration::from_secs(60));
    println!("[+] Starting message loop thread. Go ahead shutdown/restart.");
    phantom_persist_rs::message_loop_thread();
}
