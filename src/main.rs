use clap::{arg, Command};
use code_search::{find_text_in_file, get_all_symbols, get_symbol_query, recursion_dir};
use colored::*;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};

use prettytable::{
    format::{self},
    row, Table,
};
use regex::Regex;
use std::{
    fs::{self},
    path::Path,
    rc::Rc,
};

pub mod lang;

#[derive(Default)]
struct CommandArgs<'a> {
    path: &'a str,
    search_key: &'a str,
    language: &'a str,
    only_symbol: bool,
    use_reg: bool,
}
/**
* a code search engine, users can search code clips from different language source files.
* this program use tree-sitter to analyse code and index by tantivy.
*/
fn main() {
    // defined commands
    let matches = Command::new("code-search")
        .about("a command code search engine")
        .args(&[
            arg!(-p --path <Path> "搜索路径，文件或目录").default_value("."),
            arg!(-l --language <Language> "使用语言文件扩展名，如 rs、md等"),
            arg!(-s --symbol "只搜索符号，如类名、函数名称等"),
            arg!(-k --key <Key> "关键字").required(true),
            arg!(-r --reg "启用正则表达式（会减缓搜索速度）"),
        ])
        .get_matches();

    let mut args = CommandArgs::default();
    if let Some(path) = matches.get_one::<String>("path") {
        args.path = path;
    }

    if let Some(lang) = matches.get_one::<String>("language") {
        args.language = lang;
    }

    if let Some(key) = matches.get_one::<String>("key") {
        args.search_key = key;
    }

    if let Some(is_function) = matches.get_one::<bool>("symbol") {
        args.only_symbol = *is_function;
    } else {
        args.only_symbol = false;
    }

    if let Some(use_reg) = matches.get_one::<bool>("reg") {
        args.use_reg = *use_reg;
    } else {
        args.use_reg = false;
    }
    // 扫描目录
    let path = Path::new(args.path);
    let mut pathes = vec![];
    recursion_dir(path, &mut pathes, args.language);
    let mut table = Table::new();
    let format = format::FormatBuilder::new().padding(1, 1).build();
    table.set_format(format);
    table.set_format(format);
    // table.add_row(row!["文件路径".bold(), "代码".bold()]);
    let files = pathes.len();
    let pb = ProgressBar::new(files as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} \n{spinner:.blue} {msg}",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut progress = 1;
    let mut reg: Option<Rc<Regex>> = None;
    if args.use_reg {
        let regex = Regex::new(args.search_key).unwrap();
        reg = Some(Rc::new(regex));
    }
    for path in pathes {
        // let path = doc.get_first(full_path_field).unwrap().as_str().unwrap();
        let path_str = path.to_str().unwrap();
        let mut result = vec![];
        let path_string = path_str.to_string();
        pb.set_position(progress);
        pb.set_message(path_string);
        if path.extension().is_some() {
            let path_extension = path.extension().unwrap().to_str().unwrap();
            // println!("search: {}", path_str.bright_black());
            if args.only_symbol {
                let code = fs::read_to_string(Path::new(path_str)).unwrap_or("".to_string());
                if (args.use_reg && reg.clone().unwrap().captures(code.as_str()).is_some())
                    || code.contains(args.search_key)
                {
                    result =
                        get_all_symbols(&code, args.search_key, get_symbol_query(path_extension));
                }
            } else {
                result = find_text_in_file(path_str, args.search_key, reg.clone()).expect(
                    format!("Error read file {path_str}")
                        .as_str()
                        .red()
                        .to_string()
                        .as_str(),
                );
            }
        }
        progress += 1;
        for (line_number, line) in result {
            let mut replace_str = args.search_key;
            if args.use_reg {
                let re = reg.clone().unwrap();
                for cap in re.captures_iter(line.as_str()) {
                    replace_str = cap.get(0).unwrap().as_str();
                }
            }
            let new_line =
                line.replace(replace_str, replace_str.blue().bold().to_string().as_str());
            table.add_row(row![
                format!(
                    "{}{}{}",
                    path_str.green(),
                    ":".green(),
                    line_number.to_string().green()
                ),
                // line_number.to_string().normal().bold(),
                new_line.trim()
            ]);
        }
    }
    pb.with_finish(ProgressFinish::AndClear);
    println!("");
    // 输出结果
    table.printstd();
}
