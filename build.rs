use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
use std::fs::File;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let mut file = File::create("./src/opengl/bindings.rs").unwrap();

    let registry = Registry::new(
        Api::Gl,
        (4, 5),
        Profile::Core,
        Fallbacks::All,
        [
            "GL_NV_command_list", // TODO: See if this is needed or not.
        ],
    );

    registry.write_bindings(StructGenerator, &mut file).unwrap();
}
