use colored::Colorize;
use tree_sitter::{Language, Node};

use super::{get_defination_string, GoQuery, SymbolQuery};

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
        match node.kind() {
            "function_declaration"
            | "method_declaration"
            | "method_elem"
            | "type_declaration"
            | "field_declaration" => true,
            _ => false,
        }
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        let node_type = node.kind();
        let definition_list = vec![
            ("function_declaration", "parameter_list"),
            ("method_elem", "parameter_list"),
        ];
        let keywords = vec!["func", "type"];

        let mut output = String::new();

        if node_type == "type_declaration" {
            for child in node.children(&mut node.walk()) {
                if child.kind() == "type" {
                    output.push_str("type".purple().to_string().as_str())
                }
                if child.kind() == "type_spec" {
                    for sub_child in child.children(&mut child.walk()) {
                        if sub_child.kind() == "struct_type" {
                            output.push_str("struct".purple().to_string().as_str())
                        } else if sub_child.kind() == "interface_type" {
                            output.push_str("interface".purple().to_string().as_str())
                        } else {
                            output.push_str(&code[sub_child.byte_range()]);
                        }
                        output.push(' ')
                    }
                }
                output.push(' ')
            }
            return output;
        } else if node.kind() == "method_declaration" {
            for child in node.children(&mut node.walk()) {
                if child.kind() == "func" {
                    output.push_str("func".purple().to_string().as_str())
                } else if child.kind() == "block" {
                    break;
                } else {
                    output.push_str(&code[child.byte_range()]);
                }
                output.push(' ')
            }
            return output;
        } else if node.kind() == "field_declaration" {
            let id_node = node.child_by_field_name("name").unwrap();
            output.push_str(&code[id_node.byte_range()]);
            return output;
        } else {
            return get_defination_string(definition_list, keywords, code, node_type, node);
        }
    }
}
