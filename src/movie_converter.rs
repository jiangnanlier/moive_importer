use std::{
    fs,
    path::{Path, PathBuf},
};

/// 将文件转换为 JSON 格式并返回存放路径。
pub fn read_txt_file_to_json(file_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let txt = fs::read_to_string(file_path)?;
    let mut movies = Vec::new();
    let mut disc_number = 0;

    for line in txt.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(number) = line
            .strip_suffix('.')
            .and_then(|value| value.parse::<u32>().ok())
        {
            disc_number = number;
            continue;
        }

        let fields: Vec<&str> = line.split_whitespace().collect();
        let Ok(year) = fields.first().unwrap_or(&"").parse::<u32>() else {
            continue;
        };
        let details = &fields[1..];
        let english_start = details
            .iter()
            .position(|word| {
                word.chars()
                    .any(|character| character.is_ascii_alphabetic())
            })
            .unwrap_or(details.len());
        let chinese_title = details[..english_start].join(" ");
        let filename = details[english_start..].join(" ");

        movies.push(serde_json::json!({
            "disc_number": disc_number,
            "year": year,
            "chinese_title": chinese_title,
            "filename": filename,
        }));
    }

    let saved_path = file_path.with_extension("json");
    fs::write(&saved_path, serde_json::to_vec_pretty(&movies)?)?;
    Ok(saved_path)
}

#[cfg(test)]
mod tests {
    use super::read_txt_file_to_json;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn converts_movie_rows_to_four_field_json_objects() {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let input_path = std::env::temp_dir().join(format!("movies-{unique_id}.txt"));
        std::fs::write(
            &input_path,
            "DVDs\n\n1.\n1988 虎胆龙威 Die Hard 1.mkv\n2001 兄弟连\n\n2.\n1992 义海雄风 A Few Good Men.mkv\n",
        )
        .unwrap();

        let output_path = read_txt_file_to_json(&input_path).unwrap();
        let output: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&output_path).unwrap()).unwrap();

        assert_eq!(output[0]["disc_number"], 1);
        assert_eq!(output[0]["year"], 1988);
        assert_eq!(output[0]["chinese_title"], "虎胆龙威");
        assert_eq!(output[0]["filename"], "Die Hard 1.mkv");
        assert_eq!(output[1]["year"], 2001);
        assert_eq!(output[1]["chinese_title"], "兄弟连");
        assert_eq!(output[1]["filename"], "");
        assert_eq!(output[2]["disc_number"], 2);

        std::fs::remove_file(input_path).unwrap();
        std::fs::remove_file(output_path).unwrap();
    }
}
