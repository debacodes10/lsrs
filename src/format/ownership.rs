use std::ffi::CStr;

pub fn username_from_uid(uid: u32) -> String {
    unsafe {
        let pwd = libc::getpwuid(uid);
        if pwd.is_null() {
            return uid.to_string();
        }
        let name_ptr = (*pwd).pw_name;
        if name_ptr.is_null() {
            return uid.to_string();
        }
        CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
    }
}

pub fn groupname_from_gid(gid: u32) -> String {
    unsafe {
        let grp = libc::getgrgid(gid);
        if grp.is_null() {
            return gid.to_string();
        }
        let grp_ptr = (*grp).gr_name;
        if grp_ptr.is_null() {
            return gid.to_string();
        }
        CStr::from_ptr(grp_ptr).to_string_lossy().into_owned()
    }
}
