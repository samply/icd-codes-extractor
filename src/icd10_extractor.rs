use crate::catalogue::{Catalogue, Category, Criteria};
use std::{error::Error, fs::File};

pub fn extract_icd10_codes() -> Result<(), Box<dyn Error>> {
    println!("Extracting ICD-10 values from JSON...\n");

    let json_path = "resources/catalogue-with-icd10-codes.json";
    let csv_path = "resources/icdo10_full.csv";

    // Read the JSON file
    let file = File::open(json_path)?;
    let catalogue: Catalogue = serde_json::from_reader(file)?;

    // Create the CSV file
    let mut wtr = csv::Writer::from_path(csv_path)?;

    // Write headers
    wtr.write_record(&[
        "Category Key",
        "Category Name",
        "Group Key",
        "Group Name",
        "Description",
    ])?;

    // Traverse the catalogue
    for category_group in catalogue {
        if let Category::Group(group) = category_group {
            for category in group.child_categories {
                if let Category::Autocomplete(autocomplete_category) = category {
                    if autocomplete_category.key == "diagnosis" {
                        for criterion in autocomplete_category.criteria {
                            // Write the current criteria with the autocomplete category key and name
                            wtr.write_record(&[
                                &autocomplete_category.key,
                                &autocomplete_category.name,
                                &criterion.key,
                                &criterion.name,
                                criterion.description.as_deref().unwrap_or(""),
                            ])?;
                            // Pass the subgroup (Vec<Criteria>) to the recursive function
                            write_criteria_subgroups_to_csv(
                                &mut wtr,
                                &criterion.subgroup.unwrap_or(vec![]),
                                &autocomplete_category.key,
                                &autocomplete_category.name,
                            )?;
                        }
                    }
                }
            }
        }
    }

    wtr.flush()?;
    println!("CSV written to {}", csv_path);
    Ok(())
}

fn write_criteria_subgroups_to_csv(
    wtr: &mut csv::Writer<File>,
    subgroups: &[Criteria],  // Accept a slice of Criteria
    autocomplete_key: &str,  // Pass the autocomplete category key
    autocomplete_name: &str, // Pass the autocomplete category name
) -> Result<(), Box<dyn Error>> {
    // Recursively write subgroups
    for subgroup in subgroups {
        // Write the current subgroup with the autocomplete category key and name
        wtr.write_record(&[
            autocomplete_key,
            autocomplete_name,
            &subgroup.key,
            &subgroup.name,
            subgroup.description.as_deref().unwrap_or(""),
        ])?;
        // Recursively process nested subgroups
        write_criteria_subgroups_to_csv(
            wtr,
            &subgroup.subgroup.as_ref().unwrap_or(&vec![]),
            autocomplete_key,
            autocomplete_name,
        )?;
    }

    Ok(())
}
