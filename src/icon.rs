use crate::Node;

pub fn get_icon_by_node(node: &Node) -> String {
    if node.directory {
        return "📁".to_owned()
    }
    return "📃".to_owned()
}