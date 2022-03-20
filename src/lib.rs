use image::{DynamicImage, GenericImage};
use reqwest;
use reqwest::Error;
use serde_json::Value;
use crossbeam;


//? If bearer access token is greenlit switch over to 
//? GET https://api.minecraftservices.com/minecraft/profile

#[allow(dead_code)]
fn get_skin_url(user_uuid: &str) -> Result<(String, bool), Error> {
    let url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{user_uuid}");
    let response: Value = reqwest::blocking::get(url)?.json().unwrap();
    let skin_value: &str = response["properties"][0]["value"].as_str().unwrap();
    let decoded: Vec<u8> = base64::decode(&skin_value).unwrap();
    let skin_info: &str = std::str::from_utf8(&decoded).unwrap();
    let skin_url: Value = serde_json::from_str(skin_info).unwrap();
    let tex_url: &str = skin_url["textures"]["SKIN"]["url"].as_str().unwrap();
    let slim: bool = true; //TODO set true as temp because my skin is slim
    Ok((tex_url.to_string(), slim))
}

#[derive(PartialEq)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rect {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

fn move_image(src: Rect, dst: Rect, mut img: DynamicImage) -> DynamicImage {
    let mut part_img = img.sub_image(src.x, src.y, src.width, src.height).to_image();
    if src != dst {
        part_img = image::imageops::resize(&part_img, dst.width, dst.height, image::imageops::Nearest);
    }
    img.copy_from(&part_img, dst.x, dst.y).unwrap();
    img
}

/*
?LEFT_ARM 1
?RIGHT_ARM 2
?LEFT_LEG 3
?RIGHT_LEG 4
?TORSO 5
*/

#[allow(dead_code)]
fn process_skin(idx: usize, mut img: DynamicImage, slim: bool) {
    img.put_pixel(0, 0, image::Rgba([idx as u8, 0, 0, 255]));

    let img: DynamicImage = match idx {
        1 => {
            // Left arm
            let mut mod_img = img;
            if !slim {
                mod_img = move_image(Rect::new(36, 48, 8, 4), Rect::new(8, 0, 16, 8), mod_img);
                mod_img = move_image(Rect::new(32, 52, 16, 12), Rect::new(0, 8, 32, 8), mod_img);

                // 2nd layer
                mod_img = move_image(Rect::new(36 + 16, 48, 8, 4), Rect::new(8 + 32, 0, 16, 8), mod_img);
                mod_img = move_image(Rect::new(32 + 16, 52, 16, 12), Rect::new(0 + 32, 8, 32, 8), mod_img);

            } else {
                mod_img = move_image(Rect::new(36, 48, 3, 4), Rect::new(8, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(39, 48, 3, 4), Rect::new(16, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(36, 52, 3, 12), Rect::new(8, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(39, 52, 4, 12), Rect::new(16, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(32, 52, 4, 12), Rect::new(0, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(43, 52, 3, 12), Rect::new(24, 8, 8, 8), mod_img);

                // 2nd layer
                mod_img = move_image(Rect::new(36 + 16, 48, 3, 4), Rect::new(8 + 32, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(39 + 16, 48, 3, 4), Rect::new(16 + 32, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(36 + 16, 52, 3, 12), Rect::new(8 + 32, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(39 + 16, 52, 4, 12), Rect::new(16 + 32, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(32 + 16, 52, 4, 12), Rect::new(0 + 32, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(43 + 16, 52, 3, 12), Rect::new(24 + 32, 8, 8, 8), mod_img);

            }
            mod_img
        }
        2 => {
            // Right arm
            let mut mod_img = img;
            if !slim {
                mod_img = move_image(Rect::new(36, 48, 8, 4), Rect::new(8, 0, 16, 8), mod_img);
                mod_img = move_image(Rect::new(32, 52, 16, 12), Rect::new(0, 8, 32, 8), mod_img);

                // 2nd layer
                mod_img = move_image(Rect::new(36 + 16, 48, 8, 4), Rect::new(8 + 32, 0, 16, 8), mod_img);
                mod_img = move_image(Rect::new(32 + 16, 52, 16, 12), Rect::new(0 + 32, 8, 32, 8), mod_img);

            } else {
                mod_img = move_image(Rect::new(44, 16, 3, 4), Rect::new(8, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(47, 16, 3, 4), Rect::new(16, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(44, 20, 3, 12), Rect::new(8, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(47, 20, 4, 12), Rect::new(16, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(40, 20, 4, 12), Rect::new(0, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(51, 20, 3, 12), Rect::new(24, 8, 8, 8), mod_img);

                // 2nd layer
                mod_img = move_image(Rect::new(44, 16 + 16, 3, 4), Rect::new(8 + 32, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(47, 16 + 16, 3, 4), Rect::new(16 + 32, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(44, 20 + 16, 3, 12), Rect::new(8 + 32, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(47, 20 + 16, 4, 12), Rect::new(16 + 32, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(40, 20 + 16, 4, 12), Rect::new(0 + 32, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(51, 20 + 16, 3, 12), Rect::new(24 + 32, 8, 8, 8), mod_img);

            }
            mod_img
        }
        3 => {
            // Left leg
        }
        _ => DynamicImage::new_rgb8(1, 1),
    };

    img.save(format!("output/{}.png", idx)).unwrap();
}

#[allow(dead_code)]
fn generate_skin(uuid: &str) {
    let (tex_url, slim) = get_skin_url(uuid).unwrap();
    let image_bytes = reqwest::blocking::get(tex_url)
        .unwrap()
        .bytes()
        .unwrap();
    let image = image::load_from_memory(&image_bytes).unwrap();
    crossbeam::scope(|scope| {
        for idx in 1..=5 {
            let img = image.clone();
            scope.spawn(move |_| process_skin(idx, img, slim));
        }
    }).unwrap();
    // should also return if the player is slim or not
    // dont wanna send another request because of ratelimiting so cache value of slim
}

#[test]
fn it_works() {
    generate_skin("a1f943fc512e4b379f780ede2c823707");
}
