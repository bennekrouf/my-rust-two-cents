use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Property {
    Simple(&'static str, String),
    Style(&'static str, String),
    Transform(String),
}
#[derive(Debug)]
pub struct SvgTag {
    pub kind: &'static str,
    pub properties: Vec<Property>,
    pub children: Vec<SvgTag>,
}

impl SvgTag{
    pub fn new(kind: &'static str) -> Self {
        SvgTag {
            kind,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, c:SvgTag) -> Self {
        self.children.push(c);
        self
    }

    pub fn property<V: Display>(mut self, k:&'static str, v: V) -> Self {
        self.properties.push(Property::Simple(k, v.to_string()));
        self
    }

    pub fn style<V: Display>(mut self, k:&'static str, v: V) -> Self {
        self.properties.push(Property::Style(k, v.to_string()));
        self
    }

    pub fn x<V: Display>(self, v: V) -> Self {
        self.property("x", v)
    }
    pub fn y<V: Display>(self, v: V) -> Self {
        self.property("y", v)
    }
    pub fn w<V: Display>(self, v: V) -> Self {
        self.property("width", v)
    }

    pub fn h<V: Display>(self, v: V) -> Self {
        self.property("height", v)
    }
}

pub fn main() {
    println!("TOTO");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let _a = SvgTag::new("svg").w("60px").h("60px").child(
            SvgTag::new("rect").x(6).y(9).w("45").h("32")
        );
        assert_eq!(2+2, 4);
    }
}