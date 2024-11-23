use std::{env, error::Error, fs::{self, OpenOptions}, io::Write, path::PathBuf};

fn getpath_from_id(problem_id: String) -> PathBuf {
    let current = env::current_dir().unwrap();

    let mut path = PathBuf::new();
    path.push(current);
    path.push(".atcoder-judger");
    path.push(problem_id);

    path
}

fn write_file(dirname: &PathBuf, filename: String, contents: String) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(dirname)?;
    let mut file = OpenOptions::new().write(true).create(true).open(dirname.join(filename))?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn export(contest_id: String, problem_id: String, samplecase: Vec<String>) {
    println!("Exporting {} {}...", contest_id, problem_id);

    let base_path = getpath_from_id(problem_id);
    let input_path = base_path.join("in");
    let output_path = base_path.join("out");

    for i in 0..samplecase.len() / 2 {
        let input = samplecase[i * 2].clone();
        let output = samplecase[i * 2 + 1].clone();

        write_file(&input_path, format!("{:04}.txt", i), input).unwrap();
        write_file(&output_path, format!("{:04}.txt", i), output).unwrap();
    }
}
