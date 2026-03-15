mod cli;
mod gpu;
mod io;
mod pipeline;

use clap::Parser;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    let image = io::load_image(&args.input)?;
    let (width, height) = (image.width(), image.height());

    if args.verbose {
        println!("Loaded: {} ({}x{})", args.input, width, height);
    }

    let pipeline = pipeline::Pipeline::new().await?;

    if args.verbose {
        println!("Processing...");
    }

    let maps = pipeline.process(&image).await?;

    if args.verbose {
        println!("Processing complete");
    }

    let height_img = io::height_to_image(width, height, &maps.height);
    let normal_img = io::normal_to_image(width, height, &maps.normal);
    let metallic_img = io::metallic_to_image(width, height, &maps.metallic);

    let format_str = format!("{}", args.format);
    let paths = io::get_output_paths(&args.input, &args.output, &format_str);
    let image_format = io::output_format_to_image_format(&args.format);

    io::save_image(&height_img, &paths.height_path, image_format, args.quality)?;
    io::save_image(&normal_img, &paths.normal_path, image_format, args.quality)?;
    io::save_image(&metallic_img, &paths.metallic_path, image_format, args.quality)?;

    if !args.quiet {
        println!("Generated:");
        println!("  - {}", paths.height_path);
        println!("  - {}", paths.normal_path);
        println!("  - {}", paths.metallic_path);
    }

    Ok(())
}
