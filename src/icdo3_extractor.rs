use csv::Writer;
use pdf_extract::extract_text;
use regex::Regex;

pub fn extract_icdo3_codes() -> Result<(), Box<dyn std::error::Error>> {
    println!("Extracting ICD-O-3 values from PDF...\n");

    let pdf_path = "resources/sitetype.icdo3.20220429.pdf";
    let csv_path = "resources/icdo3_full.csv";

    let text = extract_text(pdf_path)?;
    let mut wtr = Writer::from_path(csv_path)?;

    wtr.write_record(&[
        "Site Group",
        "Topography Codes",
        "Label",
        "Topography",
        "Morphology",
        "Description",
    ])?;

    let site_re = Regex::new(r"^([A-Z &.,'-]+)\s+(C\d{3}(?:[-,]C\d{3})*)$")?;
    let labeled_entry_re = Regex::new(r"^([\w\s.,&'/-]+)\s+(\d{3})\s+(\d{4}/\d)\s+(.+)$")?;
    let simple_entry_re = Regex::new(r"^(\d{4}/\d)\s+(.+)$")?;

    let mut current_site = String::new();
    let mut current_topos = String::new();
    let mut current_label = String::new();
    let mut current_topo_code = String::new();

    for (i, line) in text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        if let Some(site_caps) = site_re.captures(line) {
            current_site = site_caps[1].trim().to_string();
            current_topos = site_caps[2].trim().to_string();
        } else if let Some(entry_caps) = labeled_entry_re.captures(line) {
            current_label = entry_caps[1].trim().to_string();
            current_topo_code = entry_caps[2].trim().to_string();
            let morph = entry_caps[3].trim();
            let desc = entry_caps[4].trim();
            wtr.write_record(&[
                &current_site,
                &current_topos,
                &current_label,
                &current_topo_code,
                morph,
                desc,
            ])?;
        } else if let Some(simple_caps) = simple_entry_re.captures(line) {
            let morph = simple_caps[1].trim();
            let desc = simple_caps[2].trim();
            wtr.write_record(&[
                &current_site,
                &current_topos,
                &current_label,
                &current_topo_code,
                morph,
                desc,
            ])?;
        } else {
            // Log skipped lines with line number
            // Logs any line that doesn't match:
            // - Site group header
            // - Labeled morphology entry
            // - Simple morphology entry
            // Includes the line number and content for easy debugging
            // Uses eprintln! so logs go to stderr (you can redirect if needed)
            eprintln!("⚠️ Skipped line {}: '{}'", i + 1, line);
        }
    }

    wtr.flush()?;
    println!("CSV written to {}", csv_path);
    Ok(())
}