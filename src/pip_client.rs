use std::fs;
use error_chain::error_chain;
use std::io::Read;

// const PYPI_QUERY:&str = "https://pypi.org/search/?q=";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn read_requirements_file(filename: &str) -> Result<()> {
    println!("Reading File {}", filename);
    let content = fs::read_to_string(filename)
        .expect("[Error] in pip_client.rs during file_read_to_string");
    let lines = content.split("\n");
    for line in lines {

        let line_as_string = String::from(line);

        if !line_as_string.starts_with("#") {
            let package_name = cleanup_package_name(line_as_string);
            let result = check_on_pypi(package_name);
            if result.is_err(){
                println!("Error while checking package {}", line)
            }
        }

    }
    Ok(())
}

fn cleanup_package_name(package_line: String)-> String{
    let older_version = package_line.find("<");
    if older_version.is_some() {
        let (package, version) = package_line.split_at(older_version.unwrap());
        let version_name: String = format!("{}/{}", package, version.replace("<", ""));
        return version_name;
    }
    let exact_version = package_line.find("=");
    if exact_version.is_some() {
        let (package, version) = package_line.split_at(exact_version.unwrap());
        let version_name: String = format!("{}/{}", package, version.replace("=", ""));
        return version_name;
    }
    let newer_version = package_line.find(">");
    if newer_version.is_some() {
        let package = package_line.split_at(newer_version.unwrap()).0;
        return String::from(package)
    }
    return package_line
}

fn check_on_pypi(package_name: String) -> Result<()>{
    let url:String = format!("{}{}","https://pypi.org/project/", package_name);
    let mut response = reqwest::blocking::get(url)?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    let mut license_type = "not found";
    let index = body.find("License:</strong> ");
    if index.is_some() {
            let license_start: &str = body.split_at(index.unwrap()+18).1;
            let license_end = license_start.find("</p>");
            license_type = license_start.split_at(license_end.unwrap()).0;
    }

    println!("{}:\n\t{}", package_name, license_type);
    Ok(())
}