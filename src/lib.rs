use colored::Colorize;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use lang::{
    CQuery, CppQuery, GoQuery, JavaQuery, JavascriptQuery, PythonQuery, RustQuery, SymbolQuery,
};
use regex::Regex;
use rustyline::{
    hint::{Hint, Hinter},
    Completer, Context, Helper, Highlighter, Validator,
};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{self, read_dir, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    rc::Rc,
};
use tree_sitter::{Node, Parser, Query, QueryCursor};

pub mod lang;

#[derive(Completer, Helper, Highlighter, Validator)]
pub struct CodeHinter {
    pub hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for CommandHint {
    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.to_owned().complete_up_to])
        } else {
            None
        }
    }

    fn display(&self) -> &str {
        &self.display
    }
}

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> Self {
        assert!(text.starts_with(complete_up_to));
        Self {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> Self {
        Self {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Hinter for CodeHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints
            .iter()
            .filter_map(|hint| {
                // expect hint after word complete, like redis cli, add condition:
                // line.ends_with(" ")
                if hint.display.starts_with(line) {
                    Some(hint.suffix(pos))
                } else {
                    None
                }
            })
            .next()
    }
}

pub fn diy_hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();
    set.insert(CommandHint::new("help", "help"));
    set.insert(CommandHint::new(
        format!("outline {}", "代码文件路径".bright_black()).as_str(),
        "outline ",
    ));
    set.insert(CommandHint::new(
        format!("search {}", "path search_key".bright_black()).as_str(),
        "search ",
    ));
    set.insert(CommandHint::new("quit()", "quit()"));
    set
}

fn valid_language_file(extention: &str) -> bool {
    let valid_extensions = vec![
        "rs",
        "js",
        "ts",
        "java",
        "py",
        "go",
        "c",
        "cpp",
        "md",
        "txt",
        "html",
        "css",
        "cs",
        "kt",
        "swift",
        "php",
        "rb",
        "sh",
        "sql",
        "vb",
        "lua",
        "hs",
        "scala",
        "erl",
        "m",
        "r",
        "h",
        "hpp",
        "toml",
        "yaml",
        "yml",
        "properties",
    ];
    return valid_extensions.contains(&extention);
}
/*
* 递归目录
*/
pub fn recursion_dir(root_path: &Path, pathes: &mut Vec<PathBuf>, filter: &str) {
    if root_path.is_dir() {
        for entry in read_dir(root_path).expect("Error read Dir") {
            let dir_entry = entry.expect("Error");
            let path_buf = dir_entry.path();

            recursion_dir(path_buf.as_path(), pathes, filter);
        }
    } else if root_path.is_file() {
        if root_path.extension().is_some() {
            let extension = root_path
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap();
            if (filter.is_empty() || filter == extension) && valid_language_file(extension) {
                pathes.push(root_path.to_path_buf());
            }
        }
    }
}

pub fn get_symbol_query(extention: &str) -> Box<dyn SymbolQuery> {
    match extention {
        "rs" => Box::new(RustQuery),
        "java" => Box::new(JavaQuery),
        "py" => Box::new(PythonQuery),
        "c" => Box::new(CQuery),
        "cpp" => Box::new(CppQuery),
        "js" => Box::new(JavascriptQuery),
        "go" => Box::new(GoQuery),
        _ => Box::new(RustQuery),
    }
}
/**
* 获取源码中的所有符号
*
*
*/
pub fn get_all_symbols(
    code: &String,
    search_key: &str,
    symbol_query: Box<dyn SymbolQuery>,
) -> Vec<(usize, String)> {
    let mut parser = Parser::new();
    parser
        .set_language(&symbol_query.get_lang())
        .expect("Error load Rust grammer");
    let tree = parser.parse(code.as_str(), None).unwrap();

    let mut query_cursor = QueryCursor::new();
    let mut filed_vec = vec![];
    for sq in symbol_query.get_queries() {
        let query = Query::new(
            &symbol_query.get_lang(),
            sq.replace(":?", search_key).as_str(),
        )
        .unwrap();
        let captures = query_cursor.captures(&query, tree.root_node(), code.as_bytes());
        for (m, capture_index) in captures {
            let capture = m.captures[capture_index];
            let node = capture.node;
            let text = node.utf8_text(code.as_bytes()).unwrap();
            filed_vec.push((node.start_position().row + 1, text.to_string()));
        }
    }
    return filed_vec;
}
/**
* 打印大纲
*/
pub fn print_outline(code: &String, symbol_query: Box<dyn SymbolQuery>) {
    let mut parser = Parser::new();
    parser
        .set_language(&symbol_query.get_lang())
        .expect("Error load Rust grammer");
    let tree = parser.parse(code.as_str(), None).unwrap();
    let root_node = tree.root_node();
    recursion_outline(root_node, code, 0, &symbol_query);
}

pub fn recursion_outline(
    node: Node,
    code: &String,
    indent: usize,
    symbol_query: &Box<dyn SymbolQuery>,
) {
    if symbol_query.is_key_node(&node) {
        print!("{}", " ".repeat(indent));
        let output = symbol_query.get_definition(code, &node);
        println!("{}", output);
    }

    for child in node.children(&mut node.walk()) {
        recursion_outline(child, code, indent + 2, symbol_query)
    }
}

pub fn find_text_in_file(
    filename: &str,
    text: &str,
    reg: Option<Rc<Regex>>,
) -> Result<Vec<(usize, String)>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut found_lines = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap_or("".to_string());
        if reg.is_some() {
            let reg = reg.clone().unwrap();
            if reg.captures(line.as_str()).is_some() {
                found_lines.push((line_number + 1, line));
            }
        } else if line.contains(text) {
            found_lines.push((line_number + 1, line));
        }
    }
    Ok(found_lines)
}

pub fn get_absolute_path(path: &Path) -> String {
    if path.exists() {
        let absolute_path = fs::canonicalize(path).unwrap();
        return absolute_path.to_str().unwrap().to_string();
    } else {
        return String::new();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeIndex {
    pub path: String,
    pub line: usize,
    pub line_code: String,
}

/**
* 构建索引
*/
pub fn build_index(project_path: &Path) -> Vec<CodeIndex> {
    let mut index_list = vec![];
    let mut pathes = vec![];
    // 获取项目中的文件
    recursion_dir(project_path, &mut pathes, "");
    let files = pathes.len();
    let pb = ProgressBar::new(files as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {pos}/{len} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    let mut progress = 1;
    if pathes.len() > 0 {
        for path in pathes {
            let path_extension = path.extension().unwrap().to_str().unwrap();
            let path_str = get_absolute_path(&path);

            let code = fs::read_to_string(Path::new(path_str.as_str())).unwrap_or("".to_string());
            let result = get_all_symbols(&code, ".*", get_symbol_query(path_extension));
            if result.len() > 0 {
                result
                    .iter()
                    .map(|item| CodeIndex {
                        path: path_str.to_string(),
                        line: item.0,
                        line_code: item.1.clone(),
                    })
                    .for_each(|item| {
                        pb.set_message(item.path.clone());
                        pb.set_position(progress);
                        index_list.push(item);
                    });
            }
            progress += 1;
        }
    }
    pb.with_finish(ProgressFinish::AndClear);
    return index_list;
}
