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
    output_image.save(output_path.join("b&w.png"))?;
  }

  let heightmap = utils::generate_heightmap(output_image, args.max_height, args.base_height);
  let triangles = stl::build_mesh(heightmap);
  stl::write_stl(triangles, output_path.join("generated.stl"))?;

  Ok(())
}
