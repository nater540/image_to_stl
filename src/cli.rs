use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
  #[arg(long, short = 'i', value_hint = clap::ValueHint::DirPath, help = "Input PNG file")]
  pub input: PathBuf,

  #[arg(long, short = 'o', value_hint = clap::ValueHint::DirPath, help = "Output path")]
  pub output: Option<PathBuf>,

  #[arg(long, default_value = "5", help = "Max height for vertices in the STL file")]
  pub max_height: f32,

  #[arg(long, default_value = None, help = "Base height for vertices in the STL file")]
  pub base_height: Option<f32>,

  #[arg(long = "stl", default_value = "true", help = "Generate an STL file")]
  pub generate_stl: bool,

  #[arg(long = "bw", default_value = "true", help = "Generate a black & white PNG file")]
  pub generate_black_white: bool,

  #[arg(long = "gaussian", default_value = None, help = "Gaussian blur sigma to use")]
  pub gaussian_sigma: Option<f32>
}

pub fn parse() -> Args {
  Args::parse()
}
