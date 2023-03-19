mod config {
    use std::collections::{hash_map, HashMap};

    pub enum Mode {
        Auto, // 完全自动推断，忽略白名单(likely)
        AutoFps, // 推断目标fps，准守白名单包名
        AutoGame, // 推断是否游戏，准守白名单fps
        Manual // 无自动推断，完全准守白名单
    }
    struct Config {
        mode: Mode,
        conf: HashMap<String, i32>
    }
    fn read_config() {
        use std::fs;
        use inotify::*;
        use std::sync::mpsc;

        let mut inot_config: Inotify = inotify::Inotify::init().expect("Err : Init inotify");
        inot_config.add_watch(
            "/data/FEAShelper.conf", WatchMask::MODIFY
        ).expect("Err : Failed to add inotify watch");
        loop {
            let mut buf: [u8;1024] = [0u8; 1024];
            let events = inot_config.read_events_blocking(&mut buf)
                .expect("Err : Watching config file");
            for event in events {
                if event.mask.contains(EventMask::MODIFY) {
                    let config = fs::read_to_string("/data/FEAShelper.conf")
                        .expect("Err : Read config");
                }
            }
        }
    }
    pub fn run() {
        use std::thread;
        thread::spawn(read_config);
    }
}
fn run () {
    
}