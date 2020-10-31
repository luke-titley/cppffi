//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use serde_json::json;

use crate::class_info;
use crate::ffi_expose;
use crate::utils;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
{{return}} {{class}}__{{{outer_name}}} ({{class}} * this {{comma}} {{params}});
";

static BODY_TEMPLATE: &'static str = "
{{return}} {{class}}__{{{outer_name}}} ({{class}} * this {{comma}} {{params}})
{
    return reinterpret_cast<{{{class}}}*>(this)->{{{name}}}({{{args}}});
}
";

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
            "*" | "&" | "const" => remapped_result.push(r.clone()),
            r => {
                if let Some(r_) = state.supported_types.get(r) {
                    remapped_result.push(r_.clone());
                } else {
                    println!("{:?}", state.supported_types);
                    println!("We're done");
                    return None;
                }
            }
        }
    }

    Some(remapped_result.join(" "))
}

//------------------------------------------------------------------------------
fn build_arguments(
    info: &class_info::ClassInfo,
    state: &mut State,
    class_name: &std::string::String,
    method_name: &std::string::String,
    arguments: std::vec::Vec<clang::Entity>,
) -> (std::string::String, std::string::String, &'static str) {
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
        .join(",");

    // Arguments
    let args = arguments
        .iter()
        .map(|arg| {
            let type_ =
                expand_template_parameters(info, &arg.get_type().unwrap());
            let name = arg.get_name().unwrap();

            format!("*((({}*))&{})", type_, name)
        })
        .collect::<std::vec::Vec<std::string::String>>()
        .join(",");

    let comma = if params.is_empty() { "" } else { "," };

    (params, args, comma)
}

//------------------------------------------------------------------------------
pub fn handle(
    info: &class_info::ClassInfo,
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    if let Some(ffi_arguments) =
        ffi_expose::get_arguments(state, entity).unwrap()
    {
        if let Some(result_type) =
            convert_to_c_type(info, state, &entity.get_result_type().unwrap())
        {
            let class_name =
                utils::sanitize(&parent.get_display_name().unwrap());

            let method_name = entity.get_name().unwrap();
            let outer_method_name = if ffi_arguments.arguments.is_empty()
                || ffi_arguments.arguments[0].is_empty()
            {
                method_name.clone()
            } else {
                ffi_arguments.arguments[0].clone()
            };

            // Build the parameter list
            if let Some(arguments) = entity.get_arguments() {
                //let args = arguments.iter().map(|arg| {});

                let (params, args, comma) = build_arguments(
                    info,
                    state,
                    &class_name,
                    &method_name,
                    arguments,
                );

                // Header
                state.write_header(
                    HEADER_TEMPLATE,
                    &json!({"return" : result_type,
                            "name" : method_name,
                            "outer_name" : outer_method_name,
                            "class" : class_name,
                            "comma" : comma,
                            "params": params,
                            "args": args,
                    }),
                );

                // Source
                state.write_source(
                    BODY_TEMPLATE,
                    &json!({"return" : result_type,
                            "name" : method_name,
                            "outer_name" : outer_method_name,
                            "class" : class_name,
                            "comma" : comma,
                            "params": params,
                            "args" : args,
                    }),
                );
            }
        }
    }

    Ok(())
}
