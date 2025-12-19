pub async fn get_drills_url(url: &str) -> Vec<DrillPoint> {
    let csv = reqwest::get(url).await.unwrap().text().await.unwrap();
    let rows = parse_rows(csv);
    let drills = rows
        .into_iter()
        .map(|row| DrillPoint::new(row.chapter, row.word, row.meaning))
        .collect::<Vec<_>>();
    drills
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DrillPoint {
    pub chapter: usize,
    pub furi: String,
    pub meaning: String,
    pub kanji: String,
    pub yomi: String,
}

impl DrillPoint {
    pub fn new(chapter: usize, furi: impl AsRef<str>, meaning: impl AsRef<str>) -> Self {
        let furi = furi.as_ref().to_string();
        let meaning = meaning.as_ref().to_string();
        let mut kanji_segments = Vec::new();
        let mut yomi_segments = Vec::new();
        for segment in furi.split("）") {
            let kana_yomi = segment.split("（").collect::<Vec<&str>>();
            match kana_yomi.len() {
                2 => {
                    let kana = kana_yomi[0].to_string();
                    kanji_segments.push(kana.clone());
                    let yomi = kana_yomi[1].to_string();
                    if yomi.trim().is_empty() {
                        yomi_segments.push(kana);
                    }
                    yomi_segments.push(yomi);
                }
                1 => {
                    let kana = kana_yomi[0].to_string();
                    kanji_segments.push(kana.clone());
                    yomi_segments.push(kana);
                }
                _ => panic!("Invalid furi segment: {}", segment),
            }
        }
        Self {
            chapter,
            furi,
            meaning,
            kanji: kanji_segments.join(""),
            yomi: yomi_segments.join(""),
        }
    }
}

fn parse_rows(csv: String) -> Vec<CsvRow> {
    let mut rows = vec![];
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for result in reader.deserialize() {
        let row: CsvRow = result.unwrap();
        rows.push(row);
    }
    rows
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CsvRow {
    chapter: usize,
    word: String,
    meaning: String,
}
