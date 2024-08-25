use tree_sitter::{Language, Node};

use super::{get_defination_string, RustQuery, SymbolQuery};

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
