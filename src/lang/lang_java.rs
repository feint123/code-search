use tree_sitter::{Language, Node};

use super::{get_defination_string, JavaQuery, SymbolQuery};

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
