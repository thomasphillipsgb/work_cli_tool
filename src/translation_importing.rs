use std::fs::File;
use std::io::{BufWriter, Write};

pub struct TranslationImporter;

impl TranslationImporter {
    pub fn new() -> Self {
        TranslationImporter
    }

    pub fn import(&self, csv_path: &str, resx_path: &str) -> Result<(), String> {
        // Remove surrounding quotes if present
        let csv_path = csv_path.trim_matches('"');
        let csv_path = csv_path.trim_matches('"');
        // Read CSV file
        let csv_content = std::fs::read_to_string(csv_path)
            .map_err(|e| format!("Failed to read CSV file: {}", e))?;
        // Create or overwrite RESX file
        let mut resx_file =
            File::create(resx_path).map_err(|e| format!("Failed to create RESX file: {}", e))?;
        // Write RESX header
        writeln!(resx_file, r#"<?xml version="1.0" encoding="utf-8"?>"#)
            .map_err(|e| format!("Failed to write to RESX file: {}", e))?;
        writeln!(resx_file, r#"<root>"#)
            .map_err(|e| format!("Failed to write to RESX file: {}", e))?;
        // Process CSV lines
        for (i, line) in csv_content.lines().enumerate() {
            if i == 0 {
                // Skip header
                continue;
            }
            let parts: Vec<&str> = line.splitn(3, ',').collect();
            if parts.len() < 2 {
                continue; // Skip malformed lines
            }
            let key = parts[0].trim_matches('"');
            let value = parts[1].trim_matches('"');
            let comment = if parts.len() > 2 {
                parts[2].trim_matches('"')
            } else {
                ""
            };
            writeln!(resx_file, r#"    <data name="{}">"#, key)
                .map_err(|e| format!("Failed to write to RESX file: {}", e))?;
            writeln!(resx_file, r#"        <value>{}</value>"#, value)
                .map_err(|e| format!("Failed to write to RESX file: {}", e))?;
            if !comment.is_empty() {
                writeln!(resx_file, r#"        <comment>{}</comment>"#, comment)
                    .map_err(|e| format!("Failed to write to RESX file: {}", e))?;
            }
            writeln!(resx_file, r#"    </data>"#)
                .map_err(|e| format!("Failed to write to RESX file: {}", e))?;
        }
        writeln!(resx_file, r#"</root>"#)
            .map_err(|e| format!("Failed to write to RESX file: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_import() {
        let importer = TranslationImporter::new();
        let csv_content = r#"Key,Value,Comment
"Hello","Hello World","This is a greeting"
"Goodbye","Goodbye World",""
"#;

        let resx_path = "test1.resx";
        let csv_path = "test1.csv";

        fs::write(csv_path, csv_content).unwrap();
        importer.import(csv_path, resx_path).unwrap();

        let resx_content = fs::read_to_string(resx_path).unwrap();
        let expected_resx = r#"<?xml version="1.0" encoding="utf-8"?>
<root>
    <data name="Hello">
        <value>Hello World</value>
        <comment>This is a greeting</comment>
    </data>
    <data name="Goodbye">
        <value>Goodbye World</value>
    </data>
</root>
"#;

        // TODO: have something DI-able so we aren't actually writing files in tests
        fs::remove_file(resx_path).unwrap();
        fs::remove_file(csv_path).unwrap();

        assert_eq!(resx_content, expected_resx);
    }
}
