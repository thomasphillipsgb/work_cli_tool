use std::fs::File;
use std::io::{BufWriter, Write};

pub struct TranslationExporter;

impl TranslationExporter {
    pub fn new() -> Self {
        TranslationExporter
    }

    pub fn export(&self, resx_path: &str, csv_path: &str) -> Result<(), String> {
        // Remove surrounding quotes if present
        let resx_path = resx_path.trim_matches('"');
        let csv_path = csv_path.trim_matches('"');

        // Parse RESX file (TODO: implement proper RESX parsing)
        let resx_content = std::fs::read_to_string(resx_path)
            .map_err(|e| format!("Failed to read RESX file: {}", e))?;

        // Create CSV file
        let csv_file =
            File::create(csv_path).map_err(|e| format!("Failed to create CSV file: {}", e))?;
        let mut writer = BufWriter::new(csv_file);

        // Write CSV header
        writeln!(writer, "Key,Value,Comment")
            .map_err(|e| format!("Failed to write to CSV file: {}", e))?;

        // Extract key-value pairs from RESX and write to CSV
        // This is a basic implementation - you'd want to use proper XML parsing
        for line in resx_content.lines() {
            if line.trim().starts_with("<data name=") {
                // Extract name attribute and value (simplified parsing)
                if let Some(key) = extract_name(line) {
                    let value = extract_value(&resx_content, line).unwrap_or_default();
                    let comment = extract_comment(&resx_content, line).unwrap_or_default();
                    writeln!(
                        writer,
                        "\"{}\",\"{}\",\"{}\"",
                        key.replace("\"", "\"\""),
                        value.replace("\"", "\"\""),
                        comment.replace("\"", "\"\"")
                    )
                    .map_err(|e| format!("Failed to write to CSV file: {}", e))?;
                }
            }
        }

        writer
            .flush()
            .map_err(|e| format!("Failed to flush CSV file: {}", e))?;
        Ok(())
    }
}

fn extract_name(line: &str) -> Option<String> {
    let start = line.find("name=\"")? + 6;
    let end = line[start..].find('"')? + start;
    Some(line[start..end].to_string())
}

fn extract_value(resx_content: &str, data_line: &str) -> Option<String> {
    let start_tag = "<value>";
    let end_tag = "</value>";
    let start = resx_content.find(data_line)? + data_line.len();
    let value_start = resx_content[start..].find(start_tag)? + start + start_tag.len();
    let value_end = resx_content[value_start..].find(end_tag)? + value_start;
    Some(resx_content[value_start..value_end].trim().to_string())
}

fn extract_comment(resx_content: &str, data_line: &str) -> Option<String> {
    let start_tag = "<comment>";
    let end_tag = "</comment>";
    let start = resx_content.find(data_line)? + data_line.len();
    let comment_start = resx_content[start..].find(start_tag)? + start + start_tag.len();
    let comment_end = resx_content[comment_start..].find(end_tag)? + comment_start;
    Some(resx_content[comment_start..comment_end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_export() {
        let exporter = TranslationExporter::new();
        let resx_content = r#"
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

        let resx_path = "test.resx";
        let csv_path = "test.csv";

        fs::write(resx_path, resx_content).unwrap();
        exporter.export(resx_path, csv_path).unwrap();

        let csv_content = fs::read_to_string(csv_path).unwrap();
        let expected_csv = r#"Key,Value,Comment
"Hello","Hello World","This is a greeting"
"Goodbye","Goodbye World",""
"#;

        // TODO: have something DI-able so we aren't actually writing files in tests
        fs::remove_file(resx_path).unwrap();
        fs::remove_file(csv_path).unwrap();

        assert_eq!(csv_content, expected_csv);
    }
}
