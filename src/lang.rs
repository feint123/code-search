use std::collections::HashMap;

use colored::Colorize;
use tree_sitter::{Language, Node};

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

impl SymbolQuery for CSharpQuery {
    fn get_queries(&self) -> Vec<String> {
        return vec![
            String::from(
                r#"
            ((struct_declaration
             name:(identifier) @struct)
             (#match? @struct ":?"))
            "#,
            ),
            String::from(
                r#"
            ((class_declaration
             name:(identifier) @class)
             (#match? @class ":?"))
            "#,
            ),
            String::from(
                r#"
            ((method_declaration
             name:(identifier) @method)
             (#match? @method ":?"))
            "#,
            ),
        ];
    }

    fn get_lang(&self) -> Language {
        tree_sitter_c_sharp::language()
    }

    fn is_key_node(&self, node: &Node) -> bool {
        todo!()
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        todo!()
    }
}

impl SymbolQuery for GoQuery {
    fn get_queries(&self) -> Vec<String> {
        return vec![
            String::from(
                r#"
            ((function_declaration
             name:(identifier) @function)
             (#match? @function ":?"))
            "#,
            ),
            String::from(
                r#"
            ((type_spec
             name:(type_identifier) @type)
             (#match? @type ":?"))
            "#,
            ),
        ];
    }

    fn get_lang(&self) -> Language {
        tree_sitter_go::language()
    }

    fn is_key_node(&self, node: &Node) -> bool {
        todo!()
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        todo!()
    }
}

impl SymbolQuery for JavascriptQuery {
    fn get_queries(&self) -> Vec<String> {
        return vec![String::from(
            r#"
            ((function_declaration
             name:(identifier) @function)
             (#match? @function ":?"))
            "#,
        )];
    }

    fn get_lang(&self) -> Language {
        tree_sitter_javascript::language()
    }

    fn is_key_node(&self, node: &Node) -> bool {
        todo!()
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        todo!()
    }
}

impl SymbolQuery for CppQuery {
    fn get_queries(&self) -> Vec<String> {
        return vec![
            String::from(
                r#"
            ((function_definition
                declarator:(
                    function_declarator
                        declarator:(identifier) @function
                )
            )
            (#match? @function ":?"))
            "#,
            ),
            String::from(
                r#"
                ((struct_specifier
                    name:(type_identifier) @struct)
                    (#match? @struct ":?"))
                "#,
            ),
            String::from(
                r#"
                ((class_specifier
                    name:(type_identifier) @class)
                    (#match? @class ":?"))
                "#,
            ),
        ];
    }

    fn get_lang(&self) -> Language {
        tree_sitter_cpp::language()
    }

    fn is_key_node(&self, node: &Node) -> bool {
        todo!()
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        todo!()
    }
}

impl SymbolQuery for CQuery {
    fn get_queries(&self) -> Vec<String> {
        return vec![
            String::from(
                r#"
            ((function_definition
                declarator:(
                    function_declarator
                        declarator:(identifier) @function
                )
            )
            (#match? @function ":?"))
            "#,
            ),
            String::from(
                r#"
                ((struct_specifier
                    name:(type_identifier) @struct)
                    (#match? @struct ":?"))
                "#,
            ),
        ];
    }

    fn get_lang(&self) -> Language {
        tree_sitter_c::language()
    }

    fn is_key_node(&self, node: &Node) -> bool {
        todo!()
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        todo!()
    }
}

impl SymbolQuery for PythonQuery {
    fn get_queries(&self) -> Vec<String> {
        return vec![
            String::from(
                r#"
            ((function_definition
             name:(identifier) @function)
             (#match? @function ":?"))
            "#,
            ),
            String::from(
                r#"
                ((class_definition
                    name:(identifier) @class)
                    (#match? @class ":?"))
                "#,
            ),
        ];
    }

    fn get_lang(&self) -> Language {
        tree_sitter_python::language()
    }

    fn is_key_node(&self, node: &Node) -> bool {
        match node.kind() {
            "class_definition" | "function_definition" => true,
            _ => false,
        }
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        let node_type = node.kind();
        let definition_list = vec![
            ("class_definition", "block"),
            ("function_definition", "parameters"),
        ];
        let keywords = vec!["class", "def"];

        return get_defination_string(definition_list, keywords, code, node_type, node);
    }
}

impl SymbolQuery for RustQuery {
    fn get_queries(&self) -> Vec<String> {
        return vec![
            String::from(
                r#"
            ((function_item
             name:(identifier) @function)
             (#match? @function ":?"))
            "#,
            ),
            String::from(
                r#"
                ((struct_item
                    name:(type_identifier) @struct)
                    (#match? @struct ":?"))
                "#,
            ),
        ];
    }

    fn get_lang(&self) -> Language {
        tree_sitter_rust::language()
    }

    fn is_key_node(&self, node: &Node) -> bool {
        match node.kind() {
            "function_item"
            | "struct_item"
            | "impl_item"
            | "trait_item"
            | "function_signature_item" => true,
            _ => false,
        }
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        let node_type = node.kind();
        let definition_list = vec![
            ("function_item", "parameters"),
            ("impl_item", "declaration_list"),
            ("struct_item", "field_declaration_list"),
            ("trait_item", "declaration_list"),
            ("function_signature_item", "parameters"),
        ];
        let keywords = vec!["fn", "for", "impl", "where", "struct", "pub", "trait"];

        return get_defination_string(definition_list, keywords, code, node_type, node);
    }
}

impl SymbolQuery for JavaQuery {
    fn get_queries(&self) -> Vec<String> {
        return vec![
            String::from(
                r#"
            ((method_declaration
             name:(identifier) @method)
             (#match? @method ":?"))
            "#,
            ),
            String::from(
                r#"
                ((class_declaration
                    name:(identifier) @class)
                    (#match? @class ":?"))
                "#,
            ),
            String::from(
                r#"
                ((interface_declaration
                    name:(identifier) @interface)
                    (#match? @interface ":?"))
                "#,
            ),
        ];
    }

    fn get_lang(&self) -> Language {
        tree_sitter_java::language()
    }

    fn is_key_node(&self, node: &Node) -> bool {
        match node.kind() {
            "class_declaration" | "method_declaration" | "interface_declaration" => true,
            _ => false,
        }
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        let node_type = node.kind();
        // 0: 类型的根节点， 1: 结束节点
        let definition_list = vec![
            ("class_declaration", "class_body"),
            ("method_declaration", "formal_parameters"),
            ("interface_declaration", "interface_body"),
        ];
        // 语言关键字，需要高亮展示
        let keywords = vec![
            "static",
            "class",
            "extends",
            "public",
            "private",
            "protected",
            "interface",
        ];

        return get_defination_string(definition_list, keywords, code, node_type, node);
    }
}

/**
* 获取类型定义的字符串
*/
fn get_defination_string(
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
