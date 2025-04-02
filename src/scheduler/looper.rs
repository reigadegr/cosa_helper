use crate::{
    activity::{ActivityUtils, get_tid_info::get_process_name},
    config::PROFILE,
    utils::sleep::sleep_secs,
};
use compact_str::CompactString;
use libc::pid_t;
use likely_stable::unlikely;
use log::info;
// unsafe extern "C" {
// fn __llvm_profile_write_file() -> i32;
// }
const sqlite3_path: &str = "/data/adb/modules/fas_ext/sqlite3";
const db_path: &str = "/data/data/com.oplus.cosa/databases/db_game_database";

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
                    info!("开始执行SQL\n");
                    let cmd =
                        format!("{sqlite3_path} {db_path} << EOF\nUPDATE [PackageConfigBean]");
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
