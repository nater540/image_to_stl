use stl_io::{Normal, Vertex, Triangle};

pub fn build_mesh(heightmap: Vec<Vec<f32>>) -> Vec<Triangle> {
  let mut triangles = Vec::new();

  for y in 0..(heightmap.len() - 1) {
    for x in 0..(heightmap[y].len() - 1) {
      let v1 = Vertex::new([x as f32, y as f32, heightmap[y][x]]);
      let v2 = Vertex::new([(x + 1) as f32, y as f32, heightmap[y][x + 1]]);
      let v3 = Vertex::new([x as f32, (y + 1) as f32, heightmap[y + 1][x]]);
      let v4 = Vertex::new([(x + 1) as f32, (y + 1) as f32, heightmap[y + 1][x + 1]]);

      // Calculate the normals for the triangle vectors
      let normal1 = calculate_normal(v1, v2, v3);
      let normal2 = calculate_normal(v3, v2, v4);

      triangles.push(Triangle { normal: normal1, vertices: [v1, v2, v3] });
      triangles.push(Triangle { normal: normal2, vertices: [v3, v2, v4] });
    }
  }

  triangles
}

fn calculate_normal(v1: Vertex, v2: Vertex, v3: Vertex) -> Normal {
  let u = [v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]];
  let v = [v3[0] - v1[0], v3[1] - v1[1], v3[2] - v1[2]];

  // Calculate the cross product
  let cross = [
    u[1] * v[2] - u[2] * v[1],
    u[2] * v[0] - u[0] * v[2],
    u[0] * v[1] - u[1] * v[0]
  ];

  let len = (cross[0].powi(2) + cross[1].powi(2) + cross[2].powi(2)).sqrt();
  Normal::new([cross[0] / len, cross[1] / len, cross[2] / len])
}
