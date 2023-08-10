use image::{ImageBuffer, Luma, imageops::blur};
use image::io::Reader as ImageReader;
use stl_io::{Normal, Vertex, Triangle, write_stl};
use std::fs::File;
use std::io::Write;


fn main() {
  let input_image = ImageReader::open("input.png")
    .unwrap()
    .decode()
    .unwrap()
    .to_rgb8();

  let mut output_image: ImageBuffer<Luma<u16>, _> = ImageBuffer::new(input_image.width(), input_image.height());

  for (x, y, pixel) in input_image.enumerate_pixels() {
    let image::Rgb(data) = *pixel;

    // Convert the pixel to grayscale
    let gray_value = ((data[0] as u32 * 299 + data[1] as u32 * 587 + data[2] as u32 * 114) / 1000) as u16;
    let gray_pixel = Luma([gray_value * 256]);
    output_image.put_pixel(x, y, gray_pixel);
  }

  let sigma = 50.0;
  blur(&mut output_image, sigma);

  // Save the grayscale image
  output_image.save("output.png").unwrap();

  let mut heightmap: Vec<Vec<f32>> = vec![vec![0.0; output_image.width() as usize]; output_image.height() as usize];

  for (x, y, pixel) in output_image.enumerate_pixels() {
    // Normalize the pixel value to a height range
    let height = pixel[0] as f32 / 255.0;
    heightmap[y as usize][x as usize] = height;
  }

  let mut triangles = Vec::new();

  for y in 0..(heightmap.len() - 1) {
    for x in 0..(heightmap[y].len() - 1) {
      // Create vertices for the current quad
      let v1 = Vertex::new([x as f32, y as f32, heightmap[y][x]]);
      let v2 = Vertex::new([(x + 1) as f32, y as f32, heightmap[y][x + 1]]);
      let v3 = Vertex::new([x as f32, (y + 1) as f32, heightmap[y + 1][x]]);
      let v4 = Vertex::new([(x + 1) as f32, (y + 1) as f32, heightmap[y + 1][x + 1]]);

      // Create two triangles for the current quad
      triangles.push(Triangle {
          normal: Normal::new([0.0, 0.0, 1.0]), // You may want to calculate the correct normal
          vertices: [v1, v2, v3],
      });
      triangles.push(Triangle {
          normal: Normal::new([0.0, 0.0, 1.0]), // You may want to calculate the correct normal
          vertices: [v3, v2, v4],
      });
    }
  }

  let mut binary_stl = Vec::<u8>::new();
  stl_io::write_stl(&mut binary_stl, triangles.iter()).unwrap();

  let mut file = File::create("output.stl").unwrap();
  file.write_all(&binary_stl).unwrap();

}
