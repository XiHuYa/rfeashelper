use std::{thread::sleep, time::Duration};

use config::AppConfig;
mod misc;

mod config {
    use crate::{ask, misc::cut};

    pub enum Mode {
        Auto,     // 完全自动推断，忽略白名单(likely)
        AutoFps,  // 推断目标fps，准守白名单包名
        AutoGame, // 推断是否游戏，准守白名单fps
        Manual,   // 无自动推断，完全准守白名单
    }
    pub struct AppConfig(pub bool, pub u64);

    pub fn ask(app: &str) -> AppConfig {
        use std::fs;

        let mut is_game = false;
        let mut fps: u64 = 0;
        let mut mode = Mode::Auto;
        let config =
            fs::read_to_string("/data/FEAShelper.conf").expect("Err : Fail to read config");
        for line in config.lines() {
            let first = line.chars().next();
            if let Some('#') = first {
                continue;
            }
            if line.contains("Mode") {
                let mode_conf = cut(line, "=", 1);
                mode = match &mode_conf[..] {
                    "Auto" => Mode::Auto,
                    "AutoFps" => Mode::AutoFps,
                    "AutoGame" => Mode::AutoGame,
                    "Manual" => Mode::Manual,
                    _ => {
                        panic!("Err : Failed to read mode");
                    }
                }
            }
            // println!("{}, {}", &line, app.len());
            if line.contains(&app) {
                if line.contains("[B]") {
                    return AppConfig(false, 0);
                }
                is_game = true;
                let app_cut = cut(line, "=", 1);
                let app_cut: Vec<&str> = app_cut.split_whitespace().collect();
                let mut app_conf: Vec<u64> = Vec::new();
                for s in app_cut {
                    match s.parse() {
                        Ok(o) => {
                            app_conf.push(o);
                        }
                        Err(e) => {}
                    }
                }
                fps = ask::ask_target_fps_conf(&app_conf);
                return AppConfig(is_game, fps);
            }
        }
        match mode {
            Mode::Auto => {
                is_game = ask::ask_is_game();
                fps = ask::ask_target_fps();
            }
            Mode::AutoFps => {
                fps = ask::ask_target_fps();
            }
            Mode::AutoGame => {
                is_game = ask::ask_is_game();
            }
            Mode::Manual => {}
        }
        AppConfig(is_game, fps)
    }
}

mod ask {
    use std::{
        thread::sleep,
        time::{self, Duration},
    };

    use crate::misc::{cut, exec_cmd};

    pub fn ask_top_app() -> String {
        /*use std::path::Path;
        use std::fs;*/

        let mut topapp = String::new();
        /*if Path::new("/sys/kernel/gbe/gbe2_fg_pid").exists() {
            let pid = fs::read_to_string("/sys/kernel/gbe/gbe2_fg_pid")
                .expect("Err : Fail to read pid")
                .trim()
                .to_string();
            topapp = fs::read_to_string(format!("/proc/{}/cmdline", pid))
                .expect("Err : Fail to read cmdline")
                .trim()
                .to_string();
            return topapp;
        }*/
        let dump_top = exec_cmd("dumpsys", &["activity", "activities"])
            .expect("Err : Failed to dumpsys for Topapp");
        for line in dump_top.lines() {
            if line.contains("topResumedActivity=") {
                topapp = cut(&line, "{", 1);
                topapp = cut(&topapp, "/", 0);
                topapp = cut(&topapp, " ", 2);
            }
        }
        topapp
    }
    pub fn ask_is_game() -> bool {
        let current_surface_view = exec_cmd("dumpsys", &["SurfaceFlinger", "--list"])
            .expect("Err : Failed to execute dumpsys SurfaceView");
        for line in current_surface_view.lines() {
            if line.contains("SurfaceView[") && line.contains("BLAST") {
                return true;
            } else if line.contains("SurfaceView -") {
                return true;
            }
        }
        return false;
    }

    fn get_current_fps() -> u64 {
        let mut current_fps = exec_cmd("service", &["call", "SurfaceFlinger", "1013"])
            .expect("Err : Failed to dump fps");
        current_fps = cut(&current_fps, "(", 1);
        current_fps = cut(&current_fps, "\'", 0);
        let frame_a = u64::from_str_radix(&current_fps, 16).unwrap();
        let time_a = time::SystemTime::now();
        sleep(Duration::from_millis(100));
        current_fps = exec_cmd("service", &["call", "SurfaceFlinger", "1013"])
            .expect("Err : Failed to dump fps");
        current_fps = cut(&current_fps, "(", 1);
        current_fps = cut(&current_fps, "\'", 0);
        let frame_b = u64::from_str_radix(&current_fps, 16).unwrap();
        let time_b = time::SystemTime::now();
        let result = (frame_b - frame_a) * 1000 * 1000 * 1000
            / (time_b.duration_since(time_a).unwrap().as_nanos() as u64);
        return result;
    }

    pub fn ask_target_fps() -> u64 {
        let fps = get_current_fps();
        const FPS: [u64; 6] = [30, 45, 60, 90, 120, 144];
        let mut i = 1;
        while i < (FPS.len() - 1) {
            if fps > (FPS[i] + 5) && fps < FPS[i + 1] {
                return FPS[i];
            }
            i += 1;
        }
        *FPS.last().unwrap()
    }
    pub fn ask_target_fps_conf(FPS: &Vec<u64>) -> u64 {
        let fps = get_current_fps();
        if FPS.is_empty() {
            return 0;
        }
        let mut i = 0;
        while i < FPS.len() - 1 {
            if i != 0 {
                if FPS[i - 1] + 5 < fps && FPS[i] + 5 > fps {
                    return FPS[i];
                }
            }
            i += 1;
        }
        FPS[0]
    }
}

mod process_feas {
    pub struct feas_sysfs {
        path: String,
        newer_feas: bool,
    }

    impl feas_sysfs {
        pub fn init() -> feas_sysfs {
            use std::path::Path;
            use std::process::exit;
            let test_file = |x: &str| (Path::new(x).exists());
            let path: String;
            let newer_feas: bool;
            if test_file("/sys/module/bocchi_perfmgr/parameters/perfmgr_enable") {
                // 56 fas
                path = String::from("/sys/module/bocchi_perfmgr/parameters/");
            } else if test_file("/sys/module/perfmgr/parameters/perfmgr_enable") {
                // qcom feas
                path = String::from("/sys/module/perfmgr/parameters/");
            } else if test_file("/sys/module/perfmgr_policy/parameters/perfmgr_enable") {
                // super old qcom feas
                path = String::from("/sys/module/perfmgr_policy/parameters/");
            } else if test_file("/sys/module/mtk_fpsgo/parameters/") {
                // mtk feas
                path = String::from("/sys/module/mtk_fpsgo/parameters/");
            } else {
                eprintln!("不支持的设备!");
                exit(-1);
            }
            if test_file(&format!("{}target_fps_61", path)) {
                // new feas
                newer_feas = true;
            } else {
                newer_feas = false;
            }
            feas_sysfs { path, newer_feas }
        }

        pub fn goes(&self, switch: bool, fps: u64) {
            use crate::misc::write_file;
            let sw_path = format!("{}perfmgr_enable", self.path);
            let fps_path = format!("{}fixed_target_fps", self.path);
            if switch {
                write_file("1", &sw_path);
                write_file(&fps.to_string(), &fps_path);
            } else {
                write_file("0", &sw_path);
            }
            if self.newer_feas {
                let path_61 = format!("{}target_fps_61", self.path);
                let path_91 = format!("{}target_fps_91", self.path);
                let path_121 = format!("{}target_fps_121", self.path);
                if fps <= 65 {
                    write_file("1", &path_61);
                    write_file("0", &path_91);
                    write_file("0", &path_121);
                } else if fps <= 95 {
                    write_file("0", &path_61);
                    write_file("1", &path_91);
                    write_file("0", &path_121);
                } else if fps < 144 {
                    write_file("0", &path_61);
                    write_file("0", &path_91);
                    write_file("1", &path_121)
                }
            }
        }
    }
}

pub fn run() {
    use crate::process_feas::feas_sysfs;
    let feas_sysfs = feas_sysfs::init();
    loop {
        sleep(Duration::from_secs(1));
        let AppConfig(switch, fps) = config::ask(&ask::ask_top_app());
        feas_sysfs.goes(switch, fps);
    }
}
