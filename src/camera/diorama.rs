use bevy::prelude::*;
use bevy::camera::ScalingMode;

/// Marks the 2.5D Diorama Camera entity.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DioramaCamera;

/// Global configuration and state for the Diorama Camera.
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct DioramaCameraConfig {
    /// True isometric pitch: arcsin(tan(30 deg)) ≈ 35.264 degrees in radians.
    pub pitch_radians: f32,
    /// Stepped rotation index: 0 = 45°, 1 = 135°, 2 = 225°, 3 = 315°.
    pub rotation_step: u8,
    /// Distance from the focus target.
    pub distance: f32,
    /// Orthographic vertical view height in world units.
    pub vertical_size: f32,
    /// Min allowable zoom size.
    pub min_vertical_size: f32,
    /// Max allowable zoom size.
    pub max_vertical_size: f32,
    /// Interpolation smoothing factor (0.0 to 1.0).
    pub smooth_factor: f32,
    /// Current interpolated camera yaw angle in radians.
    pub current_yaw: f32,
    /// Target camera yaw angle in radians.
    pub target_yaw: f32,
}

impl Default for DioramaCameraConfig {
    fn default() -> Self {
        let initial_yaw = 45.0_f32.to_radians();
        Self {
            pitch_radians: 35.264_f32.to_radians(),
            rotation_step: 0,
            distance: 50.0,
            vertical_size: 16.0,
            min_vertical_size: 6.0,
            max_vertical_size: 32.0,
            smooth_factor: 0.15,
            current_yaw: initial_yaw,
            target_yaw: initial_yaw,
        }
    }
}

impl DioramaCameraConfig {
    /// Computes target yaw angle in radians based on the current target yaw.
    pub fn target_yaw_radians(&self) -> f32 {
        self.target_yaw
    }

    /// Step camera 90 degrees clockwise (E key).
    pub fn rotate_cw(&mut self) {
        self.rotation_step = (self.rotation_step + 1) % 4;
        self.target_yaw += std::f32::consts::FRAC_PI_2;
    }

    /// Step camera 90 degrees counter-clockwise (Q key).
    pub fn rotate_ccw(&mut self) {
        self.rotation_step = (self.rotation_step + 3) % 4;
        self.target_yaw -= std::f32::consts::FRAC_PI_2;
    }

    /// Rotate camera by delta degrees (smooth mouse drag or analog controls).
    pub fn rotate_by_degrees(&mut self, delta_degrees: f32) {
        self.target_yaw += delta_degrees.to_radians();
        let deg = self.target_yaw.to_degrees().rem_euclid(360.0);
        let normalized = (deg - 45.0 + 360.0).rem_euclid(360.0);
        self.rotation_step = ((normalized / 90.0).round() as u8) % 4;
    }

    /// Adjust vertical size zoom within clamped limits.
    pub fn adjust_zoom(&mut self, delta: f32) {
        self.vertical_size = (self.vertical_size + delta).clamp(self.min_vertical_size, self.max_vertical_size);
    }
}

/// Converts a step index (0..=3) to an isometric yaw angle in radians.
pub fn rotation_step_to_yaw(step: u8) -> f32 {
    let degrees = 45.0 + (step % 4) as f32 * 90.0;
    degrees.to_radians()
}

/// Computes the camera orbital offset vector for a given pitch, yaw, and distance.
pub fn compute_camera_offset(pitch: f32, yaw: f32, distance: f32) -> Vec3 {
    let x = distance * yaw.sin() * pitch.cos();
    let y = distance * pitch.sin();
    let z = distance * yaw.cos() * pitch.cos();
    Vec3::new(x, y, z)
}

/// Translates screen-space 2D input (e.g. WASD movement) into world-space movement
/// aligned with the current isometric camera view orientation.
pub fn iso_input_to_world_dir(input: Vec2, rotation_step: u8) -> Vec3 {
    if input.length_squared() < 0.001 {
        return Vec3::ZERO;
    }

    let yaw = rotation_step_to_yaw(rotation_step);
    // Forward vector in world space (looking into the screen along camera yaw)
    let forward = Vec3::new(-yaw.sin(), 0.0, -yaw.cos()).normalize();
    // Right vector in world space
    let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin()).normalize();

    // input.y is Forward/Back (W/S), input.x is Right/Left (D/A)
    (right * input.x + forward * input.y).normalize()
}

/// Marker for an entity that the diorama camera should follow.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DioramaCameraTarget;

pub fn plugin(app: &mut App) {
    app.init_resource::<DioramaCameraConfig>()
        .register_type::<DioramaCamera>()
        .register_type::<DioramaCameraConfig>()
        .register_type::<DioramaCameraTarget>()
        .add_systems(Update, (handle_diorama_camera_input, update_diorama_camera));
}

/// System to handle keyboard/gamepad/mouse camera rotation and zoom inputs.
pub fn handle_diorama_camera_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mut config: ResMut<DioramaCameraConfig>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        config.rotate_cw();
    }
    if keyboard.just_pressed(KeyCode::KeyQ) {
        config.rotate_ccw();
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        config.rotate_by_degrees(1.5);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        config.rotate_by_degrees(-1.5);
    }

    if mouse_button.pressed(MouseButton::Middle) {
        for ev in mouse_motion.read() {
            config.rotate_by_degrees(-ev.delta.x * 0.4);
        }
    } else {
        mouse_motion.clear();
    }

    if keyboard.pressed(KeyCode::Equal) || keyboard.pressed(KeyCode::NumpadAdd) {
        config.adjust_zoom(-0.5);
    }
    if keyboard.pressed(KeyCode::Minus) || keyboard.pressed(KeyCode::NumpadSubtract) {
        config.adjust_zoom(0.5);
    }
}

/// Smoothly updates camera transform and orthographic projection to track the target along a true orbital sphere.
pub fn update_diorama_camera(
    mut config: ResMut<DioramaCameraConfig>,
    target_q: Query<&Transform, (With<DioramaCameraTarget>, Without<DioramaCamera>)>,
    mut camera_q: Query<(&mut Transform, &mut Projection), With<DioramaCamera>>,
) {
    let Ok((mut cam_transform, mut projection)) = camera_q.single_mut() else {
        return;
    };

    let target_pos = if let Ok(t) = target_q.single() {
        Vec3::new(t.translation.x, t.translation.y.clamp(0.0, 10.0), t.translation.z)
    } else {
        Vec3::new(3.5, 0.0, 3.5)
    };

    // Smoothly interpolate current_yaw toward target_yaw
    config.current_yaw = config.current_yaw + (config.target_yaw - config.current_yaw) * config.smooth_factor;

    // When the smooth transition is essentially complete, normalize yaw within [0, TAU)
    if (config.target_yaw - config.current_yaw).abs() < 0.001 {
        config.current_yaw = config.target_yaw;
        let tau = std::f32::consts::TAU;
        config.current_yaw = config.current_yaw.rem_euclid(tau);
        config.target_yaw = config.target_yaw.rem_euclid(tau);
    }

    let offset = compute_camera_offset(config.pitch_radians, config.current_yaw, config.distance);
    let desired_pos = target_pos + offset;

    cam_transform.translation = desired_pos;
    cam_transform.look_at(target_pos, Vec3::Y);

    // Update orthographic projection zoom
    if let Projection::Orthographic(ref mut ortho) = *projection {
        ortho.scaling_mode = ScalingMode::FixedVertical { viewport_height: config.vertical_size };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_step_cycling() {
        let mut config = DioramaCameraConfig::default();
        assert_eq!(config.rotation_step, 0);

        // Clockwise 90-degree steps: 0 -> 1 -> 2 -> 3 -> 0
        config.rotate_cw();
        assert_eq!(config.rotation_step, 1);
        config.rotate_cw();
        assert_eq!(config.rotation_step, 2);
        config.rotate_cw();
        assert_eq!(config.rotation_step, 3);
        config.rotate_cw();
        assert_eq!(config.rotation_step, 0);

        // Counter-clockwise: 0 -> 3 -> 2 -> 1 -> 0
        config.rotate_ccw();
        assert_eq!(config.rotation_step, 3);
        config.rotate_ccw();
        assert_eq!(config.rotation_step, 2);
        config.rotate_ccw();
        assert_eq!(config.rotation_step, 1);
        config.rotate_ccw();
        assert_eq!(config.rotation_step, 0);
    }

    #[test]
    fn test_rotation_step_to_yaw_angles() {
        // Step 0: 45 degrees
        let yaw0 = rotation_step_to_yaw(0).to_degrees();
        assert!((yaw0 - 45.0).abs() < 0.001);

        // Step 1: 135 degrees
        let yaw1 = rotation_step_to_yaw(1).to_degrees();
        assert!((yaw1 - 135.0).abs() < 0.001);

        // Step 2: 225 degrees
        let yaw2 = rotation_step_to_yaw(2).to_degrees();
        assert!((yaw2 - 225.0).abs() < 0.001);

        // Step 3: 315 degrees
        let yaw3 = rotation_step_to_yaw(3).to_degrees();
        assert!((yaw3 - 315.0).abs() < 0.001);
    }

    #[test]
    fn test_compute_camera_offset() {
        let pitch = 30.0_f32.to_radians();
        let yaw = 45.0_f32.to_radians();
        let distance = 10.0;
        let offset = compute_camera_offset(pitch, yaw, distance);

        assert!((offset.y - 5.0).abs() < 0.001); // sin(30) * 10 = 5.0
        assert!(offset.x > 0.0);
        assert!(offset.z > 0.0);
        assert!((offset.length() - distance).abs() < 0.001);
    }

    #[test]
    fn test_iso_input_direction_normalization() {
        let input_zero = Vec2::ZERO;
        let dir_zero = iso_input_to_world_dir(input_zero, 0);
        assert_eq!(dir_zero, Vec3::ZERO);

        let input_w = Vec2::new(0.0, 1.0); // W (forward)
        let dir_w = iso_input_to_world_dir(input_w, 0);
        assert!((dir_w.length() - 1.0).abs() < 0.001);
        assert_eq!(dir_w.y, 0.0); // movement stays on horizontal plane
    }

    #[test]
    fn test_iso_input_direction_rotates_with_camera() {
        let input_forward = Vec2::new(0.0, 1.0);

        let dir_step0 = iso_input_to_world_dir(input_forward, 0);
        let dir_step1 = iso_input_to_world_dir(input_forward, 1);
        let dir_step2 = iso_input_to_world_dir(input_forward, 2);
        let dir_step3 = iso_input_to_world_dir(input_forward, 3);

        // Consecutive 90-degree steps must be perpendicular (dot product ≈ 0)
        assert!(dir_step0.dot(dir_step1).abs() < 0.001);
        assert!(dir_step1.dot(dir_step2).abs() < 0.001);
        // Opposite 180-degree steps must be inverted (dot product ≈ -1)
        assert!((dir_step0.dot(dir_step2) + 1.0).abs() < 0.001);
        assert!((dir_step1.dot(dir_step3) + 1.0).abs() < 0.001);
    }

    #[test]
    fn test_zoom_clamping() {
        let mut config = DioramaCameraConfig {
            min_vertical_size: 5.0,
            max_vertical_size: 20.0,
            vertical_size: 10.0,
            ..DioramaCameraConfig::default()
        };

        config.adjust_zoom(-100.0);
        assert_eq!(config.vertical_size, 5.0);

        config.adjust_zoom(100.0);
        assert_eq!(config.vertical_size, 20.0);
    }

    #[test]
    fn test_full_360_degree_rotation() {
        let mut config = DioramaCameraConfig::default();
        let initial_yaw = config.target_yaw;
        // 4 x 90-degree steps = full 360-degree rotation back to starting angle
        config.rotate_cw();
        config.rotate_cw();
        config.rotate_cw();
        config.rotate_cw();
        let delta = config.target_yaw - initial_yaw;
        assert!((delta - std::f32::consts::TAU).abs() < 0.001);
        assert_eq!(config.rotation_step, 0);

        // Counter-clockwise 360-degree rotation back
        config.rotate_ccw();
        config.rotate_ccw();
        config.rotate_ccw();
        config.rotate_ccw();
        let delta_back = config.target_yaw - initial_yaw;
        assert!(delta_back.abs() < 0.001);
        assert_eq!(config.rotation_step, 0);
    }
}
