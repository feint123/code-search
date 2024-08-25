use colored::Colorize;
use tree_sitter::{Language, Node};
mod lang_c;
mod lang_cpp;
mod lang_csharp;
mod lang_go;
mod lang_java;
mod lang_js;
mod lang_py;
mod lang_rust;

pub trait SymbolQuery {
    fn get_queries(&self) -> Vec<String>;
    fn get_lang(&self) -> Language;
    fn is_key_node(&self, node: &Node) -> bool;
    fn get_definition(&self, code: &String, node: &Node) -> String;
}

pub struct RustQuery;

pub struct JavaQuery;

pub struct PythonQuery;

pub struct JavascriptQuery;

pub struct CQuery;

pub struct CppQuery;

pub struct GoQuery;

pub struct CSharpQuery;

/**
* 获取类型定义的字符串
*/
pub fn get_defination_string(
    definition_list: Vec<(&str, &str)>,
    keywords: Vec<&str>,
    code: &String,
    node_type: &str,
    node: &Node,
) -> String {
    let mut output = String::new();
    for (root_type, end_type) in definition_list {
        if node_type == root_type {
            for child in node.children(&mut node.walk()) {
                if child.kind() == end_type {
                    break;
                } else {
                    let node_text = &code[child.byte_range()];
                    // println!("node_text:{}, node_kind: {}", node_text, child.kind());
                    for node_text_item in node_text.split(" ").enumerate() {
                        if keywords.contains(&node_text_item.1) {
                            output.push_str(node_text_item.1.purple().to_string().as_str());
                        } else {
                            output.push_str(node_text_item.1);
                        }
                        output.push(' ');
                    }
                }
            }
            break;
        }
    }
    return output;
}
