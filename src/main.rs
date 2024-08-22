use clap::{arg, ArgMatches, Command};
use code_search::{
    build_index, diy_hints, find_text_in_file, get_absolute_path, get_all_symbols,
    get_symbol_query, print_outline, recursion_dir, CodeHinter, CodeIndex,
};
use colored::*;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};

use prettytable::{
    format::{self},
    row, Table,
};
use regex::Regex;
use rustyline::{error::ReadlineError, history::FileHistory, Editor};
use std::{fs, path::Path, rc::Rc};

pub mod lang;

#[derive(Default)]
struct CommandArgs<'a> {
    path: &'a str,
    search_key: &'a str,
    language: &'a str,
    only_symbol: bool,
    use_reg: bool,
    start_interactive_mode: bool,
}
/**
* a code search engine, users can search code clips from different language source files.
* this program use tree-sitter to analyse code and index by tantivy.
*/
fn main() {
    // defined commands
    let matches = Command::new("code-search")
        .about("a command code search engine")
        .version("0.0.3")
        .args(&[
            arg!(-p --path <Path> "搜索路径，文件或目录").default_value("."),
            arg!(-l --language <Language> "使用语言文件扩展名，如 rs、md等"),
            arg!(-s --symbol "只搜索符号，如类名、函数名称等"),
            arg!(-k --key <Key> "关键字").requires_if("", "interactive"),
            arg!(-r --reg "启用正则表达式（会减缓搜索速度）"),
            arg!(-i --interactive "启用交互模式（该模式会构建索引，请指定具体的项目目录）"),
        ])
        .get_matches();

    let args = get_args(&matches);

    if args.start_interactive_mode {
        do_interactive_mode(&args);
        return;
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

/**
* 解析命令参数
*/
fn get_args<'a>(matches: &'a ArgMatches) -> CommandArgs<'a> {
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

    if let Some(interactive_mode) = matches.get_one::<bool>("interactive") {
        args.start_interactive_mode = *interactive_mode;
    } else {
        args.start_interactive_mode = false;
    }
    return args;
}
/**
* 处理交互模式
*/
fn do_interactive_mode(args: &CommandArgs) {
    let path = Path::new(args.path);
    // 构建索引
    let index = build_index(&path);
    // 开始读取指令
    let h = CodeHinter { hints: diy_hints() };
    let mut rl: Editor<CodeHinter, FileHistory> =
        Editor::new().expect("Error enter interactive mode");
    rl.set_helper(Some(h));
    println!("当前根路径为 : {}", get_absolute_path(path).green());
    loop {
        let readline = rl.readline(">> ".green().to_string().as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                if "quit()" == line.as_str() {
                    println!("退出交互模式");
                    break;
                } else if line.starts_with("outline") {
                    let outline_args: Vec<&str> = line.split(" ").collect();
                    if outline_args.len() == 2 {
                        let outline_path = Path::new(outline_args[1]);
                        if outline_path.exists() && outline_path.extension().is_some() {
                            let path_extension =
                                outline_path.extension().unwrap().to_str().unwrap();
                            let code = fs::read_to_string(outline_path).unwrap();
                            print_outline(&code, get_symbol_query(path_extension));
                        } else {
                            println!("{}", "文件路径不存在".red());
                        }
                    } else {
                        println!("{}", "参数非法".red());
                    }
                } else if line.trim_end() == "help" {
                    // 打印帮助信息
                } else if line.trim().len() == 0 {
                    println!("{}", "关键词不能为空".red());
                } else {
                    // 查询信息
                    let result: Vec<&CodeIndex> = index
                        .iter()
                        .filter(|item| item.line_code.contains(line.as_str()))
                        .collect();
                    for item in result {
                        let replace_str = line.clone();
                        let new_line = item
                            .line_code
                            .replace(&replace_str, replace_str.blue().bold().to_string().as_str());
                        println!("{}({}:{})", new_line, item.path, item.line);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
