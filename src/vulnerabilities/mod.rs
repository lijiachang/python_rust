pub mod structs;
pub mod processes;

use structs::VulnerabilityFootPrint;
use processes::{merge_footprint_with_vulnerabilities, read_vulnerabilities};
use crate::footprint::structs::FootPrint;

/// 灾难足迹合并流程和脆弱性合并流程
pub fn merge_vulnerabilities_with_footprint(foot_print: Vec<FootPrint>, base_path: String) -> Vec<VulnerabilityFootPrint> {
    let vulnerabilities = read_vulnerabilities(base_path).unwrap();
    merge_footprint_with_vulnerabilities(vulnerabilities, foot_print)
}