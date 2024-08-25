use tree_sitter::{Language, Node};

use super::{get_defination_string, CSharpQuery, SymbolQuery};

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
        match node.kind() {
            "struct_declaration"
            | "class_declaration"
            | "method_declaration"
            | "namespace_declaration" => true,
            _ => false,
        }
    }

    fn get_definition(&self, code: &String, node: &Node) -> String {
        let node_type = node.kind();
        let definition_list = vec![
            ("namespace_declaration", "declaration_list"),
            ("struct_declaration", "declaration_list"),
            ("class_declaration", "declaration_list"),
            ("method_declaration", "parameter_list"),
        ];
        let keywords = vec![
            "struct",
            "class",
            "public",
            "private",
            "protected",
            "internal",
            "static",
            "void",
        ];

        return get_defination_string(definition_list, keywords, code, node_type, node);
    }
}
