use super::LinuxType;
use util::get_from_str;

pub fn check_suse(id: &str, version_id: Option<&str>) -> Option<LinuxType> {
    if id == "suse" {
        return check_suse_version(version_id);
    }

    None
}

fn check_suse_version(version_id: Option<&str>) -> Option<LinuxType> {
    let mut version_numbers = try_option!(version_id).split('.');

    match version_numbers.next().and_then(get_from_str::<u8>) {
        Some(i) if i >= 12 => Some(LinuxType::Suse12),
        Some(i) if i >= 11 => Some(LinuxType::Suse11),
        _ => None,
    }
}
