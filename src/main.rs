//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
type Result<R> = std::result::Result<R, std::string::String>;

//------------------------------------------------------------------------------
fn handle_class(entity: clang::Entity) {
    let size = entity.get_type().unwrap().get_sizeof().unwrap();
    let align = entity.get_type().unwrap().get_alignof().unwrap();
    println!("struct {} {{
    char m_impl[{}];
}} __attribute__ ((aligned ({})));", entity.get_display_name().unwrap(), size, align)
}

//------------------------------------------------------------------------------
fn doit() -> Result<()> {
    let state = clang::Clang::new()?;
    let index = clang::Index::new(&state, true, true);

    let parser = index.parser("example_class.hpp");

    let translation_unit = parser.parse().unwrap();

    // Loop over the header file exporting everything that is of interest
    translation_unit.get_entity().visit_children(|entity, parent| {
        match entity.get_kind() {
           // We want to output classes
           clang::EntityKind::ClassDecl => handle_class(entity),

           // Ignore everything else
           _ => (),
        }
        clang::EntityVisitResult::Continue
    });

    Ok(())
}

//------------------------------------------------------------------------------
fn main() {
    doit();
}
