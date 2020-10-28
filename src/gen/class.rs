//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::constructor;
use super::method;
use crate::ffi_expose;
use crate::result::Result;
use crate::state::State;
use serde_json::json;
use crate::utils::to_visit_result;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
typedef struct
{
    char m_impl[{{size}}];
} __attribute__((aligned({{align}}))) {{name}};
";

//------------------------------------------------------------------------------
pub fn handle(state: &mut State, entity: clang::Entity) -> Result<()> {

    if let Some(_) = ffi_expose::get_arguments(state, entity)? {
        let name = entity.get_display_name().unwrap();

        let size = entity.get_type().unwrap().get_sizeof().unwrap();
        let align = entity.get_type().unwrap().get_alignof().unwrap();

        //println!("name {} {}", name, entity.has_attributes());

        // If this is a template, replace the < and > with _
        let name = name.replace("<","_").replace(">", "_");

        // Generate the code for the class
        state.write_header(
            HEADER_TEMPLATE,
            &json!({"name" : name,
                    "size" : size,
                    "align" : align}),
        );

        // Add the class to the list of supported types subsequent methods
        // will be able to refer to it
        state
            .supported_types
            .insert(name.to_string(), name.to_string());

        // Generate the methods of the class
        entity.visit_children(|child, _| {
            match child.get_kind() {
                // Constructor
                clang::EntityKind::Constructor => {
                    to_visit_result(constructor::handle(state, child, entity));
                }

                // Methods
                clang::EntityKind::Method => {
                    to_visit_result(method::handle(state, child, entity));
                }

                // Ignore everything else
                _ => (),
            };
            clang::EntityVisitResult::Continue
        });
    }

    Ok(())
}
