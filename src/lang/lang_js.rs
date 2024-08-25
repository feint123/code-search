use tree_sitter::{Language, Node};

use super::{get_defination_string, JavascriptQuery, SymbolQuery};

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
        match node.kind() {
            "function_declaration"
            | "class_declaration"
            | "method_definition"
            | "lexical_declaration" => true,
            _ => false,
        }
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        let node_type = node.kind();
        let definition_list = vec![
            ("function_declaration", "formal_parameters"),
            ("class_declaration", "class_body"),
            ("method_definition", "formal_parameters"),
        ];
        let keywords = vec!["function", "async", "const", "let", "var", "class"];

        return get_defination_string(definition_list, keywords, code, node_type, node);
    }
}
