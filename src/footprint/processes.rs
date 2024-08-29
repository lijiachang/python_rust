use std::error::Error;
use csv;

use super::structs::FootPrint;

/// 从文件中读取灾难足迹数据
pub fn read_footprint(mut base_path: String) -> Result<Vec<FootPrint>, Box<dyn Error>> {
    base_path.push_str("/footprint.csv");
    let mut rdr = csv::Reader::from_path(&base_path)?;

    let mut buffer = Vec::new();

    for result in rdr.deserialize(){
        let record: FootPrint = result?;
        buffer.push(record);
    }

    Ok(buffer)
}

/// 在同一个文件中合并灾难足迹数据
pub fn merge_footprint_with_events(event_ids: Vec<i32>, footprints: Vec<FootPrint>) -> Vec<FootPrint> {
    let mut buffer = Vec::new();

    for event_id in event_ids{
        for footprint in &footprints{
            if footprint.event_id == event_id{
                buffer.push(footprint.clone())
            }
        }
    }

    buffer
}