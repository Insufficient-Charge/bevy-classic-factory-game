#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::forward_io::VertexOutput

struct CosmicSphereMaterial {
    time: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: CosmicSphereMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = material.time;
    let world_pos = in.world_position.xyz;
    let stripe = sin(world_pos.x * 2.0 + t) * 0.5 + 0.5;
    let color = vec3<f32>(stripe, 0.2, 1.0 - stripe);
    return vec4<f32>(color, 1.0);
}