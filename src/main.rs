use mottle::dsl::ThemeBuilder;

fn main() -> anyhow::Result<()> {
    let mut theme_builder = ThemeBuilder::default();
    workbench(&mut theme_builder, &Palette::XCODE_11_DEFAULT_DARK);
    let theme = theme_builder.build("Xcode 11 Default Dark");
    mottle::save_theme(&theme)?;

    Ok(())
}

fn workbench(t: &mut ThemeBuilder, p: &Palette) {
    t.w(["foreground"], p.ui_fg);
    t.w(["editor.foreground"], p.fg);
    t.w(["editor.background"], p.bg);
    t.w(["focusBorder"], p.focus_border);

    t.w(["editor.lineHighlightBackground"], p.current_line_bg);

    t.w(["statusBar.foreground"], p.status_fg);
    t.w(["statusBar.background"], p.status_bg);

    t.w(
        ["sideBar.background", "activityBar.background"],
        p.sidebar_bg,
    );
    t.w(["panel.background"], p.panel_bg);
    t.w(["sideBar.foreground"], p.light_ui_fg);
    t.w(
        ["sideBar.border", "activityBar.border", "panel.border"],
        p.dark_border,
    );
    t.w(
        ["activityBar.inactiveForeground"],
        p.inactive_activitybar_fg,
    );
    t.w(["activityBar.foreground"], p.active_activitybar_fg);
    t.w(["activityBar.activeBorder"], (0x000000, 0x00));

    t.w(["titleBar.inactiveForeground"], p.inactive_titlebar_fg);
    t.w(["titleBar.activeForeground"], p.active_titlebar_fg);
    t.w(["titleBar.inactiveBackground"], p.inactive_titlebar_bg);
    t.w(["titleBar.activeBackground"], p.active_titlebar_bg);
    t.w(["titleBar.border"], p.dark_border);

    t.w(
        [
            "tab.inactiveForeground",
            "breadcrumb.foreground",
            "breadcrumb.focusForeground",
        ],
        p.ui_fg,
    );
    t.w(["tab.activeForeground"], p.light_ui_fg);
    t.w(
        [
            "tab.inactiveBackground",
            "editorGroupHeader.tabsBackground",
            "breadcrumb.background",
        ],
        p.toolbar_bg,
    );
    t.w(["tab.activeBackground"], p.active_tab_bg);
    t.w(["tab.border", "editorGroupHeader.border"], p.light_border);

    t.w(["editorLineNumber.foreground"], p.line_numbers);
    t.w(["editorLineNumber.activeForeground"], p.active_line_number);

    t.w(
        ["editorCursor.foreground", "terminalCursor.foreground"],
        p.cursor,
    );
    t.w(
        ["editorCursor.background", "terminalCursor.background"],
        p.bg,
    );

    t.w(["editor.selectionBackground"], p.selection);
    t.w(["selection.background"], p.ui_selection);

    t.w(["list.activeSelectionBackground"], p.active_list_bg);
    t.w(["list.hoverBackground"], (0x000000, 0x00));
    t.w(["list.highlightForeground"], p.filtered_list_fg);

    t.w(
        [
            "editorSuggestWidget.foreground",
            "editorSuggestWidget.selectedForeground",
        ],
        p.suggest_fg,
    );
    t.w(["editorWidget.background"], p.suggest_bg);
    t.w(["editorWidget.border"], p.light_border);

    t.w(["input.background"], p.text_field_bg);
    t.w(["input.placeholderForeground"], p.text_field_placeholder_fg);
    t.w(["input.border"], p.light_border);

    t.w(["scrollbar.shadow"], (0x000000, 0x00));
}

struct Palette {
    ui_fg: u32,
    light_ui_fg: u32,
    fg: (u32, u8),
    bg: u32,
    dark_border: u32,
    light_border: u32,
    focus_border: u32,
    current_line_bg: u32,
    status_fg: u32,
    status_bg: u32,
    sidebar_bg: u32,
    panel_bg: u32,
    inactive_activitybar_fg: u32,
    active_activitybar_fg: u32,
    inactive_titlebar_fg: u32,
    active_titlebar_fg: u32,
    inactive_titlebar_bg: u32,
    active_titlebar_bg: u32,
    toolbar_bg: u32,
    active_tab_bg: u32,
    line_numbers: u32,
    active_line_number: u32,
    cursor: u32,
    selection: u32,
    ui_selection: u32,
    active_list_bg: u32,
    filtered_list_fg: u32,
    suggest_fg: (u32, u8),
    suggest_bg: u32,
    text_field_bg: u32,
    text_field_placeholder_fg: u32,
}

impl Palette {
    const XCODE_11_DEFAULT_DARK: Self = Self {
        ui_fg: 0xDDDDDE,
        light_ui_fg: 0xFFFFFF,
        fg: (0xFFFFFF, (0.85 * 255.0) as u8), // Xcode uses a foreground of white with 85% opacity
        bg: 0x292A30,
        dark_border: 0x000000,
        light_border: 0x36373B,
        focus_border: 0x427EA9,
        current_line_bg: 0x2F3239,
        status_fg: 0x9A9A9C,
        status_bg: 0x202124,
        sidebar_bg: 0x26282B,
        panel_bg: 0x1E2023,
        inactive_activitybar_fg: 0xA6A7A9,
        active_activitybar_fg: 0x007AFF,
        inactive_titlebar_fg: 0x686A6D,
        active_titlebar_fg: 0xEBEBEB,
        inactive_titlebar_bg: 0x27292C,
        active_titlebar_bg: 0x37383B,
        toolbar_bg: 0x202125,
        active_tab_bg: 0x335274,
        line_numbers: 0x747478,
        active_line_number: 0xE0E0E1,
        cursor: 0xFFFFFF,
        selection: 0x646F83,
        ui_selection: 0x3F638B,
        active_list_bg: 0x1658BE,
        filtered_list_fg: 0xFFFFFF,
        suggest_fg: (0xFFFFFF, 0xBB),
        suggest_bg: 0x1E2023,
        text_field_bg: 0x1E1E1E,
        text_field_placeholder_fg: 0x727272,
    };
}
