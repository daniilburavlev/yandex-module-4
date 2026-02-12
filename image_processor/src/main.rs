use clap::Parser;
use image_processor::process_image;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::exit,
};

fn get_default_plugin_path() -> PathBuf {
    let mut path = env::current_dir().expect("error opening current path");
    path.push("target/debug");
    path
}

#[derive(Debug, Parser)]
#[command(name = "image-processor", about = "Image Processor")]
struct Cli {
    #[arg(long, help = "Input image path")]
    input: PathBuf,
    #[arg(long, help = "Output image path")]
    output: PathBuf,
    #[arg(long, help = "Plugin name")]
    plugin: String,
    #[arg(long, help = "Path to text file with processing params")]
    params: PathBuf,
    #[arg(
        long,
        help = "Plugin path",
        default_value = get_default_plugin_path().into_os_string()
    )]
    plugin_path: PathBuf,
}

fn read_params(params: &Path) -> String {
    fs::read_to_string(params).expect("cannot read params, check path of file format")
}

fn main() {
    let mut cli = Cli::parse();
    cli.plugin_path.push(cli.plugin);
    let params = read_params(&cli.params);
    if let Err(e) = process_image(&cli.input, &cli.output, &cli.plugin_path, params) {
        eprintln!("{}", e);
        exit(-1);
    }
}
