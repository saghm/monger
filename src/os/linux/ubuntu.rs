use super::LinuxType;
use os::arch::Architecture;
use util::get_from_str;

pub fn check_ubuntu(id: &str, version_id: Option<&str>) -> Option<LinuxType> {
    if id == "ubuntu" {
        return check_ubuntu_version(version_id);
    }

    None
}

fn check_ubuntu_version(version_id: Option<&str>) -> Option<LinuxType> {
    let mut version_numbers = try_option!(version_id).split('.');

    match version_numbers.next().and_then(get_from_str::<u8>) {
        Some(i) if i >= 18 => Some(LinuxType::Ubuntu1804),
        Some(i) if i >= 16 => Some(LinuxType::Ubuntu1604(Architecture::X86_64)),
        Some(i) if i >= 14 => Some(LinuxType::Ubuntu1404),
        Some(i) if i >= 12 => Some(LinuxType::Ubuntu1204),
        _ => None,
    }
}
