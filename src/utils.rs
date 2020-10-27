//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
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