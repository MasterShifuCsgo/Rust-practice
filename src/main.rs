mod file_system;

#[allow(unused_imports)]
use file_system::file_logger::start_file_logger;
use file_system::file_organizer::start_file_organizer;

fn main() {
    //start_file_logger();
    start_file_organizer();
}
