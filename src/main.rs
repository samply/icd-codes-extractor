mod catalogue;
mod icdo3_extractor;
mod icd10_extractor;

use crate::{icd10_extractor::extract_icd10_codes, icdo3_extractor::extract_icdo3_codes};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    extract_icdo3_codes()?;
    extract_icd10_codes()?;
    Ok(())
}
