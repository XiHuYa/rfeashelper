mod config {
    pub enum Mode {
        Auto, // 完全自动推断，忽略白名单(likely)
        Auto_fps, // 推断目标fps，准守白名单包名
        Auto_game, // 推断是否游戏，准守白名单fps
        Manual // 无自动推断，完全准守白名单
    }
    struct single_game(String, i32); // 一个单独的游戏的配置
    struct Config {
        mode: Mod,
        
    }
    fn read_config() {
        use std::fs;
        let config = fs::read_into_string("");
    }
    pub fn run() {
        use std::thread;
        thread::spawn(read_config);
    }
}
fn run () {
    
}