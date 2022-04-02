use crossbeam;
use image::{DynamicImage, GenericImage};
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jobjectArray, JNI_TRUE};
use jni::JNIEnv;
use rand::distributions::{Alphanumeric, DistString};
use reqwest;
use reqwest::Error;
use serde_json::Value;
use std::{io::Cursor, thread, time::Duration};

const RETRY_TIMEOUT_MS: u64 = 150;

fn get_skin_url(user_uuid: &str) -> Result<String, Error> {
    let url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{user_uuid}");
    let response: Value = reqwest::blocking::get(url)?.json().unwrap();
    let skin_value: &str = response["properties"][0]["value"].as_str().unwrap();
    let decoded: Vec<u8> = base64::decode(&skin_value).unwrap();
    let skin_info: &str = std::str::from_utf8(&decoded).unwrap();
    let skin_url: Value = serde_json::from_str(skin_info).unwrap();
    let tex_url: &str = skin_url["textures"]["SKIN"]["url"].as_str().unwrap();
    Ok(tex_url.to_string())
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
    let mut part_img = img
        .sub_image(src.x, src.y, src.width, src.height)
        .to_image();
    if src != dst {
        part_img =
            image::imageops::resize(&part_img, dst.width, dst.height, image::imageops::Nearest);
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

fn process_skin(idx: usize, mut img: DynamicImage, slim: bool) -> DynamicImage {
    img.put_pixel(0, 0, image::Rgba([idx as u8, 0, 0, 255]));

    let img: DynamicImage = match idx {
        1 => {
            // Left arm
            let mut mod_img = img;
            if !slim {
                mod_img = move_image(Rect::new(36, 48, 8, 4), Rect::new(8, 0, 16, 8), mod_img);
                mod_img = move_image(Rect::new(32, 52, 16, 12), Rect::new(0, 8, 32, 8), mod_img);

                // 2nd layer
                mod_img = move_image(
                    Rect::new(36 + 16, 48, 8, 4),
                    Rect::new(8 + 32, 0, 16, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(32 + 16, 52, 16, 12),
                    Rect::new(0 + 32, 8, 32, 8),
                    mod_img,
                );
            } else {
                mod_img = move_image(Rect::new(36, 48, 3, 4), Rect::new(8, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(39, 48, 3, 4), Rect::new(16, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(36, 52, 3, 12), Rect::new(8, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(39, 52, 4, 12), Rect::new(16, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(32, 52, 4, 12), Rect::new(0, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(43, 52, 3, 12), Rect::new(24, 8, 8, 8), mod_img);

                // 2nd layer
                mod_img = move_image(
                    Rect::new(36 + 16, 48, 3, 4),
                    Rect::new(8 + 32, 0, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(39 + 16, 48, 3, 4),
                    Rect::new(16 + 32, 0, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(36 + 16, 52, 3, 12),
                    Rect::new(8 + 32, 8, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(39 + 16, 52, 4, 12),
                    Rect::new(16 + 32, 8, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(32 + 16, 52, 4, 12),
                    Rect::new(0 + 32, 8, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(43 + 16, 52, 3, 12),
                    Rect::new(24 + 32, 8, 8, 8),
                    mod_img,
                );
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
                mod_img = move_image(
                    Rect::new(36 + 16, 48, 8, 4),
                    Rect::new(8 + 32, 0, 16, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(32 + 16, 52, 16, 12),
                    Rect::new(0 + 32, 8, 32, 8),
                    mod_img,
                );
            } else {
                mod_img = move_image(Rect::new(44, 16, 3, 4), Rect::new(8, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(47, 16, 3, 4), Rect::new(16, 0, 8, 8), mod_img);
                mod_img = move_image(Rect::new(44, 20, 3, 12), Rect::new(8, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(47, 20, 4, 12), Rect::new(16, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(40, 20, 4, 12), Rect::new(0, 8, 8, 8), mod_img);
                mod_img = move_image(Rect::new(51, 20, 3, 12), Rect::new(24, 8, 8, 8), mod_img);

                // 2nd layer
                mod_img = move_image(
                    Rect::new(44, 16 + 16, 3, 4),
                    Rect::new(8 + 32, 0, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(47, 16 + 16, 3, 4),
                    Rect::new(16 + 32, 0, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(44, 20 + 16, 3, 12),
                    Rect::new(8 + 32, 8, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(47, 20 + 16, 4, 12),
                    Rect::new(16 + 32, 8, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(40, 20 + 16, 4, 12),
                    Rect::new(0 + 32, 8, 8, 8),
                    mod_img,
                );
                mod_img = move_image(
                    Rect::new(51, 20 + 16, 3, 12),
                    Rect::new(24 + 32, 8, 8, 8),
                    mod_img,
                );
            }
            mod_img
        }
        3 => {
            // Left leg
            let mut mod_img = img;
            mod_img = move_image(Rect::new(20, 48, 8, 4), Rect::new(8, 0, 16, 8), mod_img);
            mod_img = move_image(Rect::new(16, 52, 16, 12), Rect::new(0, 8, 32, 8), mod_img);

            // 2nd layer
            mod_img = move_image(
                Rect::new(20 - 16, 48, 8, 4),
                Rect::new(8 + 32, 0, 16, 8),
                mod_img,
            );
            mod_img = move_image(
                Rect::new(16 - 16, 52, 16, 12),
                Rect::new(0 + 32, 8, 32, 8),
                mod_img,
            );

            mod_img
        }
        4 => {
            // Right leg
            let mut mod_img = img;
            mod_img = move_image(Rect::new(20, 16, 16, 4), Rect::new(8, 0, 16, 8), mod_img);
            mod_img = move_image(Rect::new(16, 20, 24, 12), Rect::new(0, 8, 32, 8), mod_img);

            // 2nd layer
            mod_img = move_image(
                Rect::new(20, 16 + 16, 16, 4),
                Rect::new(8 + 32, 0, 16, 8),
                mod_img,
            );
            mod_img = move_image(
                Rect::new(16, 20 + 16, 24, 12),
                Rect::new(0 + 32, 8, 32, 8),
                mod_img,
            );

            mod_img
        }
        5 => {
            let mut mod_img = img;
            mod_img = move_image(Rect::new(4, 16, 8, 4), Rect::new(8, 0, 16, 8), mod_img);
            mod_img
        }
        _ => panic!("Invalid body part caused by invalid index"),
    };

    img
}

fn generate_skin(uuid: &str, slim: bool) -> Vec<(String, String)> {
    let tex_url = get_skin_url(uuid).unwrap();
    let image_bytes = reqwest::blocking::get(tex_url).unwrap().bytes().unwrap();
    let image = image::load_from_memory(&image_bytes).unwrap();
    //DEBUG: let image = image::io::Reader::open("test_skin.png").unwrap().decode().unwrap();

    let variant = if slim { "slim" } else { "classic" };
    let tex_data_arr = crossbeam::scope(|scope| -> Vec<(String, String)> {
        let mut data_arr: Vec<(String, String)> = vec![];
        for idx in 1..=5 {
            let img = image.clone();
            let handle = scope.spawn(move |_| -> Option<(String, String)> {
                let img = process_skin(idx, img, slim);
                let mut buf: Vec<u8> = Vec::new();
                img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
                    .unwrap();
                let client = reqwest::blocking::Client::builder()
                    .user_agent("joebama/1.0")
                    .build()
                    .unwrap();
                //TODO: need to add header with "Authorization: Bearer <your key>"
                loop {
                    let resp = loop {
                        let form = reqwest::blocking::multipart::Form::new()
                            .text("variant", variant)
                            .text(
                                "name",
                                Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
                            )
                            .text("visibility", "1");
                        let image = reqwest::blocking::multipart::Part::bytes(buf.clone())
                            .mime_str("image/png")
                            .unwrap()
                            .file_name("joebama_skin.png");
                        let form = form.part("file", image);
                        let resp = client
                            .post("https://api.mineskin.org/generate/upload")
                            .multipart(form)
                            .send();
                        if let Ok(response) = resp {
                            break response;
                        }
                        thread::sleep(Duration::from_millis(RETRY_TIMEOUT_MS / 4));
                    };
                    let response: Value = resp.json().unwrap();
                    if let Some(data) = response.get("data") {
                        if let Some(data) = data.get("texture") {
                            let signature = data.get("signature").unwrap().as_str().unwrap();
                            let value = data.get("value")?.as_str()?;
                            return Some((signature.to_string(), value.to_string()));
                        } else {
                            thread::sleep(Duration::from_millis(RETRY_TIMEOUT_MS));
                        }
                    } else {
                        thread::sleep(Duration::from_millis(RETRY_TIMEOUT_MS));
                    }
                }
            });
            let data = handle.join().unwrap();
            data_arr.push(data.unwrap_or((String::new(), String::new())));
        }
        return data_arr;
    })
    .unwrap();
    tex_data_arr
}

#[no_mangle]
pub extern "system" fn Java_SGLib_generateSkin(
    env: JNIEnv,
    _class: JClass,
    input: JString,
    slim: jboolean,
) -> jobjectArray {
    let uuid: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();
    let slim = slim == JNI_TRUE;
    let processed_data = generate_skin(&uuid, slim);
    let obj_example = env
        .new_object_array(
            2,
            env.find_class("java/lang/String").unwrap(),
            env.new_string("").unwrap(),
        )
        .unwrap();
    let class = env.get_object_class(obj_example).unwrap();
    let outer = env.new_object_array(5, class, obj_example).unwrap();
    for i in 0..5 {
        let inner = env
            .new_object_array(
                2,
                env.find_class("java/lang/String").unwrap(),
                env.new_string("").unwrap(),
            )
            .unwrap();
        env.set_object_array_element(inner, 0, env.new_string(&processed_data[i].0).unwrap())
            .unwrap();
        env.set_object_array_element(inner, 1, env.new_string(&processed_data[i].1).unwrap())
            .unwrap();
        env.set_object_array_element(outer, i as i32, inner)
            .unwrap();
    }
    outer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let output = generate_skin("a1f943fc512e4b379f780ede2c823707", true);
        println!("{:?}", output);
    }
}
