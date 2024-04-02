use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;

#[derive(Debug)]
#[allow(dead_code)]
struct SearchResult {
    query: String,
    results: Vec<String>,
}

fn search_file(name: String) -> SearchResult {
    let ps_child = Command::new("find")
        .args(&[".", "-name", &name])
        .stdout(Stdio::piped())
        .output()
        .expect("Could not spawn command.");

    let results = String::from_utf8_lossy(&ps_child.stdout);
    let result_rows: Vec<String> = results
        .split("\n")
        .map(|e| e.to_string())
        .filter(|s| s.len() > 1)
        .collect();

    SearchResult {
        query: name,
        results: result_rows,
    }
}

fn process_roundtrip() -> String {
    let mut cat_child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not spawn process.");

    let stdin = cat_child.stdin.as_mut().expect("Could not open stdin.");

    stdin
        .write_all(b"datadatadata")
        .expect("Could not write to child process.");

    String::from_utf8(
        cat_child
            .wait_with_output()
            .expect("Something went wrong!")
            .stdout
            .as_slice()
            .iter()
            .cloned()
            .collect(),
    )
    .unwrap()
}

fn main() {
    let search_thread = thread::spawn(|| {
        println!(
            "Using 'find' to search for '*.rs': {:?}",
            search_file("*.rs".to_owned())
        );
    });

    let process_thread = thread::spawn(|| {
        println!("Reading from /bin/cat > {:?}", process_roundtrip());
    });

    search_thread.join().unwrap();
    process_thread.join().unwrap();
}
