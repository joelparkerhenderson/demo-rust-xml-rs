# XML example

XML example that uses Rust and the xml-rs crate to parse a typical XML file.

Example:

```shell
demo-rust-xml-rs file.xml
```

Input file:
```xml
<foo>
    <goo>Hello</goo>
</foo>
```

Output:

```shell
StartDocument version:1.0 encoding:UTF-8 standalone:None
  StartElement name:foo atributes:[]
    StartElement name:goo atributes:[]
      Characters Hello
    EndElement goo
  EndElement foo
EndDocument
```


## Steps

The program does these steps:

1. Open a file path "file.xml".
2. Cast the file to a BufReader to an EventReader.
3. Parse the XML elements


## XmlEvent

```rust
pub enum XmlEvent {
    StartDocument {
        version: XmlVersion,
        encoding: String,
        standalone: Option<bool>,
    },
    EndDocument,
    ProcessingInstruction {
        name: String,
        data: Option<String>,
    },
    StartElement {
        name: OwnedName,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    },
    EndElement {
        name: OwnedName,
    },
    CData(String),
    Comment(String),
    Characters(String),
    Whitespace(String),
}
```