# icd-codes-extractor

Extracts ICD-O-3 & ICD10 codes from files & creates CSV files out of those, so that anyone wanting to use these codes may re-shape the data in a desired format.

| Codes | Source | Target |
|-------|--------|--------|
| ICDO3 | `resources/sitetype.icdo3.20220429.pdf` | `resources/icdo3_full.csv` |
| ICD10 | `resources/catalogue-with-icd10-codes.json` | `resources/icd10_full.csv` |

## Motivation

The ICDO3 codes had to be translated from German to English, and to avoid this pain, the generated CSV file can be used from any language and the codes translated to any desired data structure.

Similarly, the ICD10 codes are in a Lens specific JSON, and the output CSV file serves the same purpose (as above).
