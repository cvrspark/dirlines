use std::collections::HashMap;
use crate::file::Workspace;

#[derive(Default)]
pub struct Data {
    pub code: i64,
    pub comment: i64,
    pub void: i64,
}

enum Languages {
    CStyle, // // or /* */
    Hash,   // #
    Html,   // <!-- -->
    Sql,    // -- or /* */
    Json,   // no comments
}

impl Languages {
    fn from_ext(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "java" | "scala" | "kotlin" | "kt" | "swift" | "go" | "rs" | "cs"
            | "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "hh" | "c" | "h"
            | "ts" | "tsx" | "js" | "mjs" | "cjs" | "css" | "scss" | "less" => Some(Self::CStyle),

            "py" | "sh" | "bash" | "zsh" | "rb" | "ruby" | "pl" | "pm"
            | "yaml" | "yml" | "toml" | "r" | "jl" | "dockerfile" | "makefile" => Some(Self::Hash),

            "html" | "htm" | "xml" | "svg" | "vue" | "svelte" | "md" | "markdown" => Some(Self::Html),

            "sql" => Some(Self::Sql),

            "json" | "ipynb" => Some(Self::Json),

            _ => None,
        }
    }
}

pub fn count(w: &Workspace) -> HashMap<String, Data> {
    let mut map: HashMap<String, Data> = HashMap::new();

    for f in &w.files {
        let ext = f.path.extension().or_else(|| f.path.file_name())
            .and_then(|e| e.to_str()).unwrap_or("").to_string();

        let mut code_lines: i64 = 0;
        let mut comment_lines: i64 = 0;
        let mut void_lines: i64 = 0;
        let mut in_multiline = false;

        let Some(lang) = Languages::from_ext(&ext) else {
            let entry = map.entry(ext).or_insert(Data {
                code: 0, comment: -1, void: -1,
            });

            entry.code += f.content.lines().count() as i64;
            continue;
        };

        for l in f.content.lines() {
            let trimmed = l.trim();

            if trimmed.is_empty() {
                void_lines += 1;
                continue;
            }

            match lang {
                Languages::Json => {
                    code_lines += 1;
                }
                Languages::Hash => {
                    if trimmed.starts_with('#') {
                        comment_lines += 1;
                    } else {
                        code_lines += 1;
                    }
                }
                Languages::CStyle | Languages::Sql | Languages::Html => {
                    let (multi_start, multi_end, single_prefix) = match lang {
                        Languages::CStyle => ("/*", "*/", "//"),
                        Languages::Sql => ("/*", "*/", "--"),
                        Languages::Html => ("<!--", "-->", ""),
                        _ => unreachable!(),
                    };

                    if in_multiline {
                        comment_lines += 1;
                        if trimmed.contains(multi_end) {
                            in_multiline = false;
                        }
                    } else if !multi_start.is_empty() && trimmed.starts_with(multi_start) {
                        comment_lines += 1;
                        if !trimmed[multi_start.len()..].contains(multi_end) {
                            in_multiline = true;
                        }
                    } else if !single_prefix.is_empty() && trimmed.starts_with(single_prefix) {
                        comment_lines += 1;
                    } else {
                        code_lines += 1;
                    }
                }
            }
        }

        let entry = map.entry(ext).or_default();
        entry.code += code_lines; entry.comment += comment_lines; entry.void += void_lines;
    }
    map
}