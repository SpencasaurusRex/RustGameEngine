use std::ffi::{CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use cgmath::{Matrix, Matrix4, Vector3, Array};

pub struct Shader {
    pub id: u32,
}

fn to_cstr(string : &str) -> CString {
    CString::new(string.as_bytes()).unwrap()
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_src_path: &str, fragment_src_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // Get the source
        let mut v_shader_file = File::open(vertex_src_path).expect(&format!("Failed to open {}", vertex_src_path));
        let mut f_shader_file = File::open(fragment_src_path).expect(&format!("Failed to open {}", fragment_src_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");
        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");

        let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

        // Compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");

            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");

            // shader Program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");

            // Delete shaders
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    /// activate the shader
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    pub unsafe fn set_bool(&self, name: &str, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, to_cstr(&name).as_ptr()), value as i32);
    }

    pub unsafe fn set_int(&self, name: &str, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, to_cstr(&name).as_ptr()), value);
    }

    pub unsafe fn set_float(&self, name: &str, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, to_cstr(&name).as_ptr()), value);
    }

    pub unsafe fn set_vector3(&self, name: &str, value: &Vector3<f32>) {
        gl::Uniform3fv(gl::GetUniformLocation(self.id, to_cstr(&name).as_ptr()), 1, value.as_ptr());
    }

    pub unsafe fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, to_cstr(&name).as_ptr()), x, y, z);
    }

    pub unsafe fn set_mat4(&self, name: &str, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, to_cstr(&name).as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }

    /// utility function for checking shader compilation/linking errors.
    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }
        }
    }
}
