use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

///Creates a circle in the x-y plane
pub fn default_circle(vertices: i32, radius: f32) -> Mesh {
    assert!(vertices > 2, "Vertices should be larger than 2, is: {}", vertices);

    let mut circle: Vec<Vec3> = Vec::new();

    circle.push(Vec3::new(0.0, 0.0, 0.0));

    for i in 0..(vertices) {
        let angle = std::f32::consts::TAU / vertices as f32;
        circle.push(
            Vec3::new(
                (i as f32 * angle).cos() * radius,
                (i as f32 * angle).sin() * radius,
                0.0));
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION,
        dbg!(circle.iter()
        .map(|e| [e.x, e.y, e.z])
        .collect::<Vec<[f32; 3]>>()));

    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; (vertices + 1) as usize]);
    //TODO
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; (vertices + 1) as usize]);

    let indices = Some(Indices::U32(dbg!((1..(vertices)).map(|e| {
        vec![0 as u32, (e) as u32, (e + 1) as u32]
    }).flatten().chain([0u32, (vertices) as u32, 1u32]).collect::<Vec<u32>>())));

    mesh.set_indices(indices);
    mesh
}


pub fn default_ring(segments: i32, inner_radius: f32, outer_radius: f32) -> Mesh {
    assert!(segments > 2, "Segments should be greater than 2, is: {}", segments);
    assert!(inner_radius < outer_radius, "Inner radius of ring needs to be smaller than outer radius");

    let angle = std::f32::consts::TAU / segments as f32;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, (0..segments)
        .map(|i|
            [
                [
                    (i as f32 * angle).cos() * inner_radius,
                    (i as f32 * angle).sin() * inner_radius,
                    0.0
                ],
                [
                    (i as f32 * angle).cos() * outer_radius,
                    (i as f32 * angle).sin() * outer_radius,
                    0.0
                ]
            ])
        .flatten().collect::<Vec<[f32; 3]>>()
        );

    mesh.set_indices(Some(Indices::U32((0..(segments * 2)).chain(0..2).map(|e| e as u32).collect::<Vec<u32>>())));

    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; (segments * 2) as usize]);
    //TODO
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; (segments * 2) as usize]);


    mesh
}
