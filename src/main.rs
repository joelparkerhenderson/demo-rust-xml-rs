extern crate xml;

use std::env;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_indent_args_zero() {
        assert_eq!(indent(0), "");
    }

    #[test]
    fn test_indent_args_some() {
        assert_eq!(indent(3), "      ");
    }

    #[test]
    fn test_indent_text_args_zero() {
        assert_eq!(indents(0, "a"), "a");
    }

    #[test]
    fn test_indent_text_args_some() {
        assert_eq!(indents(3, "a"), "      a");
    }

}

//////////////////////////////////////////////////////////////////////////////
//
// Text utilities
//
//////////////////////////////////////////////////////////////////////////////

fn indent(size: usize) -> String {
    "  ".repeat(size)
}

fn indent_text(size: usize, s: &str) -> String {
    format!("{}{}", indent(size), s)
}

//////////////////////////////////////////////////////////////////////////////
//
// XML processing
//
//////////////////////////////////////////////////////////////////////////////

fn do_file_path(file_path: &str) {
    let file = File::open(file_path).unwrap();
    do_file(file);
}

fn do_file(file: File) {
    let reader = BufReader::new(file);
    do_reader(reader);
}

fn do_reader(reader: BufReader<File>) {
    let parser = EventReader::new(reader);
    do_parser(parser);
}

fn do_parser(parser: EventReader<BufReader<File>>) {
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(e) => {
                depth = on_xml_event(e, depth);
            }
            Err(e) => {
                on_xml_error(e);
                break;
            }
        }
    }
}

fn on_xml_event(e: XmlEvent, depth: usize) -> usize {
    let msg: String;
    let mut depth = depth;
    #[allow(unused_variables)] 
    match e {
        XmlEvent::StartDocument {
            version,
            encoding,
            standalone
        } => {
            msg = indent_text(
                depth,
                &format!("StartDocument version:{} encoding:{} standalone:{:?}", version, encoding, standalone) 
            );
            depth += 1;
        }
        XmlEvent::EndDocument {} => {
            msg = indent_text(
                depth - 1, 
                "EndDocument"
            );
            depth -=1;
        }
        XmlEvent::ProcessingInstruction {
            name,
            data,
        } => {
            msg = indent_text(
                depth, 
                &format!("ProcessingInstruction name:{} data length:{}", name, data.expect("data.expect").len())
            );
        }   
        XmlEvent::StartElement {
            name,
            attributes,
            namespace,
        } => {
            msg = indent_text(
                depth, 
                &format!("StartElement name:{} atributes:{:?}", name, attributes)
            );
            depth += 1;
        }
        XmlEvent::EndElement {
            name,
        } => {
            depth -= 1;
            msg = indent_text(
                depth, 
                &format!("EndElement {}", name)
            );
        }
        XmlEvent::CData(s) => {
            msg = indent_text(
                depth, 
                &format!("CData {}", s)
            );
        }
        XmlEvent::Comment(s) => {
            msg = indent_text(
                depth, 
                &format!("Comment {}", s)
            );
        }
        XmlEvent::Characters(s) => {
            msg = indent_text(
                depth, 
                &format!("Characters {}", s)
            );
        }
        XmlEvent::Whitespace(s)  => {
            msg = "".to_string();
        }
    };
    if !msg.is_empty() { println!("{}", msg); }
    depth
}

fn on_xml_error(e: xml::reader::Error) {
    println!("Error: {}", e);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    do_file_path(file_path);
}
