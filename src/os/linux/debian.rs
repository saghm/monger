use super::LinuxType;
use crate::util::get_from_str;

pub fn check_debian(id: &str, version_id: Option<&str>) -> Option<LinuxType> {
    if id == "debian" {
        return check_debian_version(version_id);
    }

    None
}

fn check_debian_version(version_id: Option<&str>) -> Option<LinuxType> {
    let mut version_numbers = try_option!(version_id).split('.');
    let major_version_str = try_option!(version_numbers.next());
    let major_version: u8 = try_option!(get_from_str(major_version_str));

    if major_version == 8 {
        return Some(LinuxType::Debian8);
    }

    if major_version == 7 {
        return Some(LinuxType::Debian7);
    }

    None
}
