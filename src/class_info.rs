//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
type TemplateParameters =
    std::collections::HashMap<std::string::String, std::string::String>;

//------------------------------------------------------------------------------
pub struct ClassInfo {
    pub template_parameters: TemplateParameters,
}

//------------------------------------------------------------------------------
impl Default for ClassInfo {
    fn default() -> Self {
        Self {
            template_parameters: TemplateParameters::new(),
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
    template: clang::Entity,
    template_instance: clang::Entity,
) -> TemplateParameters {
    // The regex for getting the template parameters
    let extract_template_parameters =
        regex::Regex::new(r"[a-zA-Z0-9]+<([a-zA-Z0-9]+,?)*>").unwrap();

    // Get the names
    let raw_def_name = template.get_display_name().unwrap();
    let raw_name = template_instance.get_display_name().unwrap();

    // Match the regex
    let param_names =
        extract_template_parameters.captures_iter(raw_def_name.as_str());
    let param_values =
        extract_template_parameters.captures_iter(raw_name.as_str());

    // Zip the result
    param_names
        .zip(param_values)
        .map(|(key, value)| (key[1].to_string(), value[1].to_string()))
        .collect()
}
