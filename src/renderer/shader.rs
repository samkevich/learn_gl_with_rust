use gl::types::*;
use std::ffi::{CString, NulError};
use std::ptr;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("Error while compiling shader: {0}")]
    CompilationError(String),
    #[error("Error while linking shaders: {0}")]
    LinkingError(String),
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"{0}"}]
    NulError(#[from] NulError),
}

pub struct Shader {
    pub id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub unsafe fn new(source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(source_code)?;
        let shader = Self {
            id: gl::CreateShader(shader_type),
        };
        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);

        // check for shader compilation errors
        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);

        if success == 1 {
            Ok(shader)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                shader.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::CompilationError(log))
        }
    }
}
