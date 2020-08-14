use std::io::{self, Write};
use std::{fmt, fs};

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

fn main() -> io::Result<()> {
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
    let xcode_11_default_dark = Theme {
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

    let json = xcode_11_default_dark.to_string();

    fs::write(
        format!(
            "themes/{}-color-theme.json",
            xcode_11_default_dark.file_name
        ),
        json,
    )?;

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
        writeln!(f, "{{")?;

        writeln!(f, r#""name": "{}","#, self.name)?;
        writeln!(f, r#""type": "{}","#, self.kind)?;

        writeln!(f, r#""colors": {{"#)?;

        writeln!(f, r#""editor.background": {},"#, self.background)?;

        writeln!(
            f,
            r#""editor.lineHighlightBackground": {},"#,
            self.current_line
        )?;

        writeln!(f, r#""editor.selectionBackground": {},"#, self.selection)?;

        writeln!(f, r#""editorCursor.foreground": {},"#, self.cursor)?;

        writeln!(f, r#""editorWhitespace.foreground": {},"#, self.invisibles)?;

        writeln!(f, r#""editor.foreground": {},"#, self.plain_text)?;

        writeln!(f, "}},")?;

        writeln!(f, r#""semanticHighlighting": true,"#)?;
        writeln!(f, r#""semanticTokenColors": {{"#)?;

        write!(f, r#""comment": "#)?;
        if self.are_comments_italic {
            writeln!(f, r#"{{"italic":true,"foreground":{},}},"#, self.comments)?;
        } else {
            writeln!(f, "{},", self.comments)?;
        }

        writeln!(f, r#""string": {},"#, self.strings)?;

        writeln!(f, r#""number": {},"#, self.numbers)?;

        write!(f, r#""keyword": "#)?;
        if self.are_keywords_bold {
            writeln!(f, r#"{{"bold":true,"foreground":{},}},"#, self.keywords)?;
        } else {
            writeln!(f, "{},", self.keywords)?;
        }

        writeln!(f, r#""macro": {},"#, self.preproc)?;

        for scope in TYPE_SCOPES {
            writeln!(f, r#""{}": {},"#, scope, self.types)?;
        }

        for scope in VARIABLE_SCOPES {
            writeln!(f, r#""{}": {},"#, scope, self.variables)?;
        }

        for scope in VARIABLE_SCOPES {
            writeln!(f, r#""{}.constant": {},"#, scope, self.constants)?;
        }

        writeln!(f, r#""interface": {},"#, self.interfaces)?;

        writeln!(f, r#""function": {},"#, self.functions)?;

        for scope in TYPE_SCOPES {
            let color = self.type_decls.unwrap_or(self.plain_text);
            writeln!(f, r#""{}.declaration": {},"#, scope, color)?;
        }

        writeln!(
            f,
            r#""*.declaration": {},"#,
            self.other_decls.unwrap_or(self.plain_text)
        )?;

        writeln!(f, "}},")?;

        writeln!(f, r#""tokenColors": [],"#)?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

enum ThemeKind {
    Light,
    Dark,
}

impl fmt::Display for ThemeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Light => write!(f, "light"),
            Self::Dark => write!(f, "dark"),
        }
    }
}

#[derive(Copy, Clone)]
struct Rgb(u32);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"#{:x}\"", self.0)
    }
}
