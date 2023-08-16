use std::fs::File;
use std::io::Write;
use anyhow::Result;

mod stl;
mod cli;
mod utils;

fn main() -> Result<()> {
  let args = cli::parse();
  let output_path = args.output.unwrap_or_else(|| std::env::current_dir().unwrap());

  // Attempt to load the input image
  let input_image = utils::load_png(&args.input)?;
  let output_image = utils::convert_bw(input_image, args.gaussian_sigma);

  // Save the grayscale image if requested
  if args.generate_black_white {
    output_image.save(output_path.join("bw.png"))?;
  }

  let heightmap = utils::generate_heightmap(output_image, args.max_height, args.base_height);
  utils::write_heightmap_png(&heightmap, output_path.join("heightmap.png"))?;

  let triangles = stl::build_mesh(heightmap);

  let mut binary_stl = Vec::<u8>::new();
  stl_io::write_stl(&mut binary_stl, triangles.iter()).unwrap();

  let mut file = File::create("output.stl").unwrap();
  file.write_all(&binary_stl).unwrap();

  Ok(())
}
