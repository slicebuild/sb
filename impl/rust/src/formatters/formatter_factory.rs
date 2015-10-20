//what are we returning instead of an iformatter?
// In order to use this function, you will have to use env::args().collect().to_vec() or something
// like that when you call it.
use formatter_docker::*;
use formatter_shell::*;
pub fn GetFormatter(args:Vec<String>) {
  //What are we storing the compiler flags as in this program? Is a vector okay?
    let args_vector = args.split("").collect();
    if args_vector[0].to_lowercase()  == "docker".to_string() {
        println!("Run the docker"); // Not sure how this works.
    }

    else if args_vector[0].to_lowercase() == "shell".to_string() {
        println!("run the shell"); // Same here.
    }
    else {
        return;
    }
}
