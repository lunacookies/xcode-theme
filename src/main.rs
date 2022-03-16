use mottle::dsl::{s, FontStyle, ThemeBuilder};

fn main() -> anyhow::Result<()> {
    let mut theme_builder = ThemeBuilder::default();
    ui(&mut theme_builder, &UiPalette::DARK);
    editor(&mut theme_builder, &EditorPalette::XCODE_11_DEFAULT_DARK);
    let theme = theme_builder.build("Xcode 11 Default Dark");
    mottle::save_theme(&theme)?;

    let mut theme_builder = ThemeBuilder::default();
    ui(&mut theme_builder, &UiPalette::LIGHT);
    editor(&mut theme_builder, &EditorPalette::XCODE_11_DEFAULT_LIGHT);
    let theme = theme_builder.build("Xcode 11 Default Light");
    mottle::save_theme(&theme)?;

    Ok(())
}

fn ui(t: &mut ThemeBuilder, p: &UiPalette) {
    t.w(["foreground"], p.fg);

    t.w(["focusBorder"], p.focus_border);

    t.w(["statusBar.foreground"], p.status_fg);
    t.w(["statusBar.background"], p.status_bg);

    t.w(
        ["sideBar.background", "activityBar.background"],
        p.sidebar_bg,
    );
    t.w(["panel.background"], p.panel_bg);
    t.w(["sideBar.foreground"], p.light_fg);
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
        p.fg,
    );
    t.w(["tab.activeForeground"], p.active_tab_fg);
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

    t.w(["selection.background"], p.selection);

    t.w(["list.activeSelectionBackground"], p.active_list_bg);
    t.w(["list.hoverBackground"], (0x000000, 0x00));
    t.w(["list.highlightForeground"], p.filtered_list_fg);
    t.w(["list.focusHighlightForeground"], p.active_filtered_list_fg);

    t.w(["editorSuggestWidget.foreground"], p.suggest_fg);
    t.w(
        ["editorSuggestWidget.selectedForeground"],
        p.active_suggest_fg,
    );
    t.w(["editorWidget.background"], p.suggest_bg);
    t.w(["editorWidget.border"], p.light_border);

    t.w(["input.background"], p.text_field_bg);
    t.w(["input.placeholderForeground"], p.text_field_placeholder_fg);
    t.w(["input.border"], p.light_border);

    t.w(["scrollbar.shadow"], (0x000000, 0x00));
}

fn editor(t: &mut ThemeBuilder, p: &EditorPalette) {
    t.w(["editor.foreground"], p.fg);
    t.w(["editor.background"], p.bg);
    t.w(["editor.selectionBackground"], p.selection);
    t.w(
        ["editorCursor.foreground", "terminalCursor.foreground"],
        p.cursor,
    );
    t.w(
        ["editorCursor.background", "terminalCursor.background"],
        p.bg,
    );
    t.w(["editor.lineHighlightBackground"], p.current_line_bg);
    t.w(["editorWhitespace.foreground"], p.invisibles);

    t.a([s("comment")], p.comments);
    t.a([s("string")], p.strings);
    t.a([s("character")], p.characters);
    t.a([s("number")], p.numbers);
    t.a([s("keyword")], (p.keywords, FontStyle::Bold));
    t.a([], p.preprocessor_statements); // TODO

    // Xcode doesn’t have specific styling for namespaces (it highlights them like types)
    // and enum members (it highlights them like constants)

    t.a(
        [
            s("type.declaration"),
            s("class.declaration"),
            s("struct.declaration"),
            s("enum.declaration"),
            s("union.declaration"),
            s("interface.declaration"),
            s("typeParameter.declaration"),
            s("typeAlias.declaration"),
            s("namespace.declaration"),
        ],
        p.type_declarations,
    );

    t.a(
        [
            s("variable.declaration"),
            s("parameter.declaration"),
            s("property.declaration"),
            s("function.declaration"),
            s("method.declaration"),
            s("constParameter.declaration"),
        ],
        p.other_declarations,
    );

    t.a(
        [
            s("type"),
            s("class"),
            s("struct"),
            s("enum"),
            s("union"),
            s("interface"),
            s("typeParameter"),
            s("typeAlias"),
            s("namespace"),
        ],
        p.project_types,
    );
    t.a(
        [
            s("type.library"),
            s("class.library"),
            s("struct.library"),
            s("enum.library"),
            s("union.library"),
            s("interface.library"),
            s("typeParameter.library"),
            s("typeAlias.library"),
            s("namespace"),
            s("builtinType"),
        ],
        p.library_types,
    );

    t.a([s("function"), s("method")], p.project_functions);
    t.a(
        [
            s("function.library"),
            s("method.library"),
            // Xcode highlights overloadable operators as library functions
            s("arithmetic"),
            s("bitwise"),
            s("logical"),
            s("comparison"),
        ],
        p.library_functions,
    );

    t.a(
        [
            s("variable.constant"),
            s("variable.static"),
            s("enumMember"),
        ],
        p.project_constants,
    );
    t.a(
        [
            s("variable.constant.library"),
            s("variable.static.library"),
            s("enumMember.library"),
        ],
        p.library_constants,
    );

    t.a([s("property")], p.project_properties);
    t.a([s("property.library")], p.library_properties);

    t.a([s("macro"), s("derive")], p.project_macros);
    t.a([s("macro.library"), s("derive.library")], p.library_macros);
}

struct UiPalette {
    fg: u32,
    light_fg: u32,
    dark_border: u32,
    light_border: u32,
    focus_border: u32,
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
    active_tab_fg: u32,
    line_numbers: u32,
    active_line_number: u32,
    selection: u32,
    active_list_bg: u32,
    filtered_list_fg: u32,
    active_filtered_list_fg: u32,
    suggest_fg: (u32, u8),
    active_suggest_fg: (u32, u8),
    suggest_bg: u32,
    text_field_bg: u32,
    text_field_placeholder_fg: u32,
}

impl UiPalette {
    const DARK: Self = Self {
        fg: 0xDDDDDE,
        light_fg: 0xFFFFFF,
        dark_border: 0x000000,
        light_border: 0x36373B,
        focus_border: 0x427EA9,
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
        active_tab_fg: 0xFFFFFF,
        line_numbers: 0x747478,
        active_line_number: 0xE0E0E1,
        selection: 0x3F638B,
        active_list_bg: 0x1658BE,
        filtered_list_fg: 0xFFFFFF,
        active_filtered_list_fg: 0xFFFFFF,
        suggest_fg: (0xFFFFFF, 0xBB),
        active_suggest_fg: (0xFFFFFF, 0xBB),
        suggest_bg: 0x1E2023,
        text_field_bg: 0x1E1E1E,
        text_field_placeholder_fg: 0x727272,
    };

    const LIGHT: Self = Self {
        fg: 0x272727,
        light_fg: 0x363636,
        dark_border: 0xDEDEDE,
        light_border: 0xE6E6E6,
        focus_border: 0x8DB4FC,
        status_fg: 0x808080,
        status_bg: 0xFFFFFF,
        sidebar_bg: 0xE2E1E2,
        panel_bg: 0xEEEEEE,
        inactive_activitybar_fg: 0x636263,
        active_activitybar_fg: 0x0070F5,
        inactive_titlebar_fg: 0xA8A8A8,
        active_titlebar_fg: 0x4A4A4A,
        inactive_titlebar_bg: 0xE8E8E8,
        active_titlebar_bg: 0xF5F4F4,
        toolbar_bg: 0xFFFFFF,
        active_tab_bg: 0xD2E7FF,
        active_tab_fg: 0x007AFF,
        line_numbers: 0xA6A6A6,
        active_line_number: 0x232426,
        selection: 0xB3D7FF,
        active_list_bg: 0x59A2FF,
        filtered_list_fg: 0x000000,
        active_filtered_list_fg: 0xFFFFFF,
        suggest_fg: (0x3F3F3F, 0xFF),
        active_suggest_fg: (0xFFFFFF, 0xCC),
        suggest_bg: 0xE9E8E8,
        text_field_bg: 0xFFFFFF,
        text_field_placeholder_fg: 0xC0C0C0,
    };
}

struct EditorPalette {
    fg: (u32, u8),
    bg: u32,
    selection: u32,
    cursor: u32,
    current_line_bg: u32,
    invisibles: u32,
    comments: u32,
    strings: u32,
    characters: u32,
    numbers: u32,
    keywords: u32,
    preprocessor_statements: u32,
    type_declarations: u32,
    other_declarations: u32,
    project_types: u32,
    library_types: u32,
    project_functions: u32,
    library_functions: u32,
    project_constants: u32,
    library_constants: u32,
    project_properties: u32,
    library_properties: u32,
    project_macros: u32,
    library_macros: u32,
}

impl EditorPalette {
    const XCODE_11_DEFAULT_DARK: Self = Self {
        fg: (0xFFFFFF, (0.85 * 255.0) as u8), // Xcode uses a foreground of white with 85% opacity
        bg: 0x292A30,
        selection: 0x646F83,
        cursor: 0xFFFFFF,
        current_line_bg: 0x2F3239,
        invisibles: 0x53606E,
        comments: 0x7F8C98,
        strings: 0xFF8170,
        characters: 0xD9C97C,
        numbers: 0xD9C97C,
        keywords: 0xFF7AB2,
        preprocessor_statements: 0xFFA14F,
        type_declarations: 0x6BDFFF,
        other_declarations: 0x4EB0CC,
        project_types: 0xACF2E4,
        library_types: 0xDABAFF,
        project_functions: 0x78C2B3,
        library_functions: 0xB281EB,
        project_constants: 0x78C2B3,
        library_constants: 0xB281EB,
        project_properties: 0x78C2B3,
        library_properties: 0xB281EB,
        project_macros: 0xFFA14F,
        library_macros: 0xFFA14F,
    };

    const XCODE_11_DEFAULT_LIGHT: Self = Self {
        fg: (0x000000, u8::MAX),
        bg: 0xFFFFFF,
        selection: 0xB2D7FF,
        cursor: 0x000000,
        current_line_bg: 0xECF5FF,
        invisibles: 0xD6D6D6,
        comments: 0x707F8C,
        strings: 0xD12F1B,
        characters: 0x272AD8,
        numbers: 0x272AD8,
        keywords: 0xAD3DA4,
        preprocessor_statements: 0x78492A,
        type_declarations: 0x02638C,
        other_declarations: 0x057CB0,
        project_types: 0x23575C,
        library_types: 0x4B21B0,
        project_functions: 0x3E8087,
        library_functions: 0x804FB8,
        project_constants: 0x3E8087,
        library_constants: 0x804FB8,
        project_properties: 0x3E8087,
        library_properties: 0x804FB8,
        project_macros: 0x78492A,
        library_macros: 0x78492A,
    };
}
