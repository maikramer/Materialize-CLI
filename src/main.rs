mod cli;
mod io;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    // Load image
    let image = io::load_image(&args.input)?;

    if args.verbose {
        println!("Loaded: {} ({}x{})", args.input, image.width(), image.height());
    }

    // Get output paths
    let format_str = format!("{}", args.format);
    let (height_path, normal_path, metallic_path) =
        io::get_output_paths(&args.input, &args.output, &format_str);

    println!("Would generate:");
    println!("  - {}", height_path);
    println!("  - {}", normal_path);
    println!("  - {}", metallic_path);

    Ok(())
}
