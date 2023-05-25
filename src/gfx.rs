use glium::{self, implement_vertex};

use obj::{self, Obj};
use std::fs;
use std::io::BufReader;

use glam;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

pub struct Mesh {
    pub vbo: glium::VertexBuffer<Vertex>,
    pub ibo: glium::IndexBuffer<u16>,

    pub pos: glam::Vec3,
    pub rot: glam::Vec3,
    pub scale: glam::Vec3,
}

impl Mesh {
    pub fn new(vbo: glium::VertexBuffer<Vertex>, ibo: glium::IndexBuffer<u16>) -> Mesh {
        Mesh {
            vbo,
            ibo,
            pos: glam::vec3(0.0, 0.0, 0.0),
            rot: glam::vec3(0.0, 0.0, 0.0),
            scale: glam::vec3(1.0, 1.0, 1.0),
        }
    }

    pub fn model_matrix(&self) -> glam::Mat4 {
        let t = glam::Mat4::from_translation(self.pos);
        let r = glam::Mat4::from_euler(glam::EulerRot::XYZ, self.rot.x, self.rot.y, self.rot.z);
        let s = glam::Mat4::from_scale(self.scale);

        t * r * s
    }
}

pub fn shaders(display: &glium::Display) -> glium::Program {
    let vss = r#"
    #version 330 core

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 proj;

    in vec3 position;
    in vec3 normal;

    out vec3 fPosition;
    out vec3 fNormal;

    void main() {
        gl_Position = proj * view * model * vec4(position, 1.0);

        fPosition = vec3(model * vec4(position, 1.0));
        fNormal = mat3(transpose(inverse(model))) * normal;
    }
    "#;

    let fss = r#"
    #version 330 core

    in vec3 fPosition;
    in vec3 fNormal;

    uniform vec3 view_position;

    uniform vec3 light_position;
    uniform vec3 light_color;
    
    out vec4 color;

    void main() {
        vec3 ambient = 0.05 * light_color;

        vec3 norm = normalize(fNormal);
        vec3 lightDir = normalize(light_position - fPosition);
        vec3 diffuse = max(dot(norm, lightDir), 0.0) * light_color;

        vec3 viewDir = normalize(view_position - fPosition);
        vec3 reflectDir = reflect(-lightDir, norm);

        vec3 specular = 0.5 * pow(max(dot(viewDir, reflectDir), 0.0), 32) * light_color;

        vec3 result = (ambient + diffuse + specular) * vec3(1.0, 0.0, 0.0);
        color = vec4(result, 1.0);
    }
    "#;

    glium::Program::from_source(display, &vss, &fss, None).expect("Failed to compile shaders.")
}

pub fn load_mesh(display: &glium::Display, path: &str) -> Result<Mesh, String> {
    let fread = BufReader::new(fs::File::open(path).map_err(|e| e.to_string())?);
    let obj: Obj = obj::load_obj(fread).map_err(|_| "failed to load object from file")?;

    let vbo = glium::VertexBuffer::<Vertex>::empty(display, obj.vertices.len()).map_err(|_| "could not create empty vbo")?;
    let ibo = glium::IndexBuffer::<u16>::empty(display, glium::index::PrimitiveType::TrianglesList, obj.vertices.len())
        .map_err(|_| "could not create empty ibo")?;

    vbo.write(obj.vertices.iter()
        .map(|v: &obj::Vertex| -> Vertex {
            Vertex {
                position: v.position,
                normal: v.normal,
            }
        })
        .collect::<Vec<Vertex>>()
        .as_slice()
    );

    ibo.write(obj.indices.as_slice());

    Ok(Mesh::new(vbo, ibo))
}
