use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // Get the folder name (crate name) from the argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: doc_gen <crate_folder> <doc_folder>");
        std::process::exit(1);
    }

    let crate_folder = &args[1];
    let doc_folder = &args[2];
    let crate_path = Path::new(crate_folder);
    let doc_path = Path::new(doc_folder);

    // Ensure the crate folder exists
    if !crate_path.is_dir() {
        eprintln!("Error: {} is not a valid directory.", crate_folder);
        std::process::exit(1);
    }
    // Ensure the doc folder exists
    if !doc_path.is_dir() {
        eprintln!("Error: {} is not a valid directory.", doc_folder);
        std::process::exit(1);
    }

    // Define the path to the lib.rs file inside the crate
    let lib_rs_path = crate_path.join("src").join("lib.rs");
    if !lib_rs_path.exists() {
        eprintln!("Error: {} does not exist.", lib_rs_path.display());
        std::process::exit(1);
    }
    //crate name we are generating doc for
    let crate_name = crate_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    // Output folder under doc where folder would be created if necessary
    let output_path = doc_path.join(&crate_name);
    if !output_path.exists() {
        fs::create_dir_all(&output_path)?;
    }

    // Read the lib.rs file to generate the content for introduction.md
    let lib_rs_content = fs::read_to_string(lib_rs_path)?;
    let mut lines = lib_rs_content.lines();

    // Extract the title and description
    let title = lines
        .next()
        .unwrap_or("")
        .replace("#", "")
        .trim()
        .to_string();
    let description: Vec<String> = lines
        .clone()
        .take_while(|line| line.starts_with("##"))
        .map(|line| line.trim_start_matches("##").trim().to_string())
        .collect();

    // Create the introduction.md content
    let mut introduction_md_content = format!("# {}\n\n", title);
    for desc in description {
        //introduction_md_content.push_str(&format!("{}\n\n", desc));
        introduction_md_content.push_str(&format!("{}", desc));
    }

    introduction_md_content.push_str("```rust,ignore\n");
    for line in lines {
        introduction_md_content.push_str(&format!("{}\n", line));
    }
    introduction_md_content.push_str("```\n\n");
    introduction_md_content.push_str(&format!(
        "[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/{}/src/lib.rs)\n",
        crate_name
    ));

    // Write the introduction.md file (overwrite if it already exists)
    let introduction_md_path = output_path.join("introduction.md");
    fs::write(introduction_md_path, introduction_md_content)?;
    // Update the SUMMARY.md file in the project-level src directory
    let summary_md_path = doc_path.join("SUMMARY.md");
    let mut summary_md_content = fs::read_to_string(&summary_md_path)?;

    // Add a new line with the link to the introduction.md if it's not already present
    let summary_entry = format!("* [{}](./{}/introduction.md)", title, crate_name);
    if !summary_md_content.contains(&summary_entry) {
        summary_md_content.push_str(&summary_entry);
        fs::write(summary_md_path, summary_md_content)?;
    }

    println!("Successfully generated introduction.md and updated SUMMARY.md.");
    Ok(())
}
