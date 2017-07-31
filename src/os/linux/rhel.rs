use super::LinuxType;
use util::get_from_str;

pub fn check_rhel(id: &str, version_id: Option<&str>) -> Option<LinuxType> {
    if id == "rhel" || id == "centos" {
        return check_rhel_version(version_id);
    }

    None
}

fn check_rhel_version(version_id: Option<&str>) -> Option<LinuxType> {
    let mut version_numbers = try_option!(version_id).split('.');

    match version_numbers.next().and_then(get_from_str::<u8>) {
        Some(i) if i >= 7 => Some(LinuxType::Rhel7),
        Some(i) if i >= 6 => Some(LinuxType::Rhel6),
        _ => None,
    }
}
