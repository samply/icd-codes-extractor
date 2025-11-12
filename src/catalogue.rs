use serde::{Deserialize, Serialize};

/// The catalogue is a tree-like data structure that describes what the user can search for.
pub type Catalogue = Vec<Category>;

/// The @discriminator annotation in TypeScript is represented as a tagged enum in Rust.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "fieldType")]
pub enum Category {
    #[serde(rename = "group")]
    Group(CategoryGroup),

    #[serde(rename = "single-select")]
    SingleSelect(SingleSelectCategory),

    #[serde(rename = "autocomplete")]
    Autocomplete(AutocompleteCategory),

    #[serde(rename = "number")]
    NumericRange(NumericRangeCategory),

    #[serde(rename = "date")]
    DateRange(DateRangeCategory),

    #[serde(rename = "string")]
    String(StringCategory),
}

/// A logical grouping of catalogue items that is rendered as a collapsable entry in the catalogue tree.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryGroup {
    pub key: String,

    pub name: String,

    #[serde(rename = "childCategories")]
    pub child_categories: Vec<Category>,

    /// Optional text that is accessed by clicking a "ⓘ" button next to the display name
    #[serde(rename = "infoButtonText", skip_serializing_if = "Option::is_none")]
    pub info_button_text: Option<Vec<String>>,

    /// Optional hyperlink shown next to the display name
    #[serde(rename = "infoLink", skip_serializing_if = "Option::is_none")]
    pub info_link: Option<InfoLink>,
}

impl CategoryGroup {
    pub fn new(key: &str, name: &str, child_categories: Vec<Category>) -> Self {
        Self {
            key: key.to_string(),
            name: name.to_string(),
            child_categories,
            info_button_text: None,
            info_link: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoLink {
    /// The link URL
    pub link: String,

    /// The link text
    pub display: String,
}

/// A catalogue item that lets the user select one or more criteria from a predefined list.
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleSelectCategory {
    /// A key that uniquely identifies the catalogue item. It is typically used to look up the CQL snippet for that item.
    pub key: String,

    /// The item's user-facing display name
    pub name: String,

    pub system: String,

    #[serde(rename = "type")]
    pub r#type: String, // "EQUALS"

    pub criteria: Vec<Criteria>,

    /// Optional text that is accessed by clicking a "ⓘ" button next to the display name
    #[serde(rename = "infoButtonText", skip_serializing_if = "Option::is_none")]
    pub info_button_text: Option<Vec<String>>,

    /// This overwrites the display name in the catalogue component only. The intended use-case is to have multiple catalogue items with the same key and name but different subCategoryName. They will appear as different collapsable entries in the catalogue but will be grouped together in the same chip in the search bar.
    #[serde(rename = "subCategoryName", skip_serializing_if = "Option::is_none")]
    pub sub_category_name: Option<String>,
}

impl SingleSelectCategory {
    pub fn new(key: &str, name: &str, system: &str, criteria: Vec<Criteria>) -> Self {
        Self {
            key: key.to_string(),
            name: name.to_string(),
            system: system.to_string(),
            r#type: "EQUALS".to_string(),
            criteria,
            info_button_text: None,
            sub_category_name: None,
        }
    }
}

/// A catalogue item that lets the user select one or more criteria from a predefined list via autocomplete.
#[derive(Debug, Serialize, Deserialize)]
pub struct AutocompleteCategory {
    /// A key that uniquely identifies the catalogue item. It is typically used to look up the CQL snippet for that item.
    pub key: String,

    /// The item's user-facing display name
    pub name: String,

    pub system: String,

    #[serde(rename = "type")]
    pub r#type: String, // "EQUALS"

    pub criteria: Vec<Criteria>,

    /// Optional text that is accessed by clicking a "ⓘ" button next to the display name
    #[serde(rename = "infoButtonText", skip_serializing_if = "Option::is_none")]
    pub info_button_text: Option<Vec<String>>,
}

/// A catalogue item that lets the user specify a numeric range.
#[derive(Debug, Serialize, Deserialize)]
pub struct NumericRangeCategory {
    /// A key that uniquely identifies the catalogue item. It is typically used to look up the CQL snippet for that item.
    pub key: String,

    /// The item's user-facing display name
    pub name: String,

    pub system: String,

    #[serde(rename = "type")]
    pub r#type: String, // "BETWEEN"

    /// The smallest value that the user can enter
    pub min: Option<f64>,

    /// The largest value that the user can enter
    pub max: Option<f64>,

    /// Optional text that is accessed by clicking a "ⓘ" button next to the display name
    #[serde(rename = "infoButtonText", skip_serializing_if = "Option::is_none")]
    pub info_button_text: Option<Vec<String>>,

    #[serde(rename = "unitText")]
    pub unit_text: Option<String>,
}

/// A catalogue item that lets the user specify a date range.
#[derive(Debug, Serialize, Deserialize)]
pub struct DateRangeCategory {
    /// A key that uniquely identifies the catalogue item. It is typically used to look up the CQL snippet for that item.
    pub key: String,

    /// The item's user-facing display name
    pub name: String,

    pub system: String,

    #[serde(rename = "type")]
    pub r#type: String, // "BETWEEN"

    /// The earliest date that the user can pick
    pub min: Option<String>, // ISO date string

    /// The latest date that the user can pick
    pub max: Option<String>, // ISO date string

    /// Optional text that is accessed by clicking a "ⓘ" button next to the display name
    #[serde(rename = "infoButtonText", skip_serializing_if = "Option::is_none")]
    pub info_button_text: Option<Vec<String>>,
}

/// A catalogue item that lets the user specify a string.
#[derive(Debug, Serialize, Deserialize)]
pub struct StringCategory {
    /// A key that uniquely identifies the catalogue item. It is typically used to look up the CQL snippet for that item.
    pub key: String,

    /// The item's user-facing display name
    pub name: String,

    pub system: String,

    #[serde(rename = "type")]
    pub r#type: String, // "EQUALS"

    /// Optional text that is accessed by clicking a "ⓘ" button next to the display name
    #[serde(rename = "infoButtonText", skip_serializing_if = "Option::is_none")]
    pub info_button_text: Option<Vec<String>>,
}

/// A criterion that can be selected in a single-select or autocomplete catalogue item.
#[derive(Debug, Serialize, Deserialize)]
pub struct Criteria {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,

    /// A key that uniquely identifies the criterion
    pub key: String,

    /// The criterion's user-facing display name
    pub name: String,

    /// Optional description that is shown next to the display name during autocompletion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "aggregatedValue", skip_serializing_if = "Option::is_none")]
    pub aggregated_value: Option<Vec<Vec<AggregatedValue>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgroup: Option<Vec<Criteria>>,
}

impl Criteria {
    pub fn new(key: &str, name: &str) -> Self {
        Self {
            visible: None,
            key: key.to_string(),
            name: name.to_string(),
            description: None,
            aggregated_value: None,
            subgroup: None,
        }
    }

    pub fn new_with_description(key: &str, name: &str, description: &str) -> Self {
        Self {
            visible: None,
            key: key.to_string(),
            name: name.to_string(),
            description: Some(description.to_string()),
            aggregated_value: None,
            subgroup: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregatedValue {
    pub value: String,
    pub name: String,
}
