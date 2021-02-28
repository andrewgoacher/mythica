use image::RgbaImage;
use image::ImageFormat;
use image::io::Reader as ImageReader;

pub struct Resource<'a> {
    base: &'a str
}

impl <'a> Resource<'a> {
    pub fn new(base: &'a str) -> Self {
        Self {
            base
        }
    }

    pub fn path(&self, path: &str) -> String {
        [self.base, path].join("/")
    }

    pub fn load_image_data(&self, path: &str) -> Option<RgbaImage> {
        let path = self.path(path);

        match ImageReader::open(path) {
            Ok(img) => {
                match img.decode() {
                    Err(e) => {
                        println!("{}", e);
                        None
                    },
                    Ok(img) => {
                        let rgba8 =  img.to_rgba8();
                        Some(rgba8)
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}