use std::{fmt, fs, io};

// Background: #292A30
// Current Line: #2F3239
// Selection: #646F83
// Cursor: #FFFFFF
// Invisibles: #53606E
//
// Plain text: #DFDFE0 Medium
// Comments: #7F8C98 Medium
// Documentation Markup: #7F8C98 Medium
// Documentation Markup Keywords: #A3B1BF Bold
// Strings: #FF8170 Medium
// Characters: #D9C97C Medium
// Numbers: #D9C97C Medium
// Keywords: #FF7AB2 Bold
// Preprocessor Statements: #FFA14F Medium
// URLs: #6699FF Medium
// Attributes: #CC9768 Medium
// Project Types: #ACF2E4 Medium
// Project Functions, Vars and Constants: #78C2B3 Medium
// Other Types: #DABAFF Medium
// Other Functions, Vars and Constants: #B281EB Medium
// Type Declaractions: #6BDFFF Medium
// Other Declaractions: #4EB0CC Medium
const XCODE_11_DEFAULT_DARK: Theme = Theme {
    name: "Xcode 11 Default Dark",
    file_name: "Xcode-11-Default-Dark",
    kind: ThemeKind::Dark,
    background: Rgb(0x292A30),
    current_line: Rgb(0x2F3239),
    selection: Rgb(0x646F83),
    cursor: Rgb(0xFFFFFF),
    invisibles: Rgb(0x53606E),
    plain_text: Rgb(0xDFDFE0),
    comments: Rgb(0x7F8C98),
    are_comments_italic: false,
    doc_markup: Rgb(0x7F8C98),
    doc_markup_keywords: Rgb(0xA3B1BF),
    strings: Rgb(0xFF8170),
    chars: Rgb(0xD9C97C),
    numbers: Rgb(0xD9C97C),
    keywords: Rgb(0xFF7AB2),
    are_keywords_bold: true,
    preproc: Rgb(0xFFA14F),
    urls: Rgb(0x6699FF),
    attributes: Rgb(0xCC9768),
    types: Rgb(0xACF2E4),
    variables: Rgb(0x78C2B3),
    constants: Rgb(0x78C2B3),
    interfaces: Rgb(0xDABAFF),
    functions: Rgb(0xB281EB),
    type_decls: Some(Rgb(0x6BDFFF)),
    other_decls: Some(Rgb(0x4EB0CC)),
};

// Background: #292A30
// Current Line: #2F3239
// Selection: #646F83
// Cursor: #FFFFFF
// Invisibles: #53606E
//
// Plain text: #FFFFFF Medium
// Comments: #7F8C98 Medium Italic
// Documentation Markup: #7F8C98 Medium
// Documentation Markup Keywords: #A3B1BF Bold
// Strings: #FF8170 Medium
// Characters: #A79DF7 Medium
// Numbers: #A79DF7 Medium
// Keywords: #FF7AB2 Bold
// Preprocessor Statements: #FFA14F Medium
// URLs: #63B6FC Medium
// Attributes: #86BFA3 Medium
// Project Types and Vars: #A0D975 Medium
// Project Functions and Constants: #BAF28F Medium
// Other Types and Vars: #8AD1C3 Medium
// Other Functions and Constants: #A7EBDD Medium
//
// Definitions have the same colour as plain text.
const XCODE_10_DEFAULT_DARK: Theme = Theme {
    name: "Xcode 10 Default Dark",
    file_name: "Xcode-10-Default-Dark",
    kind: ThemeKind::Dark,
    background: Rgb(0x292A30),
    current_line: Rgb(0x2F3239),
    selection: Rgb(0x646F83),
    cursor: Rgb(0xFFFFFF),
    invisibles: Rgb(0x53606E),
    plain_text: Rgb(0xFFFFFF),
    comments: Rgb(0x7F8C98),
    are_comments_italic: true,
    doc_markup: Rgb(0x7F8C98),
    doc_markup_keywords: Rgb(0xA3B1BF),
    strings: Rgb(0xFF8170),
    chars: Rgb(0xA79DF7),
    numbers: Rgb(0xA79DF7),
    keywords: Rgb(0xFF7AB2),
    are_keywords_bold: true,
    preproc: Rgb(0xFFA14F),
    urls: Rgb(0x63B6FC),
    attributes: Rgb(0x86BFA3),
    types: Rgb(0xA0D975),
    variables: Rgb(0xBAF28F),
    constants: Rgb(0xBAF28F),
    interfaces: Rgb(0x8AD1C3),
    functions: Rgb(0xA7EBDD),
    type_decls: None,
    other_decls: None,
};

// Background: #292B36
// Current Line: #353749
// Selection: #445261
// Cursor: #FFFFFF
// Invisibles: #5F5F5F
//
// Plain text: #E7E8EB Regular
// Comments: #51C34F Regular
// Documentation Markup: #23AD68 Regular
// Documentation Markup Keywords: #35D585 Bold
// Strings: #DE3A3C Regular
// Characters: #8783BE Regular
// Numbers: #00AAA3 Regular
// Keywords: #E12DA0 Regular
// Preprocessor Statements: #D38D5D Regular
// URLs: #6544E9 Regular
// Attributes: #68878F Regular
// Project Types, Vars, Functions, Constants: #18B5B1 Regular
// Other Types, Vars, Functions, Constants: #29A09F Regular
//
// Definitions have the same colour as plain text.
const XCODE_CIVIC: Theme = Theme {
    name: "Xcode Civic",
    file_name: "Xcode-Civic",
    kind: ThemeKind::Dark,
    background: Rgb(0x292B36),
    current_line: Rgb(0x353749),
    selection: Rgb(0x445261),
    cursor: Rgb(0xFFFFFF),
    invisibles: Rgb(0x5F5F5F),
    plain_text: Rgb(0xE7E8EB),
    comments: Rgb(0x51C34F),
    are_comments_italic: false,
    doc_markup: Rgb(0x23AD68),
    doc_markup_keywords: Rgb(0x35D585),
    strings: Rgb(0xDE3A3C),
    chars: Rgb(0x8783BE),
    numbers: Rgb(0x00AAA3),
    keywords: Rgb(0xE12DA0),
    are_keywords_bold: false,
    preproc: Rgb(0xD38D5D),
    urls: Rgb(0x6544E9),
    attributes: Rgb(0x68878F),
    types: Rgb(0x18B5B1),
    variables: Rgb(0x18B5B1),
    constants: Rgb(0x18B5B1),
    interfaces: Rgb(0x29A09F),
    functions: Rgb(0x29A09F),
    type_decls: None,
    other_decls: None,
};

const TYPE_SCOPES: &[&str] = &[
    "type",
    "class",
    "struct",
    "enum",
    "enumMember",
    "typeAlias",
    "typeParameter",
    "union",
];

const VARIABLE_SCOPES: &[&str] = &["variable", "member", "parameter", "property", "lifetime"];

const KEYWORD_SCOPES: &[&str] = &["keyword", "boolean", "builtinType"];

const INVISIBLE: Rgba = Rgba {
    rgb: Rgb(0x000000),
    a: 0x00,
};

fn main() -> io::Result<()> {
    let themes = &[XCODE_11_DEFAULT_DARK, XCODE_10_DEFAULT_DARK, XCODE_CIVIC];

    for theme in themes {
        let json = theme.to_string();
        fs::write(format!("themes/{}-color-theme.json", theme.file_name), json)?;
    }

    Ok(())
}

struct Theme {
    name: &'static str,
    file_name: &'static str,
    kind: ThemeKind,
    background: Rgb,
    current_line: Rgb,
    selection: Rgb,
    cursor: Rgb,
    invisibles: Rgb,
    plain_text: Rgb,
    comments: Rgb,
    are_comments_italic: bool,
    doc_markup: Rgb,
    doc_markup_keywords: Rgb,
    strings: Rgb,
    chars: Rgb,
    numbers: Rgb,
    keywords: Rgb,
    are_keywords_bold: bool,
    preproc: Rgb,
    urls: Rgb,
    attributes: Rgb,
    types: Rgb,
    variables: Rgb,
    constants: Rgb,
    interfaces: Rgb,
    functions: Rgb,
    type_decls: Option<Rgb>,
    other_decls: Option<Rgb>,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x2A2C2F)
            }
        };
        let fg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xDFDFDF)
            }
        };
        let panel_border = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x000000)
            }
        };
        let active_tab_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x383A3D)
            }
        };
        let active_tab_fg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xFFFFFF)
            }
        };
        let inactive_tab_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x1F1F21)
            }
        };
        let inactive_tab_fg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x9A9C9D)
            }
        };
        let tab_border = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x5B5D5F)
            }
        };
        let editor_group_header = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x26282B)
            }
        };
        let titlebar_fg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xB7B8BB)
            }
        };
        let status_bar_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x1C1F21)
            }
        };
        let status_bar_fg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xDCDDDD)
            }
        };
        let snippet_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x007AFF)
            }
        };
        let hover_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x2C5392)
            }
        };
        let breakpoint = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x007AFF)
            }
        };
        let current_match_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xFFFB00)
            }
        };
        let other_match_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x545558)
            }
        };
        let widget_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x303030)
            }
        };
        let selected_item_in_widget_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x3071DB)
            }
        };
        let selected_item_in_inactive_widget_bg = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x4F5153)
            }
        };
        let matching_text_in_widget = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xF5C443)
            }
        };
        let error = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xE21514)
            }
        };
        let warning = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xF5C443)
            }
        };
        let git_change = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x4F82CE)
            }
        };
        let git_add = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x3CFF55)
            }
        };
        let git_delete = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0xFF5044)
            }
        };
        let focus_ring = || {
            if self.kind == ThemeKind::Light {
                Rgb(0x000000)
            } else {
                Rgb(0x35628B)
            }
        };

        fn write_scope(
            f: &mut fmt::Formatter<'_>,
            scope: impl AsRef<str>,
            val: impl fmt::Display,
        ) -> fmt::Result {
            writeln!(f, "\"{}\": {},", scope.as_ref(), val)
        }

        writeln!(f, "{{")?;

        write_scope(f, "name", format!("\"{}\"", self.name))?;
        write_scope(f, "type", self.kind)?;

        writeln!(f, "\"colors\": {{")?;

        write_scope(f, "foreground", fg())?;

        write_scope(f, "editor.background", self.background)?;
        write_scope(f, "editor.lineHighlightBackground", self.current_line)?;
        write_scope(f, "editor.selectionBackground", self.selection)?;
        write_scope(f, "minimap.selectionHighlight", self.selection)?;
        write_scope(f, "editorCursor.foreground", self.cursor)?;
        write_scope(f, "editorWhitespace.foreground", self.invisibles)?;
        write_scope(f, "editor.foreground", self.plain_text)?;

        write_scope(f, "editorCodeLens.foreground", Rgba { rgb: fg(), a: 0x77 })?;

        write_scope(f, "editorLineNumber.activeForeground", fg())?;
        write_scope(
            f,
            "editorLineNumber.foreground",
            Rgba { rgb: fg(), a: 0x55 },
        )?;

        write_scope(f, "peekView.border", selected_item_in_widget_bg())?;
        write_scope(f, "peekViewTitle.background", bg())?;
        write_scope(f, "peekViewEditor.background", self.background)?;
        write_scope(f, "peekViewResult.background", bg())?;
        write_scope(
            f,
            "peekViewResult.selectionBackground",
            selected_item_in_widget_bg(),
        )?;
        write_scope(f, "peekViewResult.selectionForeground", fg())?;
        write_scope(f, "peekViewResult.fileForeground", fg())?;
        write_scope(
            f,
            "peekViewResult.lineForeground",
            Rgba { rgb: fg(), a: 0x99 },
        )?;
        write_scope(f, "peekViewTitleDescription.foreground", self.urls)?;
        write_scope(f, "peekViewTitleLabel.foreground", fg())?;

        write_scope(f, "tab.activeBackground", active_tab_bg())?;
        write_scope(f, "tab.activeForeground", active_tab_fg())?;
        write_scope(f, "tab.inactiveBackground", inactive_tab_bg())?;
        write_scope(f, "tab.inactiveForeground", inactive_tab_fg())?;
        write_scope(f, "tab.border", tab_border())?;
        write_scope(f, "editorGroupHeader.tabsBackground", editor_group_header())?;
        write_scope(f, "editorGroupHeader.noTabsBackground", active_tab_bg())?;

        write_scope(f, "titleBar.activeBackground", active_tab_bg())?;
        write_scope(f, "titleBar.activeForeground", titlebar_fg())?;
        write_scope(f, "titleBar.border", tab_border())?;

        write_scope(f, "breadcrumb.background", status_bar_bg())?;
        write_scope(f, "breadcrumb.foreground", status_bar_fg())?;
        write_scope(f, "breadcrumb.focusForeground", status_bar_fg())?;

        write_scope(f, "statusBar.background", status_bar_bg())?;
        write_scope(f, "statusBar.debuggingBackground", status_bar_bg())?;
        write_scope(f, "statusBar.noFolderBackground", status_bar_bg())?;
        write_scope(f, "statusBar.foreground", status_bar_fg())?;
        write_scope(f, "statusBar.debuggingForeground", status_bar_fg())?;
        write_scope(f, "statusBar.noFolderForeground", status_bar_fg())?;

        write_scope(f, "activityBar.background", bg())?;
        write_scope(f, "activityBar.border", panel_border())?;

        write_scope(f, "sideBar.background", bg())?;
        write_scope(f, "sideBar.border", panel_border())?;
        write_scope(f, "sideBarSectionHeader.background", INVISIBLE)?;

        write_scope(f, "panel.border", panel_border())?;

        write_scope(f, "editorIndentGuide.background", INVISIBLE)?;
        write_scope(f, "editorIndentGuide.activeBackground", INVISIBLE)?;

        write_scope(f, "editor.snippetTabstopHighlightBackground", snippet_bg())?;

        write_scope(f, "editor.hoverHighlightBackground", hover_bg())?;

        write_scope(f, "debugIcon.breakpointForeground", breakpoint())?;
        write_scope(
            f,
            "debugIcon.breakpointCurrentStackframeForeground",
            breakpoint(),
        )?;
        write_scope(
            f,
            "editor.stackFrameHighlightBackground",
            Rgba {
                rgb: breakpoint(),
                a: 0x22,
            },
        )?;

        write_scope(f, "textLink.foreground", self.urls)?;
        write_scope(f, "textLink.activeForeground", self.urls)?;
        write_scope(f, "editorLink.activeForeground", self.urls)?;

        write_scope(f, "editor.findMatchBackground", current_match_bg())?;
        write_scope(f, "searchEditor.findMatchBackground", current_match_bg())?;
        write_scope(f, "editor.findMatchHighlightBackground", other_match_bg())?;
        write_scope(
            f,
            "peekViewEditor.matchHighlightBackground",
            other_match_bg(),
        )?;
        write_scope(
            f,
            "peekViewResult.matchHighlightBackground",
            other_match_bg(),
        )?;
        write_scope(f, "minimap.findMatchHighlight", current_match_bg())?;
        write_scope(
            f,
            "editorOverviewRuler.findMatchForeground",
            current_match_bg(),
        )?;

        write_scope(f, "editorWidget.background", widget_bg())?;
        write_scope(f, "list.focusBackground", selected_item_in_widget_bg())?;
        write_scope(f, "list.highlightForeground", matching_text_in_widget())?;
        write_scope(
            f,
            "list.activeSelectionBackground",
            selected_item_in_widget_bg(),
        )?;
        write_scope(
            f,
            "list.inactiveSelectionBackground",
            selected_item_in_inactive_widget_bg(),
        )?;

        write_scope(f, "errorForeground", error())?;
        write_scope(f, "editorError.foreground", error())?;
        write_scope(f, "minimap.errorHighlight", error())?;
        write_scope(f, "editorOverviewRuler.errorForeground", error())?;

        write_scope(f, "editorWarning.foreground", warning())?;
        write_scope(f, "minimap.warningHighlight", warning())?;
        write_scope(f, "editorOverviewRuler.warningForeground", warning())?;

        write_scope(f, "editorGutter.addedBackground", git_change())?;
        write_scope(f, "minimapGutter.addedBackground", git_change())?;
        write_scope(f, "editorOverviewRuler.addedForeground", git_change())?;

        write_scope(f, "gitDecoration.untrackedResourceForeground", fg())?;
        write_scope(f, "gitDecoration.addedResourceForeground", fg())?;
        write_scope(f, "gitDecoration.conflictingResourceForeground", error())?;
        write_scope(f, "gitDecoration.deletedResourceForeground", fg())?;
        write_scope(f, "gitDecoration.modifiedResourceForeground", fg())?;
        write_scope(f, "gitDecoration.submoduleResourceForeground", fg())?;
        write_scope(
            f,
            "gitDecoration.ignoredResourceForeground",
            Rgba { rgb: fg(), a: 0x55 },
        )?;

        write_scope(
            f,
            "diffEditor.insertedTextBackground",
            Rgba {
                rgb: git_add(),
                a: 0x0A,
            },
        )?;
        write_scope(
            f,
            "diffEditor.removedTextBackground",
            Rgba {
                rgb: git_delete(),
                a: 0x0A,
            },
        )?;

        write_scope(f, "focusBorder", focus_ring())?;

        writeln!(f, "}},")?;

        write_scope(f, "semanticHighlighting", "true")?;
        writeln!(f, "\"semanticTokenColors\": {{")?;

        write_scope(
            f,
            "comment",
            if self.are_comments_italic {
                format!(r#"{{ "italic": true, "foreground": {} }}"#, self.comments)
            } else {
                self.comments.to_string()
            },
        )?;

        write_scope(f, "string", self.strings)?;

        write_scope(f, "number", self.numbers)?;

        for scope in KEYWORD_SCOPES {
            write_scope(
                f,
                scope,
                if self.are_keywords_bold {
                    format!(r#"{{ "bold": true, "foreground": {} }}"#, self.keywords)
                } else {
                    self.keywords.to_string()
                },
            )?;
        }

        write_scope(f, "macro", self.preproc)?;

        for scope in TYPE_SCOPES {
            write_scope(f, scope, self.types)?;
        }

        for scope in VARIABLE_SCOPES {
            write_scope(f, scope, self.variables)?;
        }

        for scope in VARIABLE_SCOPES {
            write_scope(f, format!("{}.constant", scope), self.constants)?;
        }

        write_scope(f, "interface", self.interfaces)?;

        write_scope(f, "function", self.functions)?;

        write_scope(f, "punctuation", self.plain_text)?;

        write_scope(f, "operator", self.plain_text)?;

        for scope in TYPE_SCOPES {
            let color = self.type_decls.unwrap_or(self.plain_text);
            write_scope(f, format!("{}.declaration", scope), color)?;
        }

        write_scope(
            f,
            "*.declaration",
            self.other_decls.unwrap_or(self.plain_text),
        )?;

        writeln!(f, "}},")?;

        write_scope(f, "tokenColors", "[]")?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq)]
enum ThemeKind {
    Light,
    Dark,
}

impl fmt::Display for ThemeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Light => write!(f, "\"light\""),
            Self::Dark => write!(f, "\"dark\""),
        }
    }
}

struct Rgba {
    rgb: Rgb,
    a: u8,
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:06x}{:02x}\"", self.rgb.0, self.a)
    }
}

#[derive(Copy, Clone)]
struct Rgb(u32);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:06x}\"", self.0)
    }
}
