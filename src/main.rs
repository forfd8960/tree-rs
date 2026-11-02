use tree_rs::scan::scan_dir;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let args: Vec<String> = std::env::args()
        .map(|arg| arg.as_str().to_string())
        .collect();
    let root = &args[1];
    println!("root: {}", root);

    scan_dir(root)?;

    Ok(())
}
