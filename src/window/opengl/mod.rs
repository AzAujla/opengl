use program::Programs;
use shader::{Shader, ShaderType};

pub mod program;
pub mod shader;

pub mod ibo;
pub mod uniform;
pub mod vao;
pub mod vbo;

pub fn create_program() -> Result<Programs, String> {
    let mut p = Programs::new(ShaderType::BasicFlatShader);

    let (types, shaders) = ShaderType::to_shaders();

    for i in 0..types.len() {
        // println!("{}", types[i].to_usize(),);
        // println!("{}", &shaders[i].0.to_string_lossy(),);

        let vert = match Shader::from_source(&shaders[i].0, gl::VERTEX_SHADER) {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };
        // println!("{}", &shaders[i].1.to_string_lossy());
        let frag = match Shader::from_source(&shaders[i].1, gl::FRAGMENT_SHADER) {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };

        p.add_shaders(types[i].clone(), &[vert, frag])?
    }
    Ok(p)
}
