#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_types

struct FragmentInput {
	#import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

}
