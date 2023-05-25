use glam;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub pos: glam::Vec3,
    pub pitch: f32,
    pub yaw: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: glam::Vec3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn facing(&self) -> glam::Vec3 {
        glam::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ).normalize_or_zero()
    }

    pub fn translate(&mut self, xyz: glam::Vec3) {
        self.pos += xyz;
    }

    pub fn translate_y(&mut self, y: f32) {
        self.pos.y += y;
    }

    pub fn rotate_pitch(&mut self, p: f32) {
        self.pitch += p;
        self.pitch = self.pitch.clamp(-89.0, 89.0);
    }

    pub fn rotate_yaw(&mut self, y: f32) {
        self.yaw += y;
        while self.yaw >= 360.0 { self.yaw -= 360.0; }
    }

    // moves laterally relative to the camera, where x is l/r and z (y) is f/b
    pub fn lateral_move(&mut self, xz: glam::Vec2) {
        let facing = self.facing();

        self.pos += glam::vec3(facing.x, 0.0, facing.z).normalize_or_zero() * xz.y;
        self.pos += glam::vec3(facing.x, 0.0, facing.z).cross(glam::vec3(0.0, 1.0, 0.0)).normalize_or_zero() * -xz.x;
    }
}

impl From<Camera> for glam::Mat4 {
    fn from(c: Camera) -> glam::Mat4 {
        glam::Mat4::look_at_lh(c.pos, c.pos + c.facing(), glam::vec3(0.0, 1.0, 0.0))
    }
}
