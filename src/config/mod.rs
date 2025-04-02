pub mod format_profile;
use crate::utils::node_reader::{read_file, write_to_byte};
use serde::Deserialize;
extern crate alloc;
use format_profile::format_toml;
use hashbrown::HashSet;
use once_cell::sync::Lazy;

pub static PROFILE: Lazy<Config> = Lazy::new(|| {
    let profile_path = b"/data/adb/modules/cosa_helper/cosa_apps.toml\0";
    let profile = read_file::<65536>(profile_path).unwrap();
    let format_rs = format_toml(&profile);
    let profile: Config = toml::from_str(&profile).unwrap();
    write_to_byte(profile_path, format_rs.as_bytes()).unwrap();
    profile
});

#[derive(Deserialize)]
pub struct Config {
    pub policy: HashSet<PackageConfigBean>,
}

#[derive(Debug, Clone, Deserialize, Eq, Hash, PartialEq)]
pub struct PackageConfigBean {
    pub package_name: String,
    pub cpu_config: String,
    pub gpu_config: String,
    pub gpa_config: String,
    pub game_zone: String,
    pub thermal_frame: String,
    pub fps_stabilizer: String,
    pub refresh_rate: String,
    pub resv_8: String,
    pub resv_13: String,
    pub unity_game_boost: String,
}
