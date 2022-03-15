use mottle::dsl::ThemeBuilder;

fn main() -> anyhow::Result<()> {
    let mut theme_builder = ThemeBuilder::default();
    workbench(&mut theme_builder, &Palette::XCODE_11_DEFAULT_DARK);
    let theme = theme_builder.build("Xcode 11 Default Dark");
    mottle::save_theme(&theme)?;

    Ok(())
}

fn workbench(t: &mut ThemeBuilder, p: &Palette) {
    t.w(["editor.background"], p.bg);
    t.w(["editor.foreground"], p.fg);
}

struct Palette {
    bg: u32,
    fg: (u32, u8),
}

impl Palette {
    const XCODE_11_DEFAULT_DARK: Self = Self {
        bg: 0x292A30,
        fg: (0xFFFFFF, (0.85 * 255.0) as u8), // Xcode uses a foreground of white with 85% opacity
    };
}
