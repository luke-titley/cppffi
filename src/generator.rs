//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::gen;
use super::result::Result;
use super::state;
use super::supported_types;

pub fn run(
    in_headers: &[&str],
    out_header: &str,
    out_source: &str,
) -> Result<()> {
    // The state we'll pass around during codegen
    let mut state = state::State::new(out_header, out_source)?;
    supported_types::register(&mut state.supported_types);

    // Start parsing header files
    let clang = clang::Clang::new()?;
    let index = clang::Index::new(&clang, true, true);

    // Parse each header file we have been given
    for header in in_headers.iter() {
        let parser = index.parser(header);
        let translation_unit = parser.parse().unwrap();

        // Loop over the header file exporting everything that is of interest
        translation_unit
            .get_entity()
            .visit_children(|entity, parent| {
                match entity.get_kind() {
                    // We want to output classes
                    clang::EntityKind::ClassDecl => {
                        gen::class::handle(&mut state, entity);
                    }

                    // Ignore everything else
                    _ => (),
                };
                clang::EntityVisitResult::Continue
            });
    }

    Ok(())
}