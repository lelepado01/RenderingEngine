use std::fs;
use std::path::Path;

fn load_shader_with_includes(path: &str, base_path: &str) -> String {
    let shader_code = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to load shader: {}", path));

    let mut processed_code = String::new();

    for line in shader_code.lines() {
        if line.trim().starts_with("#include") {
            // Extract the file path from the include directive
            let include_path = line.split_whitespace().nth(1)
                .expect("Invalid #include syntax")
                .replace("\"", "") // Remove quotes
                .replace(";", ""); 

            let include_file_path = Path::new(base_path).join(include_path);

            let included_code = fs::read_to_string(&include_file_path)
                .unwrap_or_else(|_| panic!("Failed to include shader: {:?}", include_file_path));

            processed_code.push_str(&included_code);
        } else {
            processed_code.push_str(line);
            processed_code.push('\n');
        }
    }

    processed_code
}

fn preprocess_shader(input_path: &str, output_path: &str) {
    let processed_shader = load_shader_with_includes(input_path, "shaders/");

    fs::write(output_path, processed_shader).expect("Failed to write preprocessed shader");
}

fn main() {
    preprocess_shader("shaders/main.wgsl", "shaders/c_main.wgsl");
    println!("cargo:rerun-if-changed=shaders/");
}