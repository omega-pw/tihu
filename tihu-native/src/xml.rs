use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Read;

#[derive(Debug)]

pub struct Node {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Child>,
}

#[derive(Debug)]
pub enum Child {
    Node(Node),
    Text(String),
}

fn parse_children<R>(
    reader: &mut Reader<R>,
    parent_node: &mut Node,
    is_root: bool,
    buf: &mut Vec<u8>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    R: Read + BufRead,
{
    loop {
        let event = reader.read_event_into(buf)?;
        match event {
            Event::Start(e) => {
                let name = String::from_utf8(e.name().as_ref().to_vec())?;
                let mut attributes = HashMap::new();
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = String::from_utf8(attr.key.as_ref().to_vec())?;
                    let value = String::from_utf8(attr.value.as_ref().to_vec())?;
                    attributes.insert(key, value);
                }
                let mut child = Node {
                    name: name,
                    attributes: attributes,
                    children: Vec::new(),
                };
                parse_children(reader, &mut child, false, buf)?;
                parent_node.children.push(Child::Node(child));
            }
            Event::End(e) => {
                let name = e.name();
                if parent_node.name.as_bytes() == name.as_ref() {
                    return Ok(());
                } else {
                    let name = String::from_utf8(name.as_ref().to_vec())?;
                    return Err(format!(
                        "End tag \"{}\" does not match start tag \"{}\"",
                        name, parent_node.name
                    )
                    .into());
                }
            }
            Event::Empty(e) => {
                let name = String::from_utf8(e.name().as_ref().to_vec())?;
                let mut attributes = HashMap::new();
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = String::from_utf8(attr.key.as_ref().to_vec())?;
                    let value = String::from_utf8(attr.value.as_ref().to_vec())?;
                    attributes.insert(key, value);
                }
                let child = Node {
                    name: name,
                    attributes: attributes,
                    children: Vec::new(),
                };
                parent_node.children.push(Child::Node(child));
            }
            Event::Text(e) => {
                let text = e.unescape()?;
                let text = text.to_string();
                parent_node.children.push(Child::Text(text));
            }
            Event::Eof => {
                if is_root {
                    return Ok(());
                } else {
                    return Err(format!("No end tag found for \"{}\"", parent_node.name).into());
                }
            }
            _ => (),
        }
    }
}

pub fn parse_xml<R>(reader: R) -> Result<Vec<Child>, Box<dyn std::error::Error + Send + Sync>>
where
    R: Read + BufRead,
{
    let mut reader = Reader::from_reader(reader);
    reader.config_mut().trim_text(true);
    let mut root = Node {
        name: String::from("root"),
        attributes: HashMap::new(),
        children: Vec::new(),
    };
    let mut buf = Vec::new();
    parse_children(&mut reader, &mut root, true, &mut buf)?;
    drop(buf);
    return Ok(root.children);
}
