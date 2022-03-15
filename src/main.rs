use mottle::dsl::ThemeBuilder;

fn main() -> anyhow::Result<()> {
    let mut theme_builder = ThemeBuilder::default();
    workbench(&mut theme_builder, &Palette::XCODE_11_DEFAULT_DARK);
    let theme = theme_builder.build("Xcode 11 Default Dark");
    mottle::save_theme(&theme)?;

    Ok(())
}

fn workbench(t: &mut ThemeBuilder, p: &Palette) {
    t.w(["editor.foreground"], p.fg);
    t.w(["editor.background"], p.bg);

    t.w(["statusBar.foreground"], p.status_fg);
    t.w(["statusBar.background"], p.status_bg);
}

struct Palette {
    fg: (u32, u8),
    bg: u32,
    status_fg: u32,
    status_bg: u32,
}

impl Palette {
    const XCODE_11_DEFAULT_DARK: Self = Self {
        fg: (0xFFFFFF, (0.85 * 255.0) as u8), // Xcode uses a foreground of white with 85% opacity
        bg: 0x292A30,
        status_fg: 0x9A9A9C,
        status_bg: 0x202124,
    };
}
