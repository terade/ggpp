use anyhow::{anyhow, Result};
use image::{ImageBuffer, Rgb, RgbImage};

pub const PAINTED_SQUARES_IN_GITHUB_PFP: u32 = 5;
pub const HORIZONTAL_TO_GITHUB_PFP: u32 = 3;
const SQUARES_IN_GITHUB_PFP: u32 = 6;
const RGB_WHITE: Rgb<u8> = Rgb([255_u8, 255_u8, 255_u8]);

pub struct GithubProfilePicture {
    positions: [Rgb<u8>; 25],
    default_color: Option<Rgb<u8>>,
}

pub type GhPfp = GithubProfilePicture;

impl GhPfp {
    pub fn new(default_color: Option<Rgb<u8>>) -> Self {
        Self {
            positions: [RGB_WHITE; 25],
            default_color,
        }
    }

    // square_length has to be even
    pub fn to_image(&self, square_length: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let total_size_xy = square_length * SQUARES_IN_GITHUB_PFP;
        let mut image = RgbImage::new(total_size_xy, total_size_xy);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = self.get_corresponding_color((x, y), square_length);
        }

        image
    }

    pub fn set_square(&mut self, position: (usize, usize), color: Option<Rgb<u8>>) -> Result<()> {
        let (x, y) = position;
        let color = match color {
            Some(color) => color,
            None => match self.default_color {
                Some(color) => color,
                None => return Err(anyhow!("no color provided and no default_color set")),
            },
        };

        if x > 2 {
            return Err(anyhow!("mirroring wont work this way"));
        }

        self.positions[x * PAINTED_SQUARES_IN_GITHUB_PFP as usize + y] = color;
        let x = PAINTED_SQUARES_IN_GITHUB_PFP as usize - x - 1;
        self.positions[x * PAINTED_SQUARES_IN_GITHUB_PFP as usize + y] = color;

        Ok(())
    }

    fn get_corresponding_color(&self, position: (u32, u32), square_length: u32) -> Rgb<u8> {
        let (x, y) = position;
        // border around the pfp
        if x <= square_length / 2_u32
            || (square_length as f32 * 5.5) as u32 <= x
            || y <= square_length / 2_u32
            || (square_length as f32 * 5.5) as u32 <= y
        {
            return RGB_WHITE;
        }

        let (x, y) = (x - square_length / 2, y - square_length / 2);
        let (x, y) = (x / square_length, y / square_length);

        self.positions[(x * PAINTED_SQUARES_IN_GITHUB_PFP + y) as usize]
    }
}
