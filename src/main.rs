extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate zip;
extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate clap;
extern crate regex;


use futures::{Future, Stream};
use futures::future::*;
use hyper::Client;
use tokio_core::reactor::Core;
use std::error::Error;
use zip::result::ZipResult;
use std::io::Read;
use std::path::Path;
use imageproc::drawing::draw_text_mut;
use image::{Rgba, RgbaImage};
use rusttype::{FontCollection, Scale, Point};
use clap::App;
use std::io::ErrorKind;
use futures::future::err;
use regex::Regex;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const APP_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");


struct RenderConfig<'a> {
    email: &'a str,
    font: &'a str,
    output_file: &'a str,
    font_size: u32,
    font_color: &'a Rgba<u8>,
    background_color: &'a Rgba<u8>,
}

fn do_unzip(data: &[u8]) -> ZipResult<Vec<u8>> {
    let reader = std::io::Cursor::new(data);

    let mut zip = zip::ZipArchive::new(reader)?;
    let mut buf = Vec::new();

    for i in 0..zip.len(){
        let mut file = zip.by_index(i)?;
        println!("Font: {}", file.name());
        file.read_to_end(&mut buf)?;
    }

    Ok(buf)
}

fn render_text(config: &RenderConfig, font_data: &[u8]) -> std::io::Result<()>{
    let path = Path::new(config.output_file);

    let height = config.font_size as f32;
    let font = Vec::from(font_data);

    let font = FontCollection::from_bytes(font).into_font().ok_or(std::io::Error::new(ErrorKind::Other, "Error loading font"))?;

    let layout = font.layout(config.email, Scale {x: height, y: height }, Point {x: 0f32, y:0f32});
    let mut total_width = 0u32;

    if let Some(glyph) = layout.last() {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            total_width = bounding_box.max.x as u32;
        }
    }

    let mut image = RgbaImage::from_pixel(total_width, height as u32, *config.background_color);

    let scale = Scale { x: height, y: height };
    draw_text_mut(&mut image, *config.font_color, 0, 0, scale, &font, config.email);

    image.save(path)
}

fn get_content(config: &RenderConfig) -> hyper::Result<()> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let font_name = config.font.to_lowercase().replace(" ", "-");
    let uri = format!("http://google-webfonts-helper.herokuapp.com/api/fonts/{}?download=zip&formats=ttf&variants=regular", font_name).parse()?;

    let mut body: Vec<u8> = Vec::new();
    let work = client.get(uri).into_future().and_then(|res| {
        if res.status().is_success() {
            Either::A(res.body().collect().map(|chunks| {
                for chunk in &chunks {
                    body.extend(chunk.iter());
                }
                if let Ok(font_data) = do_unzip(body.as_slice()){
                   let _ = render_text(config, &font_data);
                }
            }))
        }
        else {
            Either::B(err((hyper::error::Error::Io(std::io::Error::new(ErrorKind::NotFound, format!("Font {} not found", config.font))))))
        }
    });

    core.run(work)

}

fn parse_hex_color_string(hex: &str, background_default: u8) -> Result<Rgba<u8>, &str> {
    let lowered = hex.to_lowercase();

    Regex::new("^#([a-z0-9]{2})([a-z0-9]{2})([a-z0-9]{2})([a-z0-9]{2})?$")
        .map_err(|_| "Invalid regex")
        .and_then(|re| {
            re.captures(&lowered)
                .ok_or("Incorrect color format")
                .map(|captures|{
                    let mut color: [u8; 4] = [background_default; 4];

                    for (i, capture) in captures.iter()
                        .skip(1)
                        .enumerate() {
                            if let Some(capture) = capture {
                                color[i] = u8::from_str_radix(capture.as_str(), 16).unwrap_or(0u8);
                            }
                        }

                    Rgba(color)

                })
        })
}

fn main() {
    let matches = App::new(APP_NAME.unwrap_or("unknown"))
                    .version(VERSION.unwrap_or("unknown"))
                    .author("Paul Cowie <paul.cowie@ntlworld.com>")
                    .args_from_usage(
                        "-f --font=[FONT]                    'Sets the text font'
                         -s --size=[FONT_SIZE]               'Sets the font size'
                         -o --output=[OUTPUT_FILE]           'Sets the output filename'
                         -c --text-color=[TEXT_COLOR]        'Sets color of text in #rrggbb(aa) format'
                         -b --background-color=[BG_COLOR]    'Sets color of background in #rrggbb(aa) format'
                         <EMAIL>                             'Sets the email address'"
                    ).get_matches();

    let font = matches.value_of("font").unwrap_or("inconsolata");
    let output_file = matches.value_of("output").unwrap_or("email.png");

    if let Ok(size) = matches.value_of("size").unwrap_or("16").parse::<u32>() {
        
        if let Some(email) = matches.value_of("EMAIL") {
            
            let font_color = if let Some(color_string) = matches.value_of("text-color") {
                match parse_hex_color_string(color_string, 255) {
                    Ok(color) => color,
                    Err(e)    => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                }
            }
            else {
                Rgba([0u8, 0u8, 0u8, 255u8])
            };
            
            let bg_color = if let Some(color_string) = matches.value_of("background-color") {
                match parse_hex_color_string(color_string, 255) {
                    Ok(color) => color,
                    Err(e)    => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                }
            }
            else {
                Rgba([0u8, 0u8, 0u8,0u8])
            };

            match get_content(&RenderConfig {email: email,
                                             output_file: output_file, 
                                             font: font, 
                                             font_size: size, 
                                             font_color: &font_color, 
                                             background_color: &bg_color}) {
                Ok(_) => println!("Success!"),
                Err(e) => println!("{}", e.description()),
            }
        }
    }
}
