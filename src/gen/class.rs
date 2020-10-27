//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::constructor;
use super::method;
use crate::result::Result;
use crate::state::State;
use serde_json::json;
use std::io::Write;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
typedef struct
{
    char m_impl[{{size}}];
} __attribute__((aligned({{align}}))) {{name}};
";

//------------------------------------------------------------------------------
pub fn handle(state: &mut State, entity: clang::Entity) -> Result<()> {
    let size = entity.get_type().unwrap().get_sizeof().unwrap();
    let align = entity.get_type().unwrap().get_alignof().unwrap();
    let name = entity.get_display_name().unwrap();

    // Generate the code for the class
    writeln!(
        &state.out_header,
        "{}",
        state
            .renderer
            .render_template(
                HEADER_TEMPLATE,
                &json!({"name" : name,
                    "size" : size,
                    "align" : align})
            )
            .unwrap()
    );

    // Add the class to the list of supported types subsequent methods
    // will be able to refer to it
    state
        .supported_types
        .insert(name.to_string(), name.to_string());

    // Generate the methods of the class
    entity.visit_children(|child, parent| {
        match child.get_kind() {
            // Constructor
            clang::EntityKind::Constructor => {
                constructor::handle(state, child, entity);
            }

            // Methods
            clang::EntityKind::Method => {
                method::handle(state, child, entity);
            }

            // Ignore everything else
            _ => (),
        };
        clang::EntityVisitResult::Continue
    });

    Ok(())
}
