use tree_sitter::{Language, Node};

use super::{get_defination_string, PythonQuery, SymbolQuery};

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
