use tree_sitter::{Language, Node};

use super::{get_defination_string, CppQuery, SymbolQuery};

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
        match node.kind() {
            "function_definition"
            | "struct_specifier"
            | "class_specifier"
            | "field_declaration" => true,
            _ => false,
        }
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        let node_type = node.kind();
        let definition_list = vec![
            ("function_definition", "compound_statement"),
            ("struct_specifier", "field_declaration_list"),
            ("class_specifier", "field_declaration_list"),
            ("field_declaration", ";"),
        ];
        let keywords = vec![
            "struct",
            "class",
            "public",
            "private",
            "protected",
            "virtual",
            "static",
            "const",
            "void",
        ];

        return get_defination_string(definition_list, keywords, code, node_type, node);
    }
}
