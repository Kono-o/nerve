use std::ffi::c_void;
use std::mem::size_of;
use gl::types::*;
use crate::NerveShader;

pub struct NerveMesh {
   shader: NerveShader,
   vao_id: GLuint,
   vbo_id: GLuint,
}

impl NerveMesh {
   pub fn new(vertices: &[f32], indices: &[i32], material: NerveShader) -> Self {
      let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

      unsafe {
         gl::GenVertexArrays(1, &mut vao); //                           make new vertex array (holds pointers and mem layout of data in buffers), assigns vao the GL id
         gl::GenBuffers(1, &mut vbo); //                                make new vertex buffer (holds vertex attribute(s) data or just any data really), assigns vbo the GL id
         gl::GenBuffers(1, &mut ebo); //                                make new element buffer (holds indices i.e the order of which vert comes after which), assigns ebo the GL id

         gl::BindVertexArray(vao); //                                   bind(select) a vertex array with it's GL id
         gl::BindBuffer(gl::ARRAY_BUFFER, vbo); //                      bind(select) a vertex buffer with it's GL id, as a ARRAY_BUFFER (list of vertex data)
         gl::BufferData(
            //                                                          assign the currently selected buffer with data
            gl::ARRAY_BUFFER, //                                        the type of buffer (idk why it needs to again)
            (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr, //   size of the data (array_element_count * size_of_element_type) as a GL isize pointer
            &vertices[0] as *const f32 as *const c_void, //             actual data being sent the gpu, as a pointer to the start of the array
            gl::DYNAMIC_DRAW, //                                        use case of the data, DYNAMIC_DRAW means the data will be updated frequently, just a hint for GL to optimize stuff
         );

         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); //              bind(select) a element buffer with it's GL id
         gl::BufferData(
            //                                                          assign the currently selected buffer with data
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<GLint>()) as GLsizeiptr, //      size of the data (array_element_count * size_of_element_type) as a GL isize pointer
            &indices[0] as *const i32 as *const c_void, //              actual data being sent the gpu, as a pointer to the start of the array
            gl::DYNAMIC_DRAW, //                                        use case of the data, DYNAMIC_DRAW means the data will be updated frequently, just a hint for GL to optimize stuff
         );

         let stride = 6 * size_of::<GLfloat>() as GLsizei;
         gl::VertexAttribPointer(
            //                                                          define a pointer to a vertex attribute data array
            0, //                                                       the index of the vertex attribute
            3, //                                                       the number of components in a attribute element, eg: a vec3 float would have 3 float components
            gl::FLOAT, //                                               the type of the the element components
            gl::FALSE, //                                               bool for whether the data should be normalized or read as is
            stride, //                                                  the offset between consecutive same type vertex attribute to the next
            std::ptr::null(), //                                        the offset between vertex attributes themselves
         );
         gl::EnableVertexAttribArray(0); //                             enable a vertex attribute with its GL_id

         gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * size_of::<GLfloat>()) as *const c_void, //             the offset between vertex attributes themselves
         );
         gl::EnableVertexAttribArray(1); //                             enable a vertex attribute with its GL_id

         gl::BindBuffer(gl::ARRAY_BUFFER, 0); //                        deselect a vbo by selecting 0, which is a unreachable GL_id
         gl::BindVertexArray(0); //                                     deselect a vao by selecting 0, which is a unreachable GL_id
         Self {
            shader: material,
            vao_id: vao,
            vbo_id: vbo,
         }
      }
   }

   pub fn draw(&self) {
      unsafe {
         self.shader.set();
         gl::BindVertexArray(self.vao_id);
         gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
      }
   }

   pub fn kill(&self) {
      self.shader.kill();
      unsafe {
         gl::DeleteVertexArrays(1, self.vao_id as *const GLuint);
         gl::DeleteBuffers(1, self.vbo_id as *const GLuint);
      }
   }
}
