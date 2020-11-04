//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::class_info;
use super::state::State;
use super::utils;

//------------------------------------------------------------------------------
pub fn expand_template_parameters<'a>(
    info: &class_info::ClassInfo,
    type_: &'a clang::Type<'a>,
) -> std::string::String {
    // Convert the type to vec of strings
    let mut result = std::vec::Vec::new();
    utils::decompose_type(&mut result, type_);

    // Remap any template parameters
    info.remap_template_parameters(&result[..]).join(" ")
}

//------------------------------------------------------------------------------
pub fn convert_to_c_type<'a>(
    info: &class_info::ClassInfo,
    state: &mut State,
    type_: &'a clang::Type<'a>,
) -> Option<std::string::String> {
    // Convert the type to vec of strings
    let mut result = std::vec::Vec::new();
    utils::decompose_type(&mut result, type_);

    // Remap any template parameters
    let mut remapped_result = std::vec::Vec::new();
    for r in info.remap_template_parameters(&result[..]).iter() {
        match r.as_str() {
            "&" => remapped_result.push("*".to_string()),
            "*" | "const" => remapped_result.push(r.clone()),
            r => {
                if let Some(r_) = state.supported_types.get(r) {
                    remapped_result.push(r_.clone());
                } else {
                    return None;
                }
            }
        }
    }

    Some(remapped_result.join(" "))
}

//------------------------------------------------------------------------------
pub fn build_arguments(
    info: &class_info::ClassInfo,
    state: &mut State,
    class_name: &std::string::String,
    method_name: &std::string::String,
    arguments: std::vec::Vec<clang::Entity>,
) -> (
    std::string::String,
    std::string::String,
    std::string::String,
    &'static str,
) {
    // Param types
    let types = arguments
        .iter()
        .enumerate()
        .map(|(index, arg)| {
            let type_ =
                expand_template_parameters(info, &arg.get_type().unwrap());
            format!("\n    using A{} = {};", index, type_)
        })
        .collect::<std::vec::Vec<std::string::String>>()
        .join("");

    // Params
    let params = arguments
        .iter()
        .map(|arg| {
            let type_ =
                convert_to_c_type(info, state, &arg.get_type().unwrap())
                    .expect(&format!(
                        "Exposing a method that has parameter types that
              are not tagged to be exposed {}{}, see {}",
                        class_name,
                        method_name,
                        arg.get_type().unwrap().get_display_name(),
                    ));
            let name = arg.get_name().unwrap();

            format!("{} {}", type_, name)
        })
        .collect::<std::vec::Vec<std::string::String>>()
        .join(", ");

    // Arguments
    let args = arguments
        .iter()
        .enumerate()
        .map(|(index, arg)| {
            let name = arg.get_name().unwrap();

            let type_ = arg.get_type().unwrap();
            let kind = type_.get_kind();
            match kind {
                clang::TypeKind::Int
                | clang::TypeKind::Float
                | clang::TypeKind::Bool
                | clang::TypeKind::Double => {
                    format!("\n                {name}", name = name)
                }
                _ => format!(
                    "\n                ffi_cast<A{index}>({name})",
                    index = index,
                    name = name
                ),
            }
        })
        .collect::<std::vec::Vec<std::string::String>>()
        .join(",");

    let comma = if params.is_empty() { "" } else { ", " };

    (types, params, args, comma)
}
