pub const DEBUG: bool = true;
pub const VERSION: &'static str = "2.1.0";
pub const AUTHOR: &'static str = "Olivier Thornton";
pub const LICENSE: &'static str =
    "GPLV3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.";
pub const CHECK_ARGS: [&'static str; 8] = [
    "-r",
    "-v",
    "-p",
    "--help",
    "--version",
    "--verbose",
    "--recursive",
    "--passes",
];
