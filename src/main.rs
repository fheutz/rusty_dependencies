use rusty_dependencies::pip_client::print_licenses_from_requirement_file;

fn main(){
    let package_license_map = print_licenses_from_requirement_file("./test_data/sample_requirements.txt");
    if package_license_map.is_err() {
        println!("Error, could not generate license map from requirements.txt")
    }else {
        for (package, license) in &package_license_map.unwrap() {
            println!("{} \n  {}", package, license);
        }
    }
}
