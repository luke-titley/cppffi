//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::gen;
use super::result::Result;
use super::state;
use super::supported_types;
use super::utils::to_visit_result;
use serde_json::json;

//------------------------------------------------------------------------------
static HEADER_BEGIN: &'static str = "
#include \"{{{header}}}\"

#define FFI_SIZE(SIZE) char data[(SIZE)];
#define FFI_ALIGN(ALIGN) __attribute__((aligned((ALIGN))))

extern \"C\" {
";

//------------------------------------------------------------------------------
static HEADER_END: &'static str = "
}
";

//------------------------------------------------------------------------------
static SOURCE_TEMPLATE: &'static str = "
template<typename CPP>
static inline CPP & ffi_cast(void * var)
{
    return *reinterpret_cast<CPP*>(var);
}
";

pub fn run(
    in_headers: &[&str],
    out_header: &str,
    out_source: &str,
    arguments: &[&str],
) -> Result<()> {
    // The state we'll pass around during codegen
    let mut state = state::State::new(out_header, out_source)?;
    supported_types::register(&mut state.supported_types)?;

    // Start parsing header files
    let clang = clang::Clang::new()?;
    let index = clang::Index::new(&clang, true, true);

    state.write_header(HEADER_BEGIN, &json!({
        "header" : out_header
    }));

    state.write_source(SOURCE_TEMPLATE, &json!({}));

    // Parse each header file we have been given
    for header in in_headers.iter() {
        // Parse the compiler arguments
        let mut parser = index.parser(header);
        parser.arguments(arguments);

        // Loop over the header file exporting everything that is of interest
        let translation_unit = parser.parse().unwrap();
        translation_unit.get_entity().visit_children(|entity, _| {
            match entity.get_kind() {
                // Class
                clang::EntityKind::ClassDecl => {
                    to_visit_result(gen::class::handle(&mut state, entity));
                }

                // Using
                /*
                clang::EntityKind::TypedefDecl => {
                    //clang::EntityKind::UsingDeclaration => {
                    to_visit_result(gen::typedef::handle(&mut state, entity));
                }
                */
                // Ignore everything else
                _ => {
                    /*
                    if let Some(name) = entity.get_display_name() {
                        println!("name {} {:?}", name, kind);
                    }
                    */
                }
            };
            clang::EntityVisitResult::Recurse
        });

        state.write_header(HEADER_END, &json!({}));
    }

    Ok(())
}
