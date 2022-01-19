use rusty_dependencies::pip_client;

#[test]
fn cleanup_package_name_test() {
    assert_eq!("package", pip_client::cleanup_package_name(String::from("package")));
    assert_eq!("package/0", pip_client::cleanup_package_name(String::from("package=0")));
    assert_eq!("package", pip_client::cleanup_package_name(String::from("package>0")));
    assert_eq!("package/0", pip_client::cleanup_package_name(String::from("package<0")));
}

#[test]
fn extract_license_string_test() {
    let apache_license = String::from("License:</strong> Apache </p>");
    assert_eq!("Apache", pip_client::extract_license_string(apache_license));
    let not_found_string = String::from("Lice/p>");
    assert_eq!("not found", pip_client::extract_license_string(not_found_string));
}