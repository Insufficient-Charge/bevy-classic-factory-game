#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::forward_io::VertexOutput

struct CosmicSphereMaterial {
    time: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: CosmicSphereMaterial;

fn rotateY(v: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(v.x * c + v.z * s, v.y, -v.x * s + v.z * c);
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

fn hash(p: vec3<f32>) -> f32 {
    let p2 = floor(p);
    let f = fract(p);
    let a = fract(sin(dot(p2, vec3<f32>(12.9898, 78.233, 45.164))) * 43758.5453);
    return a;
}

fn noise(p: vec3<f32>) -> f32 {
    let p2 = floor(p);
    let f = fract(p);
    let a = hash(p2);
    let b = hash(p2 + vec3<f32>(1.0, 0.0, 0.0));
    let c = hash(p2 + vec3<f32>(0.0, 1.0, 0.0));
    let d = hash(p2 + vec3<f32>(1.0, 1.0, 0.0));
    let e = hash(p2 + vec3<f32>(0.0, 0.0, 1.0));
    let f1 = hash(p2 + vec3<f32>(1.0, 0.0, 1.0));
    let g = hash(p2 + vec3<f32>(0.0, 1.0, 1.0));
    let h = hash(p2 + vec3<f32>(1.0, 1.0, 1.0));
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(mix(a, b, u.x), mix(c, d, u.x), u.y),
               mix(mix(e, f1, u.x), mix(g, h, u.x), u.y), u.z);
}

fn fbm(p: vec3<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var scale = 1.0;
    for (var i = 0; i < 5; i++) {
        value += amplitude * noise(p * scale);
        scale = scale * 2.0;
        amplitude *= 0.5;
    }
    return value;
}

fn map(p: vec3<f32>) -> f32 {
    let t = material.time;
    let n = fbm(p * 0.5 + vec3<f32>(t * 0.1));
    
    var d = (-1.0 * length(p) + 3.0) + 1.5 * n;
    d = min(d, length(p) - 1.5 + 1.5 * n);
    
    let m = 1.5;
    let s = 0.03;
    d = smin(d, max(abs(p.x) - s, abs(p.y + p.z * 0.2) - 0.07), m);
    d = smin(d, max(abs(p.z) - s, abs(p.x + p.y * 0.5) - 0.07), m);
    d = smin(d, max(abs(p.z - p.y * 0.4) - s, abs(p.x - p.y * 0.2) - 0.07), m);
    d = smin(d, max(abs(p.z * 0.2 - p.y) - s, abs(p.x + p.z) - 0.07), m);
    d = smin(d, max(abs(p.z * -0.2 + p.y) - s, abs(-p.x + p.z) - 0.07), m);
    
    return d;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = material.time;
    let world_pos = in.world_position.xyz;
    let pulse = sin(t + world_pos.x * 0.1) * 0.5 + 0.5;
    let color = vec3<f32>(pulse, 0.5, 1.0 - pulse);
    return vec4<f32>(color, 1.0);
}