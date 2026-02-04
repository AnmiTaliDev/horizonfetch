use crate::config::{is_valid_ansi_code, Config};
use crate::system::SystemInfo;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout};

pub struct Display {
    config: Config,
    info: SystemInfo,
}

impl Display {
    pub fn new(config: Config, info: SystemInfo) -> Self {
        Self { config, info }
    }

    pub fn render(&self) -> io::Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;

        let color = if is_valid_ansi_code(&self.config.color) {
            &self.config.color
        } else {
            "34"
        };
        let info_color = if is_valid_ansi_code(&self.config.info_color) {
            &self.config.info_color
        } else {
            "38;5;117"
        };
        let title_color = if is_valid_ansi_code(&self.config.title_color) {
            &self.config.title_color
        } else {
            "38;5;110"
        };

        // Render ASCII art
        let ascii_lines: Vec<_> = self.config.ascii_art.lines().collect();
        let max_art_line_len = ascii_lines
            .iter()
            .map(|line| {
                let v = strip_ansi_escapes::strip(line.as_bytes());
                String::from_utf8(v)
                    .map(|s| s.chars().count())
                    .unwrap_or_else(|_| line.chars().count())
            })
            .max()
            .unwrap_or(0);

        let art_width = max_art_line_len + 3;
        let mut y = 0;

        for line in ascii_lines {
            let trimmed_line = line.trim_end();
            if !trimmed_line.is_empty() {
                execute!(
                    stdout(),
                    MoveTo(0, y),
                    Print(format!("\x1b[{}m{}\x1b[0m", color, trimmed_line))
                )?;
                y += 1;
            }
        }

        // Render system info
        let mut info_y = 0;

        if self.config.show_user {
            execute!(
                stdout(),
                MoveTo(art_width as u16, info_y),
                Print(format!(
                    "\x1b[{}m{}@{}\x1b[0m",
                    info_color, self.info.username, self.info.hostname
                ))
            )?;
            info_y += 1;

            execute!(
                stdout(),
                MoveTo(art_width as u16, info_y),
                Print("\x1b[97m-------\x1b[0m")
            )?;
            info_y += 1;
        }

        if self.config.show_os {
            self.print_line(
                art_width,
                info_y,
                title_color,
                "OS:",
                info_color,
                &self.info.os_name,
            )?;
            info_y += 1;
        }

        if self.config.show_uptime {
            self.print_line(
                art_width,
                info_y,
                title_color,
                "Uptime:",
                info_color,
                &self.info.uptime,
            )?;
            info_y += 1;
        }

        if self.config.show_shell {
            self.print_line(
                art_width,
                info_y,
                title_color,
                "Shell:",
                info_color,
                &self.info.shell,
            )?;
            info_y += 1;
        }

        if self.config.show_de {
            self.print_line(
                art_width,
                info_y,
                title_color,
                "DE:",
                info_color,
                &self.info.de,
            )?;
            info_y += 1;
        }

        if self.config.show_screen {
            if let Some(ref screen) = self.info.screen {
                self.print_line(
                    art_width,
                    info_y,
                    title_color,
                    "Screen:",
                    info_color,
                    screen,
                )?;
                info_y += 1;
            }
        }

        if self.config.show_motherboard {
            let mobo = self.info.motherboard.as_deref().unwrap_or("Unknown");
            self.print_line(
                art_width,
                info_y,
                title_color,
                "Motherboard:",
                info_color,
                mobo,
            )?;
            info_y += 1;
        }

        if self.config.show_cpu {
            self.print_line(
                art_width,
                info_y,
                title_color,
                "Cpu:",
                info_color,
                &self.info.cpu,
            )?;
            info_y += 1;
        }

        if self.config.show_gpu {
            for (i, gpu) in self.info.gpu.iter().enumerate() {
                let label = if i == 0 { "Gpu:" } else { "    " };
                self.print_line(art_width, info_y, title_color, label, info_color, gpu)?;
                info_y += 1;
            }
        }

        if self.config.show_ram {
            let ram_info = format!(
                "{:.2} / {:.2}gb ({:.0}%)",
                self.info.ram_used_gb, self.info.ram_total_gb, self.info.ram_percent
            );
            self.print_line(
                art_width,
                info_y,
                title_color,
                "Ram:",
                info_color,
                &ram_info,
            )?;
            info_y += 1;
        }

        if self.config.show_swap {
            let swap_info = format!("{:.2}gb", self.info.swap_total_gb);
            self.print_line(
                art_width,
                info_y,
                title_color,
                "Swap:",
                info_color,
                &swap_info,
            )?;
            info_y += 1;
        }

        if self.config.show_locale {
            self.print_line(
                art_width,
                info_y,
                title_color,
                "Locale:",
                info_color,
                &self.info.locale,
            )?;
            info_y += 1;
        }

        if self.config.show_disk {
            let max_len = self
                .info
                .disks
                .iter()
                .map(|d| d.name.len())
                .max()
                .unwrap_or(1);
            for disk in &self.info.disks {
                execute!(
                    stdout(),
                    MoveTo(art_width as u16, info_y),
                    Print(format!(
                        "\x1b[{}mDisk:\x1b[0m \x1b[97m{:<width$}\x1b[0m \x1b[{}m{:>3}gb\x1b[0m \x1b[97m/\x1b[0m \x1b[{}m{:>3}gb ({}%)\x1b[0m",
                        title_color,
                        disk.name,
                        info_color,
                        disk.used_gb,
                        info_color,
                        disk.total_gb,
                        disk.percent,
                        width = max_len
                    ))
                )?;
                info_y += 1;
            }
        }

        if self.config.show_color_scheme {
            info_y += 1;
            self.print_color_scheme(art_width, info_y)?;
            info_y += 2;
        }

        execute!(stdout(), MoveTo(0, y.max(info_y) + 1))?;
        Ok(())
    }

    fn print_line(
        &self,
        x: usize,
        y: u16,
        title_color: &str,
        title: &str,
        value_color: &str,
        value: &str,
    ) -> io::Result<()> {
        execute!(
            stdout(),
            MoveTo(x as u16, y),
            Print(format!(
                "\x1b[{}m{}\x1b[0m \x1b[{}m{}\x1b[0m",
                title_color, title, value_color, value
            ))
        )
    }

    fn print_color_scheme(&self, x: usize, y: u16) -> io::Result<()> {
        let top_colors = [0, 91, 92, 93, 94, 95, 96, 97];
        let bottom_colors = [30, 31, 32, 33, 34, 35, 36, 37];

        let top_line: String = top_colors
            .iter()
            .map(|&c| {
                if c == 0 {
                    "   ".to_string()
                } else {
                    format!("\x1b[{}m███\x1b[0m", c)
                }
            })
            .collect();

        let bottom_line: String = bottom_colors
            .iter()
            .map(|&c| format!("\x1b[{}m███\x1b[0m", c))
            .collect();

        execute!(
            stdout(),
            MoveTo(x as u16, y),
            Print(&top_line),
            MoveTo(x as u16, y + 1),
            Print(&bottom_line)
        )
    }
}
