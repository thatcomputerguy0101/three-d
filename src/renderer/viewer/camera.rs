use crate::*;

///
/// Represents a camera used for viewing 2D and 3D objects.
///
#[derive(Clone, Debug)]
pub struct Camera {
    camera: three_d_asset::Camera,
    /// This tone mapping is applied to the final color of renders using this camera.
    pub tone_mapping: ToneMapping,
    /// This color mapping is applied to the final color of renders using this camera.
    pub color_mapping: ColorMapping,
}

impl Viewer for Camera {
    fn position(&self) -> Vec3 {
        self.camera.position()
    }

    fn view(&self) -> Mat4 {
        self.camera.view()
    }

    fn projection(&self) -> Mat4 {
        self.camera.projection()
    }

    fn viewport(&self) -> Viewport {
        self.camera.viewport()
    }

    fn z_near(&self) -> f32 {
        self.camera.z_near()
    }

    fn z_far(&self) -> f32 {
        self.camera.z_far()
    }

    fn color_mapping(&self) -> ColorMapping {
        self.color_mapping
    }

    fn tone_mapping(&self) -> ToneMapping {
        self.tone_mapping
    }
}

impl Camera {
    ///
    /// New camera which projects the world with an orthographic projection.
    ///
    pub fn new_orthographic(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        height: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_orthographic(
                viewport, position, target, up, height, z_near, z_far,
            ),
            tone_mapping: ToneMapping::default(),
            color_mapping: ColorMapping::default(),
        }
    }

    ///
    /// New camera which projects the world with a perspective projection.
    ///
    pub fn new_perspective(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        field_of_view_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_perspective(
                viewport,
                position,
                target,
                up,
                field_of_view_y,
                z_near,
                z_far,
            ),
            tone_mapping: ToneMapping::default(),
            color_mapping: ColorMapping::default(),
        }
    }

    ///
    /// New camera which projects the world with a planar projection.
    ///
    pub fn new_planar(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        field_of_view_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_planar(
                viewport,
                position,
                target,
                up,
                field_of_view_y,
                z_near,
                z_far,
            ),
            tone_mapping: ToneMapping::default(),
            color_mapping: ColorMapping::default(),
        }
    }

    ///
    /// Returns an orthographic camera for viewing 2D content.
    /// The camera is placed at the center of the given viewport.
    /// The (0, 0) position is at the bottom left corner and the
    /// (`viewport.width`, `viewport.height`) position is at the top right corner.
    ///
    pub fn new_2d(viewport: Viewport) -> Self {
        Self::new_orthographic(
            viewport,
            vec3(
                viewport.width as f32 * 0.5,
                viewport.height as f32 * 0.5,
                1.0,
            ),
            vec3(
                viewport.width as f32 * 0.5,
                viewport.height as f32 * 0.5,
                0.0,
            ),
            vec3(0.0, 1.0, 0.0),
            viewport.height as f32,
            0.0,
            10.0,
        )
    }

    ///
    /// Disables the tone and color mapping so as to be ready for rendering into an intermediate render target with this camera.
    ///
    pub fn disable_tone_and_color_mapping(&mut self) {
        self.tone_mapping = ToneMapping::None;
        self.color_mapping = ColorMapping::None;
    }

    ///
    /// Sets the tone and color mapping to default so as to be ready for rendering into the final render target (usually the screen) with this camera.
    ///
    pub fn set_default_tone_and_color_mapping(&mut self) {
        self.tone_mapping = ToneMapping::default();
        self.color_mapping = ColorMapping::default();
    }

    ///
    /// Finds the closest intersection between a ray from the given camera in the given pixel coordinate and the given geometries.
    /// The pixel coordinate must be in physical pixels, where (viewport.x, viewport.y) indicate the bottom left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the top right corner.
    /// Returns ```None``` if no geometry was hit between the near (`z_near`) and far (`z_far`) plane for this camera.
    ///
    pub fn pick(
        &self,
        context: &Context,
        pixel: impl Into<PhysicalPoint> + Copy,
        geometries: impl IntoIterator<Item = impl Geometry>,
    ) -> Option<IntersectionResult> {
        let pos = self.position_at_pixel(pixel);
        let dir = self.view_direction_at_pixel(pixel);
        ray_intersect(
            context,
            pos + dir * self.z_near(),
            dir,
            self.z_far() - self.z_near(),
            geometries,
        )
    }

    /// Returns the [Frustum] for this camera.
    pub fn frustum(&self) -> Frustum {
        Frustum::new(self.projection() * self.view())
    }
}

impl std::ops::Deref for Camera {
    type Target = three_d_asset::Camera;
    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl std::ops::DerefMut for Camera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}