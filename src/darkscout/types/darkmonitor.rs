use rand::Rng;
use serde::{Deserialize, Serialize};

fn get_inbetween_colors(color1: &str, color2: &str, count: usize) -> Vec<String> {
    let c1 = i64::from_str_radix(&color1[1..], 16).unwrap();
    let c2 = i64::from_str_radix(&color2[1..], 16).unwrap();
    let interval = (c2 - c1) / (count as i64);

    let mut array = vec![color1.to_string()];

    for i in 1..count {
        let intermediate_color = c1 + i as i64 * interval;
        let r = (intermediate_color >> 16) as u8;
        let g = ((intermediate_color >> 8) & 0xFF) as u8;
        let b = (intermediate_color & 0xFF) as u8;
        array.push(format!("#{:02X}{:02X}{:02X}", r, g, b));
    }
    array.push(color2.to_string());

    array
}

fn generate_color() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(1..30) * 5;
    let colors = get_inbetween_colors("#41445F", "#F9B759", random_index);
    let random_index = rng.gen_range(0..colors.len());
    colors[random_index].clone()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmailStats {
    pub breach_info: Vec<BreachInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CachedEmailStats {
    pub _id: String, // email is the id
    pub email_stats: EmailStats,
}

#[derive(Serialize, Deserialize)]
pub struct DomainStats {
    pub breach_info: Vec<BreachInfo>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BreachInfo {
    pub name: String,
    pub title: String,
    pub description: String,
    pub domain: Option<String>,
    pub breach_date: Option<String>,
    pub added_date: Option<String>,
    pub modified_date: Option<String>,
    pub pwn_count: Option<u64>,
    pub data_classes: Option<Vec<String>>,
    pub logo_path: String,
    pub is_verified: Option<bool>,
    pub is_fabricated: Option<bool>,
    pub is_sensitive: Option<bool>,
    pub is_retired: Option<bool>,
    pub is_spamlist: Option<bool>,
    pub is_malicious_verified: Option<bool>,
    pub is_subscription_free: Option<bool>,
    pub country_code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmailAnalytics {
    pub chart_data: PieChartData,
    pub radial_stack_data: RadialStackData,
    pub geo_data: GeoGraphData,
    pub bento_data: BentoData,
    pub list_data: Vec<BreachInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadialStackData {
    pub data: Vec<RadialStackElementData>,
    pub config: RadialStackConfig
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadialStackElementData {
    pub leak_name: String,
    pub visitors: u64,
    pub fill: String,
}
    // { browser: "chrome", visitors: 275, fill: "#009688" },


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadialStackConfig {
    pub total_found: TotalFoundLabel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalFoundLabel {
    pub label: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PieChartData {
    pub data: Vec<ChartElementData>,
    pub fill_info: Vec<FillInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub id: String, // name of the breach
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FillInfo {
    #[serde(rename = "match")]
    pub match_: Match,
    pub id: String, // name of the style, dots, lines, etc
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoGraphData {
    data: Vec<GeoGraphElementData>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChartElementData {
    pub id: String,
    pub label: String,
    pub value: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoGraphElementData {
    pub id: String, // country code
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BentoData {
    pub total_records: TotalRecords,
    pub total_breaches: TotalBreaches,
    pub total_detected_breaches: TotalDetectedBreaches,
    pub total_sensitive_breaches: TotalSensitiveBreaches,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalRecords {
    pub field_name: String,
    pub value: String,
    pub increase_percentage: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalBreaches {
    pub field_name: String,
    pub value: String,
    pub increase_percentage: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalSensitiveBreaches {
    pub field_name: String,
    pub value: String,
    pub increase_percentage: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TotalDetectedBreaches {
    pub field_name: String,
    pub value: String,
    pub increase_percentage: String,
}

pub trait EmailStatsConverter {
    fn convert_to_email_analytics(&self) -> EmailAnalytics;
    fn create_bento_data(&self) -> BentoData;
    fn create_geo_graph_data(&self) -> GeoGraphData;
    fn create_radial_stack_data(&self) -> RadialStackData;
}

impl EmailStatsConverter for EmailStats {
    fn create_geo_graph_data(&self) -> GeoGraphData {
        let mut geo_data: Vec<GeoGraphElementData> = vec![];
        // let mut country_counts: std::collections::HashMap<String, u64> = std::collections::HashMap::new();

        for breach in &self.breach_info {
            geo_data.push(GeoGraphElementData {
                id: breach.country_code.clone(),
                value: breach.pwn_count.unwrap_or(0).to_string(),
            });
        }

        GeoGraphData { data: geo_data }
    }

    fn create_bento_data(&self) -> BentoData {
        let total_records_from_breach: u64 = self
            .breach_info
            .iter()
            .map(|breach| breach.pwn_count.unwrap_or(0))
            .sum();
        let total_breanches = 789;
        let total_sensitive_breaches = self
            .breach_info
            .iter()
            .filter(|breach| breach.is_sensitive.unwrap_or(false))
            .count();
        let total_detected_breaches = self.breach_info.len();
        return BentoData {
            total_records: TotalRecords {
                field_name: "Total Records".to_string(),
                value: total_records_from_breach.to_string(),
                increase_percentage: "100".to_string(),
            },
            total_breaches: TotalBreaches {
                field_name: "Total Breaches".to_string(),
                value: total_breanches.to_string(),
                increase_percentage: "100".to_string(),
            },
            total_sensitive_breaches: TotalSensitiveBreaches {
                field_name: "Total Sensitive Breaches".to_string(),
                value: total_sensitive_breaches.to_string(),
                increase_percentage: "100".to_string(),
            },
            total_detected_breaches: TotalDetectedBreaches {
                field_name: "Total Detected Breaches".to_string(),
                value: total_detected_breaches.to_string(),
                increase_percentage: "100".to_string(),
            },
        };
        // put increase percentages as 100 for now
    }
    fn create_radial_stack_data(&self) -> RadialStackData {
        let label = TotalFoundLabel {
            label: "Total Found".to_string(),
        };
        let data = self.breach_info.iter().map(|breach| {
            let leak_name = breach.name.clone();
            let visitors = breach.pwn_count.unwrap_or(0);
            let fill = generate_color();
            RadialStackElementData {
                leak_name,
                visitors,
                fill,
            }
        }).collect();

        return RadialStackData {
            data,
            config: RadialStackConfig {
                total_found: label,
            }
        };
    }
    fn convert_to_email_analytics(&self) -> EmailAnalytics {
        let chart_data = self
            .breach_info
            .iter()
            .map(|breach| {
                let color = generate_color();
                let value = breach.pwn_count.unwrap_or(0).to_string();
                ChartElementData {
                    id: breach.name.clone(),
                    label: breach.name.clone(),
                    value,
                    color,
                }
            })
            .collect();

        let mut toggle = false;
        let fill_info = self
            .breach_info
            .iter()
            .map(|breach| {
                let match_ = Match {
                    id: breach.name.clone(),
                };
                if toggle {
                    toggle = !toggle;
                    return FillInfo {
                        match_,
                        id: "lines".to_string(),
                    };
                }
                FillInfo {
                    match_,
                    id: "dots".to_string(),
                }
            })
            .collect();

        let geo_data = self.create_geo_graph_data();

        EmailAnalytics {
            chart_data: PieChartData {
                data: chart_data,
                fill_info,
            },
            geo_data,
            bento_data: self.create_bento_data(),
            list_data: self.breach_info.clone(),
            radial_stack_data: self.create_radial_stack_data(),
        }
    }
}
// pub trait EmailStatsConverter {
//     fn convert_to_email_analytics(&self) -> EmailAnalytics;
// }
//
// impl EmailStatsConverter for EmailStats {
//     fn convert_to_email_analytics(&self) -> EmailAnalytics {
//         let chart_data = self.breach_info.iter().map(|breach| {
//             let color = generate_color();
//             let value = breach.pwn_count.unwrap_or(0).to_string();
//             ChartElementData {
//                 id: breach.name.clone(),
//                 label: breach.name.clone(),
//                 value,
//                 color,
//             }
//         }).collect();
//
//         let geo_data = self.create_geo_graph_data();
//
//         EmailAnalytics {
//             chart_data: PieChartData { data: chart_data },
//             geo_data,
//         }
//     }
// }
//
// impl EmailStats {
//     fn create_geo_graph_data(&self) -> GeoGraphData {
//         let mut geo_data: Vec<GeoGraphElementData> = vec![];
//         let mut country_counts: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
//
//         for breach in &self.breach_info {
//             if let Some(country_code) = &breach.country_code {
//                 let count = country_counts.entry(country_code.clone()).or_insert(0);
//                 *count += 1;
//             }
//         }
//
//         for (country_code, count) in country_counts {
//             geo_data.push(GeoGraphElementData {
//                 id: country_code.clone(),
//                 value: count.to_string(),
//             });
//         }
//
//         GeoGraphData { data: geo_data }
//     }
// }
// Total Records
// $45,231.89
//
// +20.1% from last month
// Total Breaches
// 789
//
// +20.1% from last month
// Total Detected Breaches
// 13
//
// +20.1% from last month
// Sensitive Breaches
// 6
//
// +20.1% from last month
