use crate::core::drill_point::DrillPoint;

const API_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQjXD1Z1nrpTS60VhvlyI3Gha7bS-XP1r_nv3ITYbw4JBL-FA8SB6irRsVHhlEje5ZZT_H8uwFuRGgw/pub?gid=0&single=true&output=csv";

pub async fn get_drills() -> Vec<DrillPoint> {
    let drills = vec![
        DrillPoint::new(1, "始（はじ）まり", "the beginning"),
        DrillPoint::new(
            1,
            "幸（こう）か（）不（ふ）幸（こう）か",
            "for better or worse, lucky or unlucky",
        ),
    ];
    drills
}
