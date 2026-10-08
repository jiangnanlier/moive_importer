use std::{
    fs,
    path::{Path, PathBuf},
};
use regex::Regex;

/// 将文件转换为 JSON 格式并返回存放路径。
pub fn convert_txt_to_json(file_path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let txt = fs::read_to_string(file_path)?;
    let mut movies = Vec::new();
    let mut disc_number = 0;
    // 匹配行末的中文括号备注，如 "（儿童）"，组1为备注内容
    let remark_re = Regex::new(r"[ \t]*（(.+?)）[ \t]*$")?;

    for line in txt.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(number) = line
            .strip_suffix('.')
            .and_then(|value| value.parse::<u32>().ok())
        {
            disc_number = number;
            continue;
        }

        // 提取并剥离末尾的中文括号备注
        let (remark, cleaned) = match remark_re.captures(line) {
            Some(caps) => (
                caps.get(1).map(|m| m.as_str().to_string()),
                remark_re.replace(line, "").into_owned(),
            ),
            None => (None, line.to_string()),
        };

        let mut fields = cleaned.split_whitespace();
        let Some(year_text) = fields.next() else {
            continue;
        };
        let Ok(year) = year_text.parse::<u32>() else {
            continue;
        };
        let details: Vec<&str> = fields.collect();
        let english_start = details
            .iter()
            .position(|word| word.bytes().any(|byte| byte.is_ascii_alphabetic()))
            .unwrap_or(details.len());
        let chinese_title = details[..english_start].join(" ");
        let filename = details[english_start..].join(" ");

        movies.push(serde_json::json!({
            "disc_number": disc_number,
            "year": year,
            "chinese_title": chinese_title,
            "filename": filename,
            "remark": remark,
        }));
    }

    let saved_path = file_path.with_extension("json");
    fs::write(&saved_path, serde_json::to_vec_pretty(&movies)?)?;
    Ok(saved_path)
}

#[cfg(test)]
mod tests {
    use super::convert_txt_to_json;
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
            "DVDs\n\n1.\n1988 虎胆龙威 Die Hard 1.mkv\n2001 兄弟连\n2010 卑鄙的我 Despicable Me.mkv（儿童）\n\n2.\n1992 义海雄风 A Few Good Men.mkv\n",
        )
        .unwrap();

        let output_path = convert_txt_to_json(&input_path).unwrap();
        let output: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&output_path).unwrap()).unwrap();

        assert_eq!(output[0]["disc_number"], 1);
        assert_eq!(output[0]["year"], 1988);
        assert_eq!(output[0]["chinese_title"], "虎胆龙威");
        assert_eq!(output[0]["filename"], "Die Hard 1.mkv");
        assert!(output[0]["remark"].is_null());
        assert_eq!(output[1]["year"], 2001);
        assert_eq!(output[1]["chinese_title"], "兄弟连");
        assert_eq!(output[1]["filename"], "");
        assert_eq!(output[2]["chinese_title"], "卑鄙的我");
        assert_eq!(output[2]["filename"], "Despicable Me.mkv");
        assert_eq!(output[2]["remark"], "儿童");
        assert_eq!(output[3]["disc_number"], 2);

        std::fs::remove_file(input_path).unwrap();
        std::fs::remove_file(output_path).unwrap();
    }
}
