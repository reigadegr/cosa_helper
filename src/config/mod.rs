pub mod format_profile;
use crate::utils::node_reader::{read_file, write_to_byte};
use compact_str::CompactString;
use serde::Deserialize;
extern crate alloc;
use alloc::boxed::Box;
use format_profile::format_toml;
use once_cell::sync::Lazy;

pub static PROFILE: Lazy<Config> = Lazy::new(|| {
    let profile_path = b"/data/adb/modules/freezer_rs/naughty_apps.toml\0";
    let profile = read_file::<65536>(profile_path).unwrap();
    let format_rs = format_toml(&profile);
    let profile: Config = toml::from_str(&profile).unwrap();
    write_to_byte(profile_path, format_rs.as_bytes()).unwrap();
    profile
});

#[derive(Deserialize)]
pub struct Config {
    pub policy: Box<[PackageConfigBean]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackageConfigBean {
    pub package_name: CompactString,
    pub cpu_config: CompactString,
    pub gpu_config: CompactString,
    pub gpa_config: CompactString,
    pub game_zone: CompactString,
    pub thermal_frame: CompactString,
    pub fps_stabilizer: CompactString,
    pub refresh_rate: CompactString,
    pub resv_8: CompactString,
    pub resv_13: CompactString,
    pub unity_game_boost: CompactString,
}
