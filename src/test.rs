use std::{path, process::{Command, Stdio}};

fn run_script(problem_id: String) {
    let input_path = path::PathBuf::from(format!(".atcoder-judger\\{problem_id}\\in"));
    let input_files = input_path.read_dir().unwrap();

    for files_entry in input_files {
        let file = files_entry.unwrap().path();

        let _compile = Command::new("g++")
            .args(["-o", "main", "main.cpp"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();

        let output = Command::new("./main")
            .args(["<", &file.into_os_string().into_string().unwrap()])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

pub fn test(problem_id: String) {
    println!("I will test {}...", problem_id);

    run_script(problem_id);
}
