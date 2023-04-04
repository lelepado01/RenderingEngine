
use std::fs::File;
use std::io::Read;

fn file_to_string(path : &str) -> String {
    let mut file = File::open(path).expect("Failed to open shader file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read shader file");
    contents
}

pub fn compile_shader(
    path : &str, 
) -> String {

    let original_dir = path.split("/").collect::<Vec<&str>>()[0..path.split("/").collect::<Vec<&str>>().len() - 1].join("/");
    let contents = file_to_string(path);
    let mut final_shader_code = String::new();

    for line in contents.lines() {
        if line.contains("#include") {
            
            let include_path = original_dir.clone() + "/" + &line.split(" ").collect::<Vec<&str>>()[1][1..line.split(" ").collect::<Vec<&str>>()[1].len() - 2];
            let include_contents = compile_shader(include_path.as_str());
            final_shader_code.push_str(&include_contents);

        } else {
            final_shader_code.push_str(&(line.to_owned() + "\n")); 
        }
    }

    // final_shader_code.push_str(&contents);
    final_shader_code
}