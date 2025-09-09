use std::io;

#[test]
fn xattr_basic() {
    use rustix::fs::XattrFlags;

    // The error code when an attribute doesn't exist.
    #[cfg(not(apple))]
    let enodata = libc::ENODATA;
    #[cfg(apple)]
    let enodata = libc::ENOATTR;

    let mut empty: [u8; 0] = [];

    assert_eq!(
        rustix::fs::getxattr("/no/such/path", "user.test", &mut empty)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::lgetxattr("/no/such/path", "user.test", &mut empty)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::setxattr("/no/such/path", "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::lsetxattr("/no/such/path", "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::listxattr("/no/such/path", &mut empty)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::llistxattr("/no/such/path", &mut empty)
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::removexattr("/no/such/path", "user.test")
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(
        rustix::fs::lremovexattr("/no/such/path", "user.test")
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );

    assert_eq!(
        rustix::fs::getxattr("Cargo.toml", "user.test", &mut empty)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::lgetxattr("Cargo.toml", "user.test", &mut empty)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::setxattr("Cargo.toml", "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::lsetxattr("Cargo.toml", "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::listxattr("Cargo.toml", &mut empty).unwrap(),
        libc_listxattr("Cargo.toml")
    );
    assert_eq!(
        rustix::fs::llistxattr("Cargo.toml", &mut empty).unwrap(),
        libc_listxattr("Cargo.toml")
    );
    assert_eq!(
        rustix::fs::removexattr("Cargo.toml", "user.test")
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::lremovexattr("Cargo.toml", "user.test")
            .unwrap_err()
            .raw_os_error(),
        enodata
    );

    let file = std::fs::File::open("Cargo.toml").unwrap();
    assert_eq!(
        rustix::fs::fgetxattr(&file, "user.test", &mut empty)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::fsetxattr(&file, "user.test", &[], XattrFlags::REPLACE)
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
    assert_eq!(
        rustix::fs::flistxattr(&file, &mut empty).unwrap(),
        libc_listxattr("Cargo.toml"),
    );
    assert_eq!(
        rustix::fs::fremovexattr(&file, "user.test")
            .unwrap_err()
            .raw_os_error(),
        enodata
    );
}

/// To check the correctness of the tested implementations of *listxattr(), their output can be
/// compared to an external implementation, in this case listxattr() from the libc crate.
fn libc_listxattr(path: &str) -> usize {
    let path = std::ffi::CString::new(path).unwrap();
    let path: *const _ = path.as_ptr();

    let list = std::ffi::CString::new("").unwrap();
    let list = list.as_ptr() as *mut _;

    libc_listxattr_inner(path, list, 0) as usize
}

#[cfg(not(target_os = "macos"))]
fn libc_listxattr_inner(
    path: *const libc::c_char,
    list: *mut libc::c_char,
    length: libc::size_t,
) -> libc::ssize_t {
    unsafe { libc::listxattr(path, list, length) }
}

// Mac has an extra "options" argument to listxattr(), so it needs special handling:
#[cfg(target_os = "macos")]
fn libc_listxattr_inner(
    path: *const libc::c_char,
    list: *mut libc::c_char,
    length: libc::size_t,
) -> libc::ssize_t {
    unsafe { libc::listxattr(path, list, length, 0) }
}
