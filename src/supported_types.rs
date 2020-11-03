//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::result::Result;

pub type SupportedTypes =
    std::collections::HashMap<std::string::String, std::string::String>;

//------------------------------------------------------------------------------
pub fn register(supported_types: &mut SupportedTypes) -> Result<()> {
    for i in ["int", "float", "double", "char"].iter() {
        supported_types.insert(i.to_string(), i.to_string());
    }
    
    Ok(())
}
