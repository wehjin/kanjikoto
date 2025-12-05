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

    pub fn to_meanings(&self) -> Vec<String> {
        self.meaning
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    }
}
