use crate::ResolverError;
use anyhow::Result;

pub fn parse_json_pointer_ref(ref_str: &str) -> Result<Vec<String>, ResolverError> {
    if ref_str.starts_with('#') {
        return Err(ResolverError::InvalidRef(ref_str.to_string()));
    }
    println!("{ref_str}");
    Ok(Vec::new())
}
