use obfstr::obfstr;
use std::sync::LazyLock;
// Your backend ip and port
pub static BACKEND_URL: LazyLock<String> = LazyLock::new(|| {
    obfstr!("http://127.0.0.1:3551").to_string()
});
// Discord bot client id for RPC
pub const DISCORD_CLIENT_ID: u64 = 000000000;
// Enable/disable login verification
pub const ENABLE_LOGIN_API: bool = false;
//login api endpoint
pub static BACKEND_LOGIN_API: LazyLock<String> = LazyLock::new(||{
    obfstr!("http://127.0.0.1:3551/api/launcher/login").to_string()
});
// Set from 1-42
pub const FORTNITE_SEASON: i32 = 0;
// Enable/disable only joinable version
pub const ENABLE_VERSION_ONLY: bool = false;
// What version only is permitted
pub const FORTNITE_VERSION_ONLY: &str = "00.00";
// Enable/disable downloads in library
pub const ENABLE_DOWNLOADABLE_VERSION: bool = false;
// Url link for the version
pub static FORTNITE_DOWNLOADABLE_VERSION: LazyLock<String> = LazyLock::new(||{
    obfstr!("https://example.com/download.zip").to_string()
});
