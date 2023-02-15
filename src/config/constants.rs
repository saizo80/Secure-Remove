pub const DEBUG: bool = false;
const VERSION: &str = "2.2.2";
const AUTHOR: &str = "Olivier Thornton";
const LICENSE: &str = "GPLV3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.";

pub fn version() {
    println!("srm v{}", VERSION);
    println!("License {}", LICENSE);
    println!("This is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.");
    println!("\nWritten by {}.", AUTHOR);
}

pub fn help() {
    println!("Usage: srm [OPTION]... FILE...");
    println!("Securely remove files or directories.\n");
    println!("  -r, --recursive\tremove directories and their contents recursively");
    println!("  -p, --passes\t\tset the number of passes (default: 10)");
    println!("  -v, --verbose\t\texplain what is being done");
    println!("      --help\t\tdisplay this help and exit");
    println!("      --version\t\toutput version information and exit");
}
