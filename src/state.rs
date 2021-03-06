//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::result::Result;
use super::supported_types::SupportedTypes;

pub struct State<'a> {
    pub renderer: handlebars::Handlebars<'a>,
    pub buffer: std::string::String,
    pub supported_types: SupportedTypes,
    pub out_source: std::fs::File,
    pub out_header: std::fs::File,
}

//------------------------------------------------------------------------------
impl<'a> State<'a> {
    pub fn new(out_header: &str, out_source: &str) -> Result<Self> {
        Ok(Self {
            renderer: handlebars::Handlebars::new(),
            buffer: std::string::String::new(),
            supported_types: SupportedTypes::new(),
            out_source: std::fs::File::create(out_source).unwrap(),
            out_header: std::fs::File::create(out_header).unwrap(),
        })
    }

    pub fn write_header<T>(&self, template_string: &str, data: &T)
    where
        T: serde::ser::Serialize,
    {
        self.renderer
            .render_template_to_write(template_string, data, &self.out_header)
            .unwrap();
    }

    pub fn write_source<T>(&self, template_string: &str, data: &T)
    where
        T: serde::ser::Serialize,
    {
        self.renderer
            .render_template_to_write(template_string, data, &self.out_source)
            .unwrap();
    }
}
