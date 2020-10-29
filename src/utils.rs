//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::result::Result;

//------------------------------------------------------------------------------
pub fn sanitize(name: &std::string::String) -> std::string::String {
    name.replace("<", "_").replace(">", "_")
}

//------------------------------------------------------------------------------
type TemplateParameters =
    std::collections::HashMap<std::string::String, std::string::String>;
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

//------------------------------------------------------------------------------
pub fn decompose_type<'a, 'b>(
    result: &'b mut std::vec::Vec<std::string::String>,
    type_: &'a clang::Type<'a>,
) {
    if let Some(type_) = type_.get_pointee_type() {
        result.push("*".to_string());
        decompose_type(result, &type_)
    } else if type_.is_const_qualified() {
        result.push("const".to_string());
        result.push(type_.get_display_name()[6..].to_string());
    } else {
        result.push(type_.get_display_name())
    }
}

//------------------------------------------------------------------------------
pub fn to_visit_result(result: Result<()>) -> clang::EntityVisitResult {
    match result {
        Ok(_) => clang::EntityVisitResult::Recurse,
        Err(error) => {
            println!("{}", error);
            clang::EntityVisitResult::Break
        }
    }
}
