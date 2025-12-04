use crate::core::drill_point::DrillPoint;

const API_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQjXD1Z1nrpTS60VhvlyI3Gha7bS-XP1r_nv3ITYbw4JBL-FA8SB6irRsVHhlEje5ZZT_H8uwFuRGgw/pub?gid=0&single=true&output=csv";

pub async fn get_drills() -> Vec<DrillPoint> {
    let csv = reqwest::get(API_URL).await.unwrap().text().await.unwrap();
    let rows = parse_rows(csv);
    let drills = rows
        .into_iter()
        .map(|row| DrillPoint::new(row.chapter, row.word, row.meaning))
        .collect::<Vec<_>>();
    drills
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
