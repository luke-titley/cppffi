//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::result::Result;

//------------------------------------------------------------------------------
pub fn sanitize(name: &std::string::String) -> std::string::String {
    name.replace("<", "_").replace(">", "_").replace(":", "_")
}

//------------------------------------------------------------------------------
pub fn decompose_type<'a, 'b>(
    result: &'b mut std::vec::Vec<std::string::String>,
    type_: &'a clang::Type<'a>,
) {
    *result = type_
        .get_display_name()
        .split(" ")
        .map(|i| i.to_string())
        .collect();
    /*
    println!("Decomposing {}", type_.get_display_name());
    if let Some(type_) = type_.get_pointee_type() {
        decompose_type(result, &type_);
        result.push("*".to_string());
    } else if type_.is_const_qualified() {
        result.push("const".to_string());
        result.push(type_.get_display_name()[6..].to_string());
    } else {
        result.push(type_.get_display_name())
    }
    println!("/Decomposing {:?}", &result);
    */
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
