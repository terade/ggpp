mod model;
use clap::Parser;
use image::Rgb;
use model::*;
use rand::prelude::*;
use std::env;

const DEFAULT_SQUARE_LENGTH: u32 = 70;
const DEFAULT_OUTPUT_FILE_NAME: &str = "gh_pfp";
const DEFAULT_OUTPUT_FILE_FORMAT: &str = "png";

#[derive(Debug, Parser)]
struct Args {
    /// Each square in a new color.
    #[arg(long, short)]
    rainbow: bool,
    /// How many profile pictures are to be generated.
    #[arg(long, short, default_value_t = 1)]
    count: usize,
    /// Scale up the image.
    #[arg(long, short, default_value_t = 1)]
    upscale: u8,
}

fn gen_image_names(count: usize) -> std::io::Result<Vec<String>> {
    let dir = env::current_dir()?.read_dir()?;
    let mut res = Vec::new();

    let mut max = 0;
    for file in dir {
        let name_binding = file?.file_name();
        let name = name_binding.to_str();

        if let Some(name) = name {
            let number = name
                .strip_prefix(DEFAULT_OUTPUT_FILE_NAME)
                .map(|name| name.strip_suffix(&(String::from(".") + DEFAULT_OUTPUT_FILE_FORMAT)));

            if let Some(Some(number)) = number {
                let number: Result<usize, _> = number.parse();

                match number {
                    Ok(number) if number > max => max = number,
                    _ => (),
                }
            }
        }
        // else skip
    }

    max += 1;
    for i in max..(max + count) {
        res.push(format!(
            "{}{}.{}",
            DEFAULT_OUTPUT_FILE_NAME, i, DEFAULT_OUTPUT_FILE_FORMAT
        ));
    }

    Ok(res)
}

fn main() {
    let args = Args::parse();

    if args.upscale == 0 {
        println!("upscale has to be greater than 0, can't generate an empty image.");
        return;
    }
    if args.count == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let output_file_names = match gen_image_names(args.count) {
        Ok(names) => names,
        _ => return,
    };

    println!("Generating profile picture ...");
    println!("first new: {}", output_file_names.get(0).unwrap());

    for current_count in 0..args.count {
        let mut gh_pfp = GhPfp::new(None);
        let squares_count = rng.gen_range(4..=11);
        let mut color: Option<Rgb<u8>> = None;

        for _ in 0..squares_count {
            color = match color {
                None => Some(Rgb(rng.gen())),
                Some(_) if args.rainbow => Some(Rgb(rng.gen())),
                val => val,
            };

            let mut position: (usize, usize);
            let mut painted = Vec::new();

            loop {
                position = (
                    rng.gen_range(0..model::HORIZONTAL_TO_GITHUB_PFP as usize),
                    rng.gen_range(0..model::PAINTED_SQUARES_IN_GITHUB_PFP as usize),
                );

                if painted.contains(&position) {
                    continue;
                }

                painted.push(position);
                break;
            }

            match gh_pfp.set_square(position, color) {
                Ok(_) => (),
                Err(err) => panic!("{}", err),
            }
        }

        gh_pfp
            .to_image(DEFAULT_SQUARE_LENGTH * args.upscale as u32)
            .save(output_file_names.get(current_count).unwrap())
            .unwrap();
    }
}
