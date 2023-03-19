use std::{thread::sleep, time::Duration};

use config::App_config;
mod misc;

mod config {
    use crate::{ask, misc::cut};

    pub enum Mode {
        Auto, // 完全自动推断，忽略白名单(likely)
        AutoFps, // 推断目标fps，准守白名单包名
        AutoGame, // 推断是否游戏，准守白名单fps
        Manual // 无自动推断，完全准守白名单
    }
    pub struct App_config (pub bool, pub i32);

    pub fn ask(app: &str) -> App_config {
        use std::fs;
        
        let mut isGame = false;
        let mut fps: i32 = 0;
        let mut mode = Mode::Auto;
        let config = fs::read_to_string("/data/FEAShelper.conf").expect("Err : Fail to read config");
        for line in config.lines() {
            if line.contains("Mode") {
                let mode_conf = cut(line, "=", 1);
                mode = match &mode_conf[..] {
                    "Auto" => Mode::Auto,
                    "AutoFps" => Mode::AutoFps,
                    "AutoGame" => Mode::AutoGame,
                    "Manual" => Mode::Manual,
                    _ => {panic!("Err : Failed to read mode");}
                }
            }
            if (line.contains(app)) {
                isGame = true;
                let app_conf = cut(line, "=", 1);
                fps = app_conf.parse()
                    .expect("Err : Failed to read fps");
            }
        }
        match mode {
            Mode::Auto => {
                isGame = ask::ask_isGame(app);
                fps = ask::ask_target_fps();
            }
            Mode::AutoFps => {
                fps = ask::ask_target_fps();
            }
            Mode::AutoGame => {
                isGame = ask::ask_isGame(app);
            }
            Mode::Manual => {}
        }
        App_config ( isGame , fps )
    }
}

mod ask {
    use std::{thread::sleep, time::{Duration, self}};

    use crate::misc::{exec_cmd, cut};

    pub fn ask_topApp() -> String {
        use std::path::Path;
        use std::fs;

        let mut topapp = String::new();
        if Path::new("/sys/kernel/gbe/gbe2_fg_pid").exists() {
            let mut pid = fs::read_to_string("/sys/kernel/gbe/gbe2_fg_pid")
                .expect("Err : Fail to read pid")
                .trim();
            topapp = fs::read_to_string(format!("/proc/{}/cmdline", pid))
                .expect("Err : Fail to read cmdline")
                .trim()
                .to_string();
            return topapp;
        }
        let dump_top = exec_cmd("dumpsys", &["activity", "activities"])
            .expect("Err : Failed to dumpsys for Topapp");
        for line in dump_top.lines() {
            if line.contains("topResumedActivit=") {
                topapp = cut(&line, "{", 1);
                topapp = cut(&topapp, "/", 0);
                topapp = cut(&topapp, " ", 2);
            }
        }
        topapp
    }
    pub fn ask_isGame(app: &str) -> bool {
        let mut current_surface_view = exec_cmd("dumpsys", &["SurfaceFlinger", "--list"])
            .expect("Err : Failed to execute dumpsys SurfaceView");
        for line in current_surface_view.lines() {
            if line.contains("SurfaceView[") && line.contains("BLAST") {
                let current_surface_view = line;
                return current_surface_view.contains(app);
            } else if line.contains("SurfaceView -") {
                let current_surface_view = line;
                return current_surface_view.contains(app);
            }
        }
        return false;
    }
    fn get_current_fps() -> u64 {
        let mut current_fps = exec_cmd("service", &["call", "SurfaceFlinger", "1013"])
            .expect("Err : Failed to dump fps");

        current_fps = cut(&current_fps, "(", 1);
        current_fps = cut(&current_fps, "\'", 0);

        let frame_A = u64::from_str_radix(&current_fps, 16)
            .unwrap();

        let timeA = time::SystemTime::now();

        sleep(Duration::from_millis(100));

        current_fps = exec_cmd("service", &["call", "SurfaceFlinger", "1013"])
            .expect("Err : Failed to dump fps");

        current_fps = cut(&current_fps, "(", 1);
        current_fps = cut(&current_fps, "\'", 0);

        let frame_B = u64::from_str_radix(&current_fps, 16)
            .unwrap();

        let timeB = time::SystemTime::now();
        (frame_B - frame_A) / (timeB.duration_since(timeA)
            .unwrap()
            .as_secs();)
    }
    pub fn ask_target_fps() -> u64 {
        let fps = get_current_fps();
    }
}

fn run () {
    loop {
        sleep(Duration::from_secs(1));
        let App_config(on, fps) = config::ask(&ask::ask_topApp());
    }
}