use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;
use syn_rsx::{parse, Node, NodeType};

fn walk_nodes(nodes: Vec<Node>) -> (String, Vec<Expr>) {
    let mut out = String::new();
    let mut values = vec![];

    for node in nodes {
        match node.node_type {
            NodeType::Element => {
                let name = node.name_as_string().expect("unexpected missing node name");
                out.push_str(&format!("<{}", name));

                // attributes
                let (html_string, attribute_values) = walk_nodes(node.attributes);
                out.push_str(&html_string);
                values.extend(attribute_values);
                out.push('>');

                // https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
                match name.as_str() {
                    "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                    | "meta" | "param" | "source" | "track" | "wbr" => continue,
                    _ => (),
                }

                // children
                let (html_string, children_values) = walk_nodes(node.children);
                out.push_str(&html_string);
                values.extend(children_values);

                out.push_str(&format!("</{}>", name));
            }
            NodeType::Attribute => {
                out.push_str(&format!(
                    " {}",
                    node.name_as_string().expect("unexpected missing node name")
                ));
                if node.value.is_some() {
                    out.push_str(r#"="{}""#);
                    values.push(node.value.expect("unexpected missing node value"));
                }
            }
            NodeType::Text | NodeType::Block => {
                out.push_str("{}");
                values.push(node.value.expect("unexpected missing node value"));
            }
            NodeType::Fragment => {
                let (html_string, children_values) = walk_nodes(node.children);
                out.push_str(&html_string);
                values.extend(children_values);
            }
            NodeType::Comment => {
                out.push_str("<!-- {} -->");
                values.push(node.value.expect("unexpected missing node value"));
            }
            NodeType::Doctype => {
                let value = node
                    .value_as_string()
                    .expect("unexpected missing node value");
                out.push_str(&format!("<!DOCTYPE {}>", value));
            }
        }
    }

    (out, values)
}

/// Converts HTML to `String`
///
/// Embedded Rust code in braced blocks must return something that implements
/// [`std::fmt::Display`]. More information about supported syntax can be found [here].
///
/// [here]: https://docs.rs/syn-rsx/latest/syn_rsx/#features
#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    match parse(tokens) {
        Ok(nodes) => {
            let (html_string, values) = walk_nodes(nodes);
            quote! { format!(#html_string, #(#values),*) }
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}
