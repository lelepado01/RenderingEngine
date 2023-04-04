
use std::fs::File;
use std::io::Read;

fn file_to_string(path : &str) -> String {
    let mut file = File::open(path).expect("Failed to open shader file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read shader file");
    contents
}

pub fn compile_shader(path : &str) -> String {

    let path_dirs = path.split("/").collect::<Vec<&str>>();
    let original_dir = path_dirs[0..path_dirs.len() - 1].join("/");

    let mut final_shader_code = String::new();
    
    for line in file_to_string(path).lines() {
        if line.contains("#include") {
            
            let file_name = line
                .split(" ").collect::<Vec<&str>>()[1]
                .replace("\"", "")
                .replace(";", ""); 

            let include_path = original_dir.clone() + "/" + &file_name.to_owned();
            let include_contents = compile_shader(include_path.as_str());
            final_shader_code.push_str(&include_contents);

        } else {
            final_shader_code.push_str(&(line.to_owned() + "\n")); 
        }
    }

    final_shader_code
}