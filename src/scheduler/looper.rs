use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name},
    config::{PROFILE, PackageConfigBean},
    utils::sleep::sleep_secs,
};
use anyhow::Result;
use compact_str::CompactString;
use libc::pid_t;
use likely_stable::unlikely;
use log::info;
use rusqlite::{Connection, params};
// unsafe extern "C" {
// fn __llvm_profile_write_file() -> i32;
// }
// const sqlite3_path: &str = "/data/adb/modules/fas_ext/sqlite3";
const DB_PATH: &str = "/data/data/com.oplus.cosa/databases/db_game_database";

pub struct Looper {
    pub activity_utils: ActivityUtils,
    pub global_package: CompactString,
    pub pid: pid_t,
}

impl Looper {
    pub fn new(activity_utils: ActivityUtils) -> Self {
        Self {
            activity_utils,
            global_package: CompactString::new(""),
            pid: -1,
        }
    }

    fn wait_until_exit(&mut self) {
        loop {
            sleep_secs(1);
            let pid = self.activity_utils.top_app_utils.get_top_pid();
            if unlikely(pid != self.pid) {
                self.game_exit();
                return;
            }
        }
    }

    const fn game_exit(&mut self) {
        self.pid = -1;
    }

    pub fn enter_loop(&mut self) {
        'outer: loop {
            sleep_secs(1);
            {
                let pid = self.activity_utils.top_app_utils.get_top_pid();
                if self.pid == pid {
                    continue 'outer;
                }
                self.pid = pid;
                let name = get_process_name(pid).unwrap_or_default();
                self.global_package = name;
            }

            for i in &PROFILE.policy {
                if self.global_package == i.package_name {
                    info!("开始为{0}执行SQL\n", self.global_package);
                    let pkg_cfg = PackageConfigBean {
                        cpu_config: i.cpu_config.clone(),
                        gpu_config: i.gpu_config.clone(),
                        gpa_config: i.gpa_config.clone(),
                        game_zone: i.game_zone.clone(),
                        thermal_frame: i.thermal_frame.clone(),
                        fps_stabilizer: i.fps_stabilizer.clone(),
                        refresh_rate: i.refresh_rate.clone(),
                        resv_8: i.resv_8.clone(),
                        resv_13: i.resv_13.clone(),
                        unity_game_boost: i.unity_game_boost.clone(),
                        package_name: i.package_name.clone(),
                    };

                    let rs = update_package_config(&pkg_cfg);
                    info!("完毕");
                    self.wait_until_exit();
                    continue 'outer;
                }
            }
            // unsafe {
            // __llvm_profile_write_file();
            // }
        }
    }
}

pub fn update_package_config(pkg_cfg: &PackageConfigBean) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    // 遍历 PROFILE.policy，找到匹配的配置
    let cmd = "
            UPDATE PackageConfigBean
            SET 
                cpu_config = ?,
                gpu_config = ?,
                gpa_config = ?,
                game_zone = ?,
                thermal_frame = ?,
                fps_stabilizer = ?,
                refresh_rate = ?,
                resv_8 = ?,
                resv_13 = ?,
                unity_game_boost = ?
            WHERE PackageConfigBean.package_name = ?;
        ";

    // 使用参数化查询，避免 SQL 注入
    conn.execute(
        cmd,
        params![
            pkg_cfg.cpu_config,
            pkg_cfg.gpu_config,
            pkg_cfg.gpa_config,
            pkg_cfg.game_zone,
            pkg_cfg.thermal_frame,
            pkg_cfg.fps_stabilizer,
            pkg_cfg.refresh_rate,
            pkg_cfg.resv_8,
            pkg_cfg.resv_13,
            pkg_cfg.unity_game_boost,
            pkg_cfg.package_name
        ],
    )?;
    Ok(())
}
