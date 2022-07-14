use std::process::Command;
use std::collections::HashMap;
use std::env;

fn main() {
    let keys = HashMap::from([
            (" M ", "modified"),
            (" A ", "added"),
            (" D ", "deleted"),
            (" R ", "renamed"),
            (" C ", "copied"),
            ("?? ", "new file"),
    ]);

    let out = Command::new("git").args(["status", "--porcelain=v1"]).output().unwrap();
    let output = String::from_utf8(out.stdout).unwrap();

    for line in output.split('\n') {
        for (key, desc) in &keys {
            if line.starts_with(key) {
                let maybe_stripped = line.strip_prefix(key);
                let stripped = maybe_stripped.unwrap_or(line);

                let prefix_to_strip = env::args().nth(1).unwrap();

                println!("{}", desc);
                println!(
                    "{}\n",
                    stripped.strip_prefix(&prefix_to_strip).unwrap_or(stripped)
                );
            }
        }
    }
}
