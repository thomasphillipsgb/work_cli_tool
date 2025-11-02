use std::fs::File;
use std::io::{BufWriter, Write};

const CSV_HEADER: &str = "Key,Value,Comment";
const VALUE_TAG: &str = "value";
const COMMENT_TAG: &str = "comment";
const NAME_ATTRIBUTE: &'static str = r#"name=""#;

pub fn export(resx_path: &str, csv_path: &str) -> Result<(), String> {
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

    writeln!(writer, "{}", CSV_HEADER)
        .map_err(|e| format!("Failed to write to CSV file: {}", e))?;

    // Extract key-value pairs from RESX and write to CSV
    // This is a basic implementation - you'd want to use proper XML parsing
    for line in resx_content.lines() {
        if line.trim().starts_with("<data name=") {
            // Extract name attribute and value (simplified parsing)
            if let Some(key) = extract_name(line) {
                let value = extract_tag_content(&resx_content, line, VALUE_TAG).unwrap_or_default();
                let comment =
                    extract_tag_content(&resx_content, line, COMMENT_TAG).unwrap_or_default();
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

fn extract_name(line: &str) -> Option<String> {
    let start = line.find(NAME_ATTRIBUTE)? + 6;
    let end = line[start..].find('"')? + start;
    Some(line[start..end].to_string())
}

fn extract_tag_content(resx_content: &str, data_line: &str, tag_name: &str) -> Option<String> {
    let open_tag = format!("<{}>", tag_name);
    let close_tag = format!("</{}>", tag_name);
    let start = resx_content.find(data_line)? + data_line.len();
    let content_start = resx_content[start..].find(&open_tag)? + start + open_tag.len();
    let content_end = resx_content[content_start..].find(&close_tag)? + content_start;
    Some(resx_content[content_start..content_end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_export() {
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
        export(resx_path, csv_path).unwrap();

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
