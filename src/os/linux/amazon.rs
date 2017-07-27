use super::LinuxType;

pub fn check_amazon(id: &str) -> Option<LinuxType> {
    // Why do you not like vowels, Amazon...
    if id == "amzn" {
        return Some(LinuxType::Amazon);
    } else {
        None
    }
}
