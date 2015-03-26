use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xml::Element;

///
/// Representation of a bitmap font, generated with a tool like
/// [BMFont](http://www.angelcode.com/products/bmfont/)
///
/// A BitmapFont describes a bitmap font texture, providing a mapping from character
/// codes to a rectangular area within a corresponding font texture that contains a
/// bitmap representation for that character code.
///
/// See http://www.angelcode.com/products/bmfont/doc/file_format.html for more information.
///
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

    ///
    /// Constructs a BitmapFont for the xml configuration file at the given path
    ///
    /// Expects file format like:
    ///
    /// ```xml
    /// <font>
    ///   <common scaleW="128" scaleH="128" ... />
    ///   <chars count="95">
    ///     <char id="32" x="2" y="2" width="0" height="0" xoffset="0" yoffset="14" xadvance="16" ... />
    ///     ...
    ///   </chars>
    /// ```
    ///
    /// See http://www.angelcode.com/products/bmfont/doc/file_format.html for more information.
    ///
    pub fn from_path(path: &Path) -> Result<BitmapFont, &'static str> {

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open font file at path.")
        };

        let mut xml_string = String::new();
        match file.read_to_string(&mut xml_string) {
            Ok(_) => {},
            Err(_) => return Err("Failed to read font file.")
        };

        BitmapFont::from_string(&xml_string[..])
    }

    ///
    /// Constructs a BitmapFont from the given string
    ///
    /// Expects string format like:
    ///
    /// ```xml
    /// <font>
    ///   <common scaleW="128" scaleH="128" ... />
    ///   <chars count="95">
    ///     <char id="32" x="2" y="2" width="0" height="0" xoffset="0" yoffset="14" xadvance="16" ... />
    ///     ...
    ///   </chars>
    /// ```
    ///
    /// See http://www.angelcode.com/products/bmfont/doc/file_format.html for more information.
    ///
    ///
    pub fn from_string(xml_string: &str) -> Result<BitmapFont, &'static str> {
        match xml_string.parse() {
            Ok(xml_root) =>  {
                BitmapFont::from_xml_document(&xml_root)
            },
            Err(_) => Err("Error while parsing font document."),
        }
    }

    ///
    /// Constructs a BitmapFont for the given root xml element
    ///
    fn from_xml_document(xml_root: &Element) -> Result<BitmapFont, &'static str> {
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
