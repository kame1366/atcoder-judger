use std::{env, error::Error, fs::OpenOptions, io::Write, path::PathBuf};

fn getpath_from_id(contest_id: String, problem_id: String) -> PathBuf {
    let home = env::var("HOME").unwrap();

    let mut path = PathBuf::new();
    path.push(home);
    path.push(".atcoder-judger");
    path.push(contest_id);
    path.push(problem_id);

    path
}

fn write_file(path: &PathBuf, text: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;

    file.write_all(text.as_bytes())?;

    Ok(())
}

pub fn export(contest_id: String, problem_id: String, samplecase: Vec<String>) {
    println!("Exporting...");

    let path = getpath_from_id(contest_id, problem_id);

    for i in 0..samplecase.len() / 2 {
        let input = samplecase[i * 2].clone();
        let output = samplecase[i * 2 + 1].clone();

        write_file(&path.join(format!("in/{:04}.txt", i)), input).unwrap();
        write_file(&path.join(format!("out/{:04}.txt", i)), output).unwrap();
    }
}
