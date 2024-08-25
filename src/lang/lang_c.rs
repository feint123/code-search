use tree_sitter::{Language, Node};

use super::{get_defination_string, CQuery, SymbolQuery};

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
        match node.kind() {
            "function_definition" => true,
            _ => false,
        }
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        let node_type = node.kind();
        let definition_list = vec![("function_definition", "compound_statement")];
        let keywords = vec![
            "struct", "int", "char", "void", "float", "double", "long", "unsigned", "signed",
        ];

        return get_defination_string(definition_list, keywords, code, node_type, node);
    }
}
