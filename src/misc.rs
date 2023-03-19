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