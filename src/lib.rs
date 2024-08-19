use lang::{
    CQuery, CppQuery, GoQuery, JavaQuery, JavascriptQuery, PythonQuery, RustQuery, SymbolQuery,
};
use regex::Regex;
use std::{
    ffi::OsStr,
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    rc::Rc,
};
use tree_sitter::{Parser, Query, QueryCursor};

pub mod lang;

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
