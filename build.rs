extern crate ron;

use std::fs;

fn main() {
    // // Parse project.ron for scenes
    // println!("cargo:rerun-if-changed=src/project.ron");
    // let project_ron_content = fs::read_to_string("src/project.ron")
    //     .expect("Error: Could not read src/project.ron.");
    // let project_ron_parsed = ron::from_str(project_ron_content)?;

    // * Parse shader.ron for init and draw code
    // * Create standalone shaders from shadertoy fragment shader files.
    // * glslang-validator
    // * ctrl-alt-test-minifier onto shader files, convert to plaintext
    // * auto-create rs file with shader content
}