pub fn bound_to_little() {
    let cpu0 = std::fs::read_to_string("/sys/devices/system/cpu/cpufreq/policy0/related_cpus")
    .unwrap_or_else(|e| {
        eprintln!("{}", e);
        String::new()
    });
    
    let cpu0 :Vec<&str> = cpu0.split_whitespace().collect();
    let cpu0 :Vec<usize> = cpu0
        .iter()
        .map(|s| s.trim().parse().unwrap())
        .collect();
    affinity::set_thread_affinity(&cpu0).unwrap();
}

pub fn exec_cmd(command :&str, args :&[&str]) -> Result<String, i32> {
    use std::process::Command;
    let output = Command::new(command)
        .args(args)
        .output();
    
    match output {
        Ok(o) => {
            Ok(String::from_utf8(o.stdout).expect("utf8 error"))
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(-1)
        }
    }
}

pub fn cut(str: &str, sym: &str, f: usize) -> String {
    let fs: Vec<&str> = str.split(sym).collect();
    match fs.get(f) {
        Some(s) => s.trim()
            .to_string(),
        None => String::new()
    }
}

pub fn write_file(content: &str, path: &str) {
    use std::fs::{OpenOptions, set_permissions};
    use std::io::Write;
    use std::os::unix::fs::{PermissionsExt};
    // println!("path: {}, value: {}", &content, &path);
    match set_permissions(path, PermissionsExt::from_mode(0o644)) {
        Ok(()) => {
            match OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path) {
                    Ok(mut file) => {
                        match file.write_all(content.as_bytes()) {
                            Ok(()) => {}
                            Err(e) => println!("Write failed: {}", e),
                        }
                    },
                    Err(e) => println!("Open failed: {}", e),
                }
        },
        Err(e) => println!("Set permissions failed: {}", e),
    }
}