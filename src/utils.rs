use std::path::{Path, PathBuf};

pub fn format_path(path: &String, as_template: bool) -> String {
    // Check if the path has a file extension
    if let Some(extension) = Path::new(path).extension() {
        // Convert the extension to a string for comparison
        let extension_str = extension.to_str().unwrap_or("");

        // If as_template is true and the extension is not "md", return an error
        if as_template && extension_str != "md" {
            panic!("Error: Templates must have a .md file extension.");
        }
    } else {
        // If no extension is present, append ".md" if as_template is true
        if as_template {
            return format!("{}.md", path);
        }
    }

    // If the path does not end with .md and no other extension is present, append ".md"
    if !path.ends_with(".md") {
        return format!("{}.md", path);
    }

    // Return the path as-is if it already ends with .md
    path.clone()
}

pub fn get_file_name_from_path(path: &String) -> Option<String> {
    let path_buf = PathBuf::from(path);

    // Get the file name as an Option<&OsStr>
    match path_buf.file_name() {
        Some(name) => name.to_str().map(String::from), // Convert to String
        None => None,                                  // Return None if there is no file name
    }
}
