//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
type TemplateParameters =
    std::collections::HashMap<std::string::String, std::string::String>;

//------------------------------------------------------------------------------
pub struct ClassInfo {
    pub template_parameters: TemplateParameters,
    pub c_name: std::string::String,
    pub cpp_name: std::string::String,
    pub namespace: std::string::String,
}

impl ClassInfo {
    pub fn new(namespace: &str, name: &str, cpp_name: &str) -> Self {
        Self {
            template_parameters: TemplateParameters::new(),
            c_name: name.to_string(),
            cpp_name: cpp_name.to_string(),
            namespace: namespace.to_string(),
        }
    }
}

//------------------------------------------------------------------------------
impl ClassInfo {
    pub fn remap_template_parameters(
        &self,
        types: &[std::string::String],
    ) -> std::vec::Vec<std::string::String> {
        types
            .iter()
            .map(|key| {
                if let Some(value) = self.template_parameters.get(key) {
                    value.clone()
                } else {
                    key.clone()
                }
            })
            .collect()
    }
}

//------------------------------------------------------------------------------
pub fn build_template_parameter_mapping(
    ns: &str,
    template: clang::Entity,
    template_instance: clang::Entity,
) -> TemplateParameters {
    // The regex for getting the template parameters
    let extract_template_parameters =
        regex::Regex::new(r"[a-zA-Z0-9]+<([a-zA-Z0-9]+,?)*>").unwrap();

    // Get the names
    let raw_def_name = template.get_display_name().unwrap();
    let raw_name = template_instance.get_display_name().unwrap();

    // Match the template parameters
    let param_names =
        extract_template_parameters.captures_iter(raw_def_name.as_str());
    let param_values =
        extract_template_parameters.captures_iter(raw_name.as_str());

    // Zip the result
    let mut result = TemplateParameters::new();
    result.insert(
        raw_def_name.to_string(),
        format!("::{}::{}", ns, raw_name.to_string()),
        //raw_name.to_string(),
    );
    for (key, value) in param_names.zip(param_values) {
        result.insert(key[1].to_string(), value[1].to_string());
    }

    result
}
