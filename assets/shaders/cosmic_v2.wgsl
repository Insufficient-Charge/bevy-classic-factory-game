#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::forward_io::VertexOutput

struct CosmicSphereMaterial {
    time: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> material: CosmicSphereMaterial;

fn hash3(p: vec3<f32>) -> f32 {
    let p2 = floor(p);
    let f = fract(p);
    let a = fract(sin(dot(p2, vec3<f32>(12.9898, 78.233, 45.164))) * 43758.5453);
    return a;
}

fn noise3d(p: vec3<f32>) -> f32 {
    let p2 = floor(p);
    let f = fract(p);
    let a = hash3(p2);
    let b = hash3(p2 + vec3<f32>(1.0, 0.0, 0.0));
    let c = hash3(p2 + vec3<f32>(0.0, 1.0, 0.0));
    let d = hash3(p2 + vec3<f32>(1.0, 1.0, 0.0));
    let e = hash3(p2 + vec3<f32>(0.0, 0.0, 1.0));
    let f1 = hash3(p2 + vec3<f32>(1.0, 0.0, 1.0));
    let g = hash3(p2 + vec3<f32>(0.0, 1.0, 1.0));
    let h = hash3(p2 + vec3<f32>(1.0, 1.0, 1.0));
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(mix(a, b, u.x), mix(c, d, u.x), u.y),
        mix(mix(e, f1, u.x), mix(g, h, u.x), u.y), u.z);
}

// 4D noise using rotation in 4th dimension for seamless time
fn noise4d(p: vec4<f32>) -> f32 {
    let px = p.x;
    let py = p.y;
    let pz = p.z;
    let pw = p.w;

    // Use spherical coordinates for 4D to avoid seams
    // Use multiple offsets to create seamless blend
    let offset1 = vec3<f32>(pw * 10.0, 0.0, 0.0);
    let offset2 = vec3<f32>(0.0, pw * 10.0, 0.0);
    let offset3 = vec3<f32>(0.0, 0.0, pw * 10.0);

    let n1 = noise3d(vec3<f32>(px, py, pz) + offset1);
    let n2 = noise3d(vec3<f32>(px, py, pz) + offset2);
    let n3 = noise3d(vec3<f32>(px, py, pz) + offset3);

    // Smooth blend based on time
    let blend = sin(pw * 6.28318) * 0.5 + 0.5;
    return mix(mix(n1, n2, blend), n3, blend);
}

fn fbm3d(p: vec3<f32>) -> f32 {
    var value = 0.0;
    var amp = 0.5;
    var scale = 3.0;
    for (var i = 0; i < 6; i++) {
        scale = scale * 3.0;
        value += amp * noise3d(p * scale);
    }
    return value;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = material.time;
    let pos = in.world_position.xyz;

    // Triplanar mapping with time-varying rotation
    // Each axis projection rotates based on time to create seamless flow
    let p = pos;
    let t_scaled = t * 0.1;

    // Rotated coordinates for each axis - prevents fixed reference points
    let rot1 = t_scaled * 0.7;
    let c1 = cos(rot1);
    let s1 = sin(rot1);
    let p_yz_rot = vec3<f32>(p.y * c1 - p.z * s1, p.y * s1 + p.z * c1, t_scaled);

    let rot2 = t_scaled * 0.5 + 1.57;
    let c2 = cos(rot2);
    let s2 = sin(rot2);
    let p_xz_rot = vec3<f32>(p.x * c2 - p.z * s2, p.x * s2 + p.z * c2, t_scaled + 100.0);

    let rot3 = t_scaled * 0.3 + 3.14;
    let c3 = cos(rot3);
    let s3 = sin(rot3);
    let p_xy_rot = vec3<f32>(p.x * c3 - p.y * s3, p.x * s3 + p.y * c3, t_scaled + 200.0);

    // Sample noise with rotated coordinates - lower frequency for smoother look
    let n_x = noise3d(p_yz_rot * 1.1);
    let n_y = noise3d(p_xz_rot * 1.1);
    let n_z = noise3d(p_xy_rot * 1.1);

    // Weights based on normal
    let normal = normalize(pos);
    let w_x = abs(normal.x);
    let w_y = abs(normal.y);
    let w_z = abs(normal.z);
    let total_weight = w_x + w_y + w_z;

    let n = (n_x * w_x + n_y * w_y + n_z * w_z) / total_weight;

    // Shadertoy colors
    let cyan = vec3<f32>(0.1, 1.0, 0.8);
    let teal = vec3<f32>(0.1, 0.5, 0.6);
    let purple = vec3<f32>(0.27, 0.2, 0.4);

    var color = vec3<f32>(0.0);

    let n_val = max(0.0, n * 6.0 - 2.6);
    color = color + 2.0 * cyan * n_val;
    color = color + teal * 0.5;
    color = color + purple * 0.3;

    // Color adjustments
    color.y = color.y * 0.8;
    color.x = color.x * 1.5;
    color = color * 2.0 - 0.15;

    return vec4<f32>(color, 1.0);
}
