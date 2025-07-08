use const_format::formatcp;

/// API only for staff and teachers.
pub const APIP_URL: &str = "https://apip.ecoledirecte.com";

/// API for everyone else - students, parents, ...
pub const API_URL: &str = "https://api.ecoledirecte.com";

pub const API_VERSION: &str = "7.2.3";
pub const USER_AGENT: &str = formatcp!("Android EDMOBILE v{API_VERSION}");
