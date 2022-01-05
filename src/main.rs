mod pip_client;
use pip_client::read_requirements_file;


fn main(){
    read_requirements_file("./test_data/sample_requirements.txt");
}
