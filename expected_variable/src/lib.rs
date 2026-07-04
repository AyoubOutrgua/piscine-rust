use case::CaseExt;
use ::edit_distance::edit_distance;
pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let comp_lower = compared.to_lowercase();
        if comp_lower != compared.to_camel().to_lowercase() 
        && comp_lower != compared.to_snake().to_lowercase() {
        return None;
    }
    let expected_len = expected.len() as f64;
    if expected_len == 0.0 { return None; }
    let distance = edit_distance(&comp_lower, &expected.to_lowercase()) as f64;
    let alikeness = ((1.0 - (distance / expected_len)) * 100.0).round();
    (alikeness > 50.0).then(|| format!("{}%", alikeness as i32))
}