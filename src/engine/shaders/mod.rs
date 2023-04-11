
use std::fs::File;
use std::io::Read;

fn file_to_string(path : &str) -> String {
    let mut file = File::open(path).expect("Failed to open shader file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read shader file");
    contents
}

fn parse_include_directive(line : &str, original_dir : &String) -> String {
    let file_name = line
        .split(" ").collect::<Vec<&str>>()[1]
        .replace("\"", "")
        .replace(";", ""); 

    let include_path = original_dir.clone() + "/" + &file_name.to_owned();
    compile_shader(include_path.as_str())
}

fn parse_define_directive(line : &str) -> (String, String) {

    let define_type = line
        .split("<").collect::<Vec<&str>>()[1]
        .split(">").collect::<Vec<&str>>()[0]; 

    let define_name = line
        .split(" ").collect::<Vec<&str>>()[1]
        .split(":").collect::<Vec<&str>>()[0]
        .replace(" ", "")
        .replace(define_type, "")
        .replace("<", "")
        .replace(">", "")
        .replace(";", ""); 

    let define_value = line
        .split(" ").collect::<Vec<&str>>()[2]
        .replace(";", ""); 

    (define_name, format!("{}({})", define_type, define_value))
}

pub fn compile_shader(path : &str) -> String {

    let path_dirs = path.split("/").collect::<Vec<&str>>();
    let original_dir = path_dirs[0..path_dirs.len() - 1].join("/");

    let mut final_shader_code = String::new();

    let mut defines = Vec::new();
    
    for line in file_to_string(path).lines() {
        if line.contains("#include") {
            let source = parse_include_directive(line, &original_dir); 
            final_shader_code.push_str(&source);
        } else if line.contains("#define") {
            defines.push(parse_define_directive(line)); 
        } else {
            final_shader_code.push_str(&(line.to_owned() + "\n")); 
        }
    }

    for define in defines {
        final_shader_code = final_shader_code.replace(&define.0, &define.1);
    }

    final_shader_code
}