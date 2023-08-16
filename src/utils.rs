use image::io::Reader as ImageReader;
use image::{ImageBuffer, Luma, Rgb};
use std::path::PathBuf;
use anyhow::Result;

/// Simple black & white image conversion with optional gaussian blur operations
pub fn convert_bw(input_image: ImageBuffer<Rgb<u8>, Vec<u8>>, gaussian: Option<f32>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
  let mut output_image: ImageBuffer<Luma<u8>, _> = ImageBuffer::new(input_image.width(), input_image.height());

  for (x, y, pixel) in input_image.enumerate_pixels() {
    let Rgb(data) = *pixel;

    // Convert the pixel to grayscale
    let gray_value = ((data[0] as u32 * 299 + data[1] as u32 * 587 + data[2] as u32 * 114) / 1000) as u8;
    let gray_pixel = Luma([gray_value]);
    output_image.put_pixel(x, y, gray_pixel);
  }

  if let Some(sigma) = gaussian {
    output_image = image::imageops::blur(&output_image, sigma);
  }

  output_image
}

/// Loads a PNG file from disk
pub fn load_png(file_path: &PathBuf) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
  Ok(
    ImageReader::open(file_path)?
      .decode()?
      .to_rgb8()
  )
}

pub fn write_heightmap_png(heightmap: &Vec<Vec<f32>>, path: PathBuf) -> Result<()> {
  let width = heightmap[0].len() as u32;
  let height = heightmap.len() as u32;
  let max_height = heightmap
    .iter()
    .flat_map(|row| row.iter()).cloned().fold(f32::MIN, f32::max);

  let mut output = ImageBuffer::new(width, height);

  for (y, row) in heightmap.iter().enumerate() {
    for (x, &value) in row.iter().enumerate() {
      let pixel_value = (value / max_height * 255.0) as u8;
      let pixel = Luma([pixel_value]);
      output.put_pixel(x as u32, y as u32, pixel);
    }
  }

  output.save(path).map_err(|e| e.into())
}

/// Generates a heightmap from a u16 grayscale image with optional heights
pub fn generate_heightmap(image: ImageBuffer<Luma<u8>, Vec<u8>>, max_height: f32, base_height: Option<f32>) -> Vec<Vec<f32>> {
  let mut heights: Vec<(u32, u32, f32)> = image
    .enumerate_pixels()
    .map(|(x, y, pixel)| (x, y, pixel[0] as f32 / 255.0 * max_height))
    .collect::<Vec<_>>();

  // Sort the calculated heights by their Y coord
  heights.sort_by(|a, b| a.1.cmp(&b.1));

  let mut heightmap: Vec<Vec<f32>> = Vec::new();
  let mut row: Vec<f32> = Vec::new();
  let mut current_y = 0;

  for (_, y, height) in heights {
    if y != current_y {
      heightmap.push(row);
      row = Vec::new();
      current_y = y;
    }
    row.push(height);
  }
  heightmap.push(row);

  // Apply a base height if necessary (helps prevent jagged edges)
  if let Some(base_height) = base_height {
    let length = heightmap.len();

    for y in 0..heightmap.len() {
      let last_index = heightmap[y].len() - 1;
      heightmap[y][0] = base_height;
      heightmap[y][last_index] = base_height;
    }

    for x in 0..heightmap[0].len() {
      heightmap[0][x] = base_height;
      heightmap[length - 1][x] = base_height;
    }
  }

  heightmap
}
