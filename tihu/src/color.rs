use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/**
 * 颜色分量最大值是1.0，不是255
 */
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct RgbColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

/**
 * 颜色分量最大值是1.0，不是255
 */
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct RgbaColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct HslColor {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct HslaColor {
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
    pub alpha: f64,
}

fn normalize_degree(num: f64) -> f64 {
    let max = 360.0;
    let mut num = num % max;
    if 0.0 > num {
        num += max;
    }
    return num;
}

impl HslaColor {
    pub fn add_hue(&self, addon: f64) -> HslaColor {
        return HslaColor {
            hue: normalize_degree(self.hue + addon),
            saturation: self.saturation,
            lightness: self.lightness,
            alpha: self.alpha,
        };
    }
    pub fn add_saturation(&self, addon: f64) -> HslaColor {
        return HslaColor {
            hue: self.hue,
            saturation: (self.saturation + addon).max(0.0).min(1.0),
            lightness: self.lightness,
            alpha: self.alpha,
        };
    }
    pub fn add_lightness(&self, addon: f64) -> HslaColor {
        return HslaColor {
            hue: self.hue,
            saturation: self.saturation,
            lightness: (self.lightness + addon).max(0.0).min(1.0),
            alpha: self.alpha,
        };
    }
    pub fn to_css(&self) -> String {
        let saturation = (self.saturation * 10000.0).round() / 100.0;
        let lightness = (self.lightness * 10000.0).round() / 100.0;
        return format!(
            "hsla({}, {}%, {}%, {})",
            self.hue, saturation, lightness, self.alpha
        );
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Color {
    Rgb(RgbColor),
    Rgba(RgbaColor),
    Hsl(HslColor),
    Hsla(HslaColor),
}

fn parse_css_fn(color: &str) -> Option<(&str, Vec<&str>)> {
    if let Some(start) = color.find("(") {
        let fn_name = color.split_at(start).0;
        let rest = color.split_at(start + 1).1;
        if let Some(end) = rest.find(")") {
            let args = rest.split_at(end).0;
            return Some((
                fn_name.trim(),
                args.split(",").map(|item| item.trim()).collect(),
            ));
        }
    }
    return None;
}

fn parse_css_number(number: &str, max: Option<f64>) -> Option<f64> {
    if number.ends_with("%") {
        let number = number.split_at(number.len() - 1).0;
        f64::from_str(number).map(|number| number / 100.0).ok()
    } else {
        f64::from_str(number)
            .map(|number| number / max.unwrap_or(1.0))
            .ok()
    }
}

const MAX_RGB: f64 = 255.0;

fn parse_hex_color(color: &str) -> Option<Color> {
    let items: Vec<_> = color.chars().collect();
    let seeds = "0123456789abcdef";
    let data: [usize; 6] = if 4 == items.len() {
        [
            seeds.find(*items.get(1)?)?,
            seeds.find(*items.get(1)?)?,
            seeds.find(*items.get(2)?)?,
            seeds.find(*items.get(2)?)?,
            seeds.find(*items.get(3)?)?,
            seeds.find(*items.get(3)?)?,
        ]
    } else {
        [
            seeds.find(*items.get(1)?)?,
            seeds.find(*items.get(2)?)?,
            seeds.find(*items.get(3)?)?,
            seeds.find(*items.get(4)?)?,
            seeds.find(*items.get(5)?)?,
            seeds.find(*items.get(6)?)?,
        ]
    };
    let red = 16 * data[0] + data[1];
    let green = 16 * data[2] + data[3];
    let blue = 16 * data[4] + data[5];
    return Some(Color::Rgb(RgbColor {
        red: red as f64 / MAX_RGB,
        green: green as f64 / MAX_RGB,
        blue: blue as f64 / MAX_RGB,
    }));
}

pub fn parse_css_color(color: &str) -> Option<Color> {
    let color = color.trim().to_lowercase();
    if color.starts_with("#") {
        return parse_hex_color(&color);
    } else {
        if let Some((fn_name, args)) = parse_css_fn(&color) {
            match fn_name {
                "rgb" => {
                    let red = parse_css_number(args.get(0)?, Some(MAX_RGB))?;
                    let green = parse_css_number(args.get(1)?, Some(MAX_RGB))?;
                    let blue = parse_css_number(args.get(2)?, Some(MAX_RGB))?;
                    return Some(Color::Rgb(RgbColor { red, green, blue }));
                }
                "rgba" => {
                    let red = parse_css_number(args.get(0)?, Some(MAX_RGB))?;
                    let green = parse_css_number(args.get(1)?, Some(MAX_RGB))?;
                    let blue = parse_css_number(args.get(2)?, Some(MAX_RGB))?;
                    let alpha = parse_css_number(args.get(3)?, None)?;
                    return Some(Color::Rgba(RgbaColor {
                        red,
                        green,
                        blue,
                        alpha,
                    }));
                }
                "hsl" => {
                    let hue = parse_css_number(args.get(0)?, None)?;
                    let saturation = parse_css_number(args.get(1)?, Some(100.0))?;
                    let lightness = parse_css_number(args.get(2)?, Some(100.0))?;
                    return Some(Color::Hsl(HslColor {
                        hue,
                        saturation,
                        lightness,
                    }));
                }
                "hsla" => {
                    let hue = parse_css_number(args.get(0)?, None)?;
                    let saturation = parse_css_number(args.get(1)?, Some(100.0))?;
                    let lightness = parse_css_number(args.get(2)?, Some(100.0))?;
                    let alpha = parse_css_number(args.get(3)?, None)?;
                    return Some(Color::Hsla(HslaColor {
                        hue,
                        saturation,
                        lightness,
                        alpha,
                    }));
                }
                _ => {
                    return None;
                }
            };
        } else {
            let named_colors = get_named_colors();
            let color = named_colors.get(color.as_str())?;
            return parse_hex_color(color);
        }
    }
}

/**
 * 颜色分量最大值是1.0，不是255
 */
fn rgb_to_hsl(red: f64, green: f64, blue: f64, alpha: f64) -> HslaColor {
    let max = red.max(green).max(blue);
    let min = red.min(green).min(blue);
    let hue = {
        if max == min {
            0.0
        } else {
            if max == green {
                60.0 * (blue - red) / (max - min) + 120.0
            } else if max == blue {
                60.0 * (red - green) / (max - min) + 240.0
            } else {
                if green >= blue {
                    60.0 * (green - blue) / (max - min)
                } else {
                    60.0 * (green - blue) / (max - min) + 360.0
                }
            }
        }
    };
    let lightness = 0.5 * (max + min);
    let saturation = {
        if 0.0 == lightness || max == min {
            0.0
        } else if 0.0 < lightness && 0.5 >= lightness {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        }
    };
    return HslaColor {
        hue,
        saturation,
        lightness,
        alpha,
    };
}

fn get_named_colors() -> HashMap<&'static str, &'static str> {
    return vec![
        ("aliceblue", "#f0f8ff"),
        ("antiquewhite", "#faebd7"),
        ("aqua", "#00ffff"),
        ("aquamarine", "#7fffd4"),
        ("azure", "#f0ffff"),
        ("beige", "#f5f5dc"),
        ("bisque", "#ffe4c4"),
        ("black", "#000000"),
        ("blanchedalmond", "#ffebcd"),
        ("blue", "#0000ff"),
        ("blueviolet", "#8a2be2"),
        ("brown", "#a52a2a"),
        ("burlywood", "#deb887"),
        ("cadetblue", "#5f9ea0"),
        ("chartreuse", "#7fff00"),
        ("chocolate", "#d2691e"),
        ("coral", "#ff7f50"),
        ("cornflowerblue", "#6495ed"),
        ("cornsilk", "#fff8dc"),
        ("crimson", "#dc143c"),
        ("cyan", "#00ffff"),
        ("darkblue", "#00008b"),
        ("darkcyan", "#008b8b"),
        ("darkgoldenrod", "#b8860b"),
        ("darkgray", "#a9a9a9"),
        ("darkgreen", "#006400"),
        ("darkgrey", "#a9a9a9"),
        ("darkkhaki", "#bdb76b"),
        ("darkmagenta", "#8b008b"),
        ("darkolivegreen", "#556b2f"),
        ("darkorange", "#ff8c00"),
        ("darkorchid", "#9932cc"),
        ("darkred", "#8b0000"),
        ("darksalmon", "#e9967a"),
        ("darkseagreen", "#8fbc8f"),
        ("darkslateblue", "#483d8b"),
        ("darkslategray", "#2f4f4f"),
        ("darkslategrey", "#2f4f4f"),
        ("darkturquoise", "#00ced1"),
        ("darkviolet", "#9400d3"),
        ("deeppink", "#ff1493"),
        ("deepskyblue", "#00bfff"),
        ("dimgray", "#696969"),
        ("dimgrey", "#696969"),
        ("dodgerblue", "#1e90ff"),
        ("firebrick", "#b22222"),
        ("floralwhite", "#fffaf0"),
        ("forestgreen", "#228b22"),
        ("fuchsia", "#ff00ff"),
        ("gainsboro", "#dcdcdc"),
        ("ghostwhite", "#f8f8ff"),
        ("gold", "#ffd700"),
        ("goldenrod", "#daa520"),
        ("gray", "#808080"),
        ("green", "#008000"),
        ("greenyellow", "#adff2f"),
        ("grey", "#808080"),
        ("honeydew", "#f0fff0"),
        ("hotpink", "#ff69b4"),
        ("indianred", "#cd5c5c"),
        ("indigo", "#4b0082"),
        ("ivory", "#fffff0"),
        ("khaki", "#f0e68c"),
        ("lavender", "#e6e6fa"),
        ("lavenderblush", "#fff0f5"),
        ("lawngreen", "#7cfc00"),
        ("lemonchiffon", "#fffacd"),
        ("lightblue", "#add8e6"),
        ("lightcoral", "#f08080"),
        ("lightcyan", "#e0ffff"),
        ("lightgoldenrodyellow", "#fafad2"),
        ("lightgray", "#d3d3d3"),
        ("lightgreen", "#90ee90"),
        ("lightgrey", "#d3d3d3"),
        ("lightpink", "#ffb6c1"),
        ("lightsalmon", "#ffa07a"),
        ("lightseagreen", "#20b2aa"),
        ("lightskyblue", "#87cefa"),
        ("lightslategray", "#778899"),
        ("lightslategrey", "#778899"),
        ("lightsteelblue", "#b0c4de"),
        ("lightyellow", "#ffffe0"),
        ("lime", "#00ff00"),
        ("limegreen", "#32cd32"),
        ("linen", "#faf0e6"),
        ("magenta", "#ff00ff"),
        ("maroon", "#800000"),
        ("mediumaquamarine", "#66cdaa"),
        ("mediumblue", "#0000cd"),
        ("mediumorchid", "#ba55d3"),
        ("mediumpurple", "#9370db"),
        ("mediumseagreen", "#3cb371"),
        ("mediumslateblue", "#7b68ee"),
        ("mediumspringgreen", "#00fa9a"),
        ("mediumturquoise", "#48d1cc"),
        ("mediumvioletred", "#c71585"),
        ("midnightblue", "#191970"),
        ("mintcream", "#f5fffa"),
        ("mistyrose", "#ffe4e1"),
        ("moccasin", "#ffe4b5"),
        ("navajowhite", "#ffdead"),
        ("navy", "#000080"),
        ("oldlace", "#fdf5e6"),
        ("olive", "#808000"),
        ("olivedrab", "#6b8e23"),
        ("orange", "#ffa500"),
        ("orangered", "#ff4500"),
        ("orchid", "#da70d6"),
        ("palegoldenrod", "#eee8aa"),
        ("palegreen", "#98fb98"),
        ("paleturquoise", "#afeeee"),
        ("palevioletred", "#db7093"),
        ("papayawhip", "#ffefd5"),
        ("peachpuff", "#ffdab9"),
        ("peru", "#cd853f"),
        ("pink", "#ffc0cb"),
        ("plum", "#dda0dd"),
        ("powderblue", "#b0e0e6"),
        ("purple", "#800080"),
        ("rebeccapurple", "#663399"),
        ("red", "#ff0000"),
        ("rosybrown", "#bc8f8f"),
        ("royalblue", "#4169e1"),
        ("saddlebrown", "#8b4513"),
        ("salmon", "#fa8072"),
        ("sandybrown", "#f4a460"),
        ("seagreen", "#2e8b57"),
        ("seashell", "#fff5ee"),
        ("sienna", "#a0522d"),
        ("silver", "#c0c0c0"),
        ("skyblue", "#87ceeb"),
        ("slateblue", "#6a5acd"),
        ("slategray", "#708090"),
        ("slategrey", "#708090"),
        ("snow", "#fffafa"),
        ("springgreen", "#00ff7f"),
        ("steelblue", "#4682b4"),
        ("tan", "#d2b48c"),
        ("teal", "#008080"),
        ("thistle", "#d8bfd8"),
        ("tomato", "#ff6347"),
        ("turquoise", "#40e0d0"),
        ("violet", "#ee82ee"),
        ("wheat", "#f5deb3"),
        ("white", "#ffffff"),
        ("whitesmoke", "#f5f5f5"),
        ("yellow", "#ffff00"),
        ("yellowgreen", "#9acd32"),
    ]
    .into_iter()
    .collect();
}

pub fn calc_hsla_color(color: &str) -> Option<HslaColor> {
    let color = parse_css_color(color)?;
    let hsla = match color {
        Color::Rgb(color) => rgb_to_hsl(color.red, color.green, color.blue, 1.0),
        Color::Rgba(color) => rgb_to_hsl(color.red, color.green, color.blue, color.alpha),
        Color::Hsl(color) => HslaColor {
            hue: color.hue,
            saturation: color.saturation,
            lightness: color.lightness,
            alpha: 1.0,
        },
        Color::Hsla(color) => color,
    };
    return Some(hsla);
}
