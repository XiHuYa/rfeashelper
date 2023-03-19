use std::{thread::sleep, time::Duration};
mod misc;

mod config {
    use std::collections::{hash_map, HashMap};

    use crate::ask;

    pub enum Mode {
        Auto, // 完全自动推断，忽略白名单(likely)
        AutoFps, // 推断目标fps，准守白名单包名
        AutoGame, // 推断是否游戏，准守白名单fps
        Manual // 无自动推断，完全准守白名单
    }
    pub struct App_config {
        isGame: bool,
        fps: i32
    }
    fn read_config() {
        
    }
    pub fn ask(app: &str) -> App_config {
        use std::fs;
        
        let mut isGame = false;
        let mut fps: i32 = 0;
        let mut mode = Mode::Auto;
        let config = fs::read_to_string("/data/FEAShelper.conf").expect("Err : Fail to read config");
        for line in config.lines() {
            if line.contains("Mode") {
                let mode_conf: Vec<&str> = line.split("=")
                    .collect();
                let mode_conf = mode_conf[1]
                    .trim();
                mode = match mode_conf {
                    "Auto" => Mode::Auto,
                    "AutoFps" => Mode::AutoFps,
                    "AutoGame" => Mode::AutoGame,
                    "Manual" => Mode::Manual,
                    _ => {panic!("Err : Failed to read mode");}
                }
            }
            if (line.contains(app)) {
                isGame = true;
                let app_conf: Vec<&str> = line.split(" ")
                    .collect();
                fps = app_conf[1].trim()
                    .parse()
                    .expect("Err : Failed to read fps");
            }
        }
        match mode {
            Mode::Auto => {
                isGame = ask::ask_isGame();
                fps = ask::ask_target_fps();
            }
            Mode::AutoFps => {
                fps = ask::ask_target_fps();
            }
            Mode::AutoGame => {
                isGame = ask::ask_isGame();
            }
            Mode::Manual => {}
        }
        App_config { isGame , fps }
    }
}

mod ask {
    use crate::misc::exec_cmd;

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
                topapp = line.split('{').collect();
            }
        }
        topapp
    }
    pub fn ask_isGame() -> bool {

    }
    pub fn ask_target_fps() -> i32 {

    }
}

fn run () {
    loop {
        sleep(Duration::from_secs(1));
    }
}