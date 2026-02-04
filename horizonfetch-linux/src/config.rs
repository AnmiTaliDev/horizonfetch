use std::{fs, io};

const DEFAULT_ASCII: &str = r#"


  1111111  1111111
  1111111  1111111
  1111111  1111111
ㅤ
  1111111  1111111
  1111111  1111111
  1111111  11111;.
"#;

#[derive(Debug, Clone)]
pub struct Config {
    pub ascii_art: String,
    pub color: String,
    pub info_color: String,
    pub title_color: String,
    pub show_user: bool,
    pub show_os: bool,
    pub show_uptime: bool,
    pub show_shell: bool,
    pub show_de: bool,
    pub show_screen: bool,
    pub show_motherboard: bool,
    pub show_cpu: bool,
    pub show_gpu: bool,
    pub show_ram: bool,
    pub show_swap: bool,
    pub show_locale: bool,
    pub show_disk: bool,
    pub show_ram_ext_info: bool,
    pub show_color_scheme: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ascii_art: DEFAULT_ASCII.to_string(),
            color: "34".to_string(),
            info_color: "38;5;117".to_string(),
            title_color: "38;5;110".to_string(),
            show_user: true,
            show_os: true,
            show_uptime: true,
            show_shell: true,
            show_de: true,
            show_screen: true,
            show_motherboard: true,
            show_cpu: true,
            show_gpu: true,
            show_ram: true,
            show_swap: true,
            show_locale: true,
            show_disk: true,
            show_ram_ext_info: false,
            show_color_scheme: true,
        }
    }
}

impl Config {
    pub fn load(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let ascii_art = extract_ascii_art(&content)
            .unwrap_or(DEFAULT_ASCII)
            .to_string();
        let color = extract_param(&content, "ascii_color")
            .unwrap_or("34")
            .to_string();
        let info_color = extract_param(&content, "info_color")
            .unwrap_or("38;5;117")
            .to_string();
        let title_color = extract_param(&content, "title_color")
            .unwrap_or("38;5;110")
            .to_string();

        let show_user = extract_bool(&content, "show_user", true);
        let show_os = extract_bool(&content, "show_os", true);
        let show_uptime = extract_bool(&content, "show_uptime", true);
        let show_shell = extract_bool(&content, "show_shell", true);
        let show_de = extract_bool(&content, "show_de", true);
        let show_screen = extract_bool(&content, "show_screen", true);
        let show_motherboard = extract_bool(&content, "show_motherboard", true);
        let show_cpu = extract_bool(&content, "show_cpu", true);
        let show_gpu = extract_bool(&content, "show_gpu", true);
        let show_ram = extract_bool(&content, "show_ram", true);
        let show_swap = extract_bool(&content, "show_swap", true);
        let show_locale = extract_bool(&content, "show_locale", true);
        let show_disk = extract_bool(&content, "show_disk", true);
        let show_ram_ext_info = extract_bool(&content, "show_ram_ext_info", false);
        let show_color_scheme = extract_bool(&content, "show_color_scheme", true);

        Ok(Config {
            ascii_art,
            color,
            info_color,
            title_color,
            show_user,
            show_os,
            show_uptime,
            show_shell,
            show_de,
            show_screen,
            show_motherboard,
            show_cpu,
            show_gpu,
            show_ram,
            show_swap,
            show_locale,
            show_disk,
            show_ram_ext_info,
            show_color_scheme,
        })
    }

    pub fn load_default() -> Self {
        let config_path = dirs::home_dir()
            .map(|p| p.join(".config/horizonfetch/hf.conf"))
            .and_then(|p| p.to_str().map(String::from));

        if let Some(path) = config_path {
            Self::load(&path).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

fn extract_ascii_art(content: &str) -> Option<&str> {
    let start = content.find("{|")? + 2;
    let end = content.find("|}")?;
    Some(&content[start..end])
}

fn extract_param<'a>(content: &'a str, param: &str) -> Option<&'a str> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with(param) {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let value = parts[1].trim();
                if value.starts_with('"') && value.ends_with('"') && value.len() > 2 {
                    return Some(&value[1..value.len() - 1]);
                } else if !value.is_empty() {
                    return Some(value);
                }
            }
        }
    }
    None
}

fn extract_bool(content: &str, param: &str, default: bool) -> bool {
    extract_param(content, param).map_or(default, |v| v == "true")
}

pub fn is_valid_ansi_code(code: &str) -> bool {
    if code.is_empty() {
        return false;
    }

    if let Ok(num) = code.parse::<u8>() {
        return matches!(num, 30..=37 | 40..=47 | 90..=97 | 100..=107);
    }

    let parts: Vec<&str> = code.split(';').collect();
    match parts.as_slice() {
        ["38", "5", color] if color.parse::<u8>().is_ok() => true,
        ["38", "2", r, g, b] => {
            r.parse::<u8>().is_ok() && g.parse::<u8>().is_ok() && b.parse::<u8>().is_ok()
        }
        _ => false,
    }
}
