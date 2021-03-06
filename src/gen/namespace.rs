//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use crate::utils::to_visit_result;

use crate::gen;

//------------------------------------------------------------------------------
pub fn handle(state: &mut State, entity: clang::Entity) -> Result<()> {
    // We dont expose annonymous namespaces
    if let Some(ns) = entity.get_display_name() {
        entity.visit_children(|child, _| {
            match child.get_kind() {
                clang::EntityKind::ClassDecl => {
                    to_visit_result(gen::class::handle(&ns, state, child));
                }

                // Ignore everything else
                _ => {}
            }

            clang::EntityVisitResult::Recurse
        });
    }

    Ok(())
}
