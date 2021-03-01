use image::io::Reader as ImageReader;
use image::RgbaImage;

pub struct Resource<'a> {
    base: &'a str,
}

impl<'a> Resource<'a> {
    pub fn new(base: &'a str) -> Self {
        Self { base }
    }

    pub fn path(&self, path: &str) -> String {
        [self.base, path].join("/")
    }

    pub fn read_string(&self, path: &str) -> Option<String> {
        match std::fs::read_to_string(self.path(path)) {
            Err(_) => None,
            Ok(s) => Some(s),
        }
    }

    pub fn load_image_data(&self, path: &str) -> Option<RgbaImage> {
        let path = self.path(path);

        match ImageReader::open(path) {
            Ok(img) => match img.decode() {
                Err(e) => {
                    println!("{}", e);
                    None
                }
                Ok(img) => {
                    let rgba8 = img.to_rgba8();
                    Some(rgba8)
                }
            },
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
