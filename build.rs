use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let problems_dir = Path::new("src/problems");

    println!("cargo:rerun-if-changed=src/problems/");

    let mut entries: Vec<(usize, String)> = fs::read_dir(problems_dir)
        .expect("failed to read src/problems/")
        .filter_map(|e| {
            let name = e.ok()?.file_name().into_string().ok()?;
            let stem = name.strip_suffix(".rs")?;
            let digits = stem.strip_prefix('p')?;
            let num = digits.parse::<usize>().ok()?;
            Some((num, stem.to_string()))
        })
        .collect();

    entries.sort_by_key(|(n, _)| *n);

    update_mod_rs(&entries);

    let dest = Path::new(&out_dir).join("problems_generated.rs");
    let mut f = fs::File::create(&dest).expect("failed to create problems_generated.rs");

    writeln!(f, "pub static PROBLEMS: LazyLock<HashMap<usize, ProblemInfo>> = LazyLock::new(|| {{").unwrap();
    writeln!(f, "    let mut problems = HashMap::new();").unwrap();

    for (num, module) in &entries {
        writeln!(
            f,
            "    problems.insert({num}, ProblemInfo {{ solver: {module}::main, status: {module}::STATUS }});"
        )
        .unwrap();
    }

    writeln!(f, "    problems").unwrap();
    writeln!(f, "}});").unwrap();
}

fn update_mod_rs(entries: &[(usize, String)]) {
    let mod_path = Path::new("src/problems/mod.rs");
    let content = fs::read_to_string(mod_path).expect("failed to read mod.rs");

    let mut current_mods: Vec<usize> = content
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("pub mod p")?;
            rest.strip_suffix(';')?.parse::<usize>().ok()
        })
        .collect();
    current_mods.sort();

    let new_mods: Vec<usize> = entries.iter().map(|(n, _)| *n).collect();

    if current_mods == new_mods {
        return;
    }

    // Replace the pub mod p* block with the new sorted set
    let mod_declarations: String = entries
        .iter()
        .map(|(_, module)| format!("pub mod {module};\n"))
        .collect();

    // Will be the new full mod.rs content
    let mut result = String::new();
    let mut mod_block_written = false;
    let mut in_mod_block = false;

    for line in content.lines() {
        if line.starts_with("pub mod p") {
            if !mod_block_written {
                result.push_str(&mod_declarations);
                mod_block_written = true;
            }
            in_mod_block = true;
        } else {
            in_mod_block = false;
            result.push_str(line);
            result.push('\n');
        }
    }
    let _ = in_mod_block;

    fs::write(mod_path, result).expect("failed to write mod.rs");
}
