use std::collections::HashMap;
use std::default::Default;
use std::old_io::{File};

use xml::Element;

pub struct BitmapFont {
    pub scale_w: u8,
    pub scale_h: u8,
    pub characters: HashMap<char, BitmapCharacter>
}

#[derive(Debug)]
#[derive(Default)]
pub struct BitmapCharacter {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
    pub xoffset: u8,
    pub yoffset: u8,
    pub xadvance: u8,
}

impl BitmapFont {

    pub fn from_path(path: &Path) -> Result<BitmapFont, &'static str> {
        let xml_root = try!(parse_file(path));

        let chars_element = match xml_root.get_child("chars", None) {
            Some(chars_element) => chars_element,
            None => return Err("Missing <chars> element"),
        };

        let common_element = match xml_root.get_child("common", None) {
            Some(e) => e,
            None => return Err("Missing <common> element"),
        };

        let mut bitmap_font = BitmapFont{
            characters: HashMap::new(),
            scale_w: get_attribute(&common_element, "scaleW"),
            scale_h: get_attribute(&common_element, "scaleH"),
        };

        for char_elem in chars_element.get_children("char", None).iter() {
            let character = BitmapCharacter {
                x: get_attribute(char_elem, "x"),
                y: get_attribute(char_elem, "y"),
                width: get_attribute(char_elem, "width"),
                height: get_attribute(char_elem, "height"),
                xoffset: get_attribute(char_elem, "xoffset"),
                yoffset: get_attribute(char_elem, "yoffset"),
                xadvance: get_attribute(char_elem, "xadvance"),
            };
            let id = get_attribute(char_elem, "id");
            bitmap_font.characters.insert(id as char, character);
        }

        Ok(bitmap_font)
    }
}

///
/// Parse the file at the given path into an XML element tree
///
fn parse_file(path: &Path) -> Result<Element, &'static str> {

    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(_) => return Err("Failed to open font file at path.")
    };

    let xml_string = match file.read_to_string() {
        Ok(file_string) => file_string,
        Err(_) => return Err("Failed to read font file.")
    };

    match xml_string.parse() {
        Ok(root_element) => Ok(root_element),
        Err(_) => Err("Error while parsing font file."),
    }
}

///
/// Get a u8 value for for the attribute name on the given element,
/// defaulting to 0 if attribute unavaiable or failing to parse
///
fn get_attribute(element: &Element, name: &str) -> u8 {
    match element.get_attribute(name, None) {
        Some(value_string) => match value_string.parse() {
            Ok(value) => value,
            Err(_) => 0,
        },
        None => 0
    }
}
