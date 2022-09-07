// creates a new python project named by the user and a config.toml file with entries in the config.toml file from stdinput by asking the user
use std::io;
use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::process::Command;
use std::process::exit;
use std::fmt::Write as FmtWrite;
use std::write;

fn main() {
    // ask the user what the name of the project is
    println!("What is the name of your project?");
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name).expect("Failed to read line");
    // remove the newline character from the string
    //project_name.pop();
    project_name.pop();
    // loop while prompting the user for key value pairs for configuration.toml file
    let mut config_values = vec![];
    loop{
        println!("What is the key for this value? enter 'done' when done or to exit");
        let mut key = String::new();
        io::stdin().read_line(&mut key).expect("Failed to read line");
        //key.pop();//because reasons
        key.pop();
        if key == "done"|| key==""{
            break;
        }else{
            println!("What is the value for this key?");
            let mut value = String::new();
            io::stdin().read_line(&mut value).expect("Failed to read line");
            //value.pop();
            value.pop();
            config_values.push((key,value));
        }
    }
    // create the project directory
    let project_dir = Path::new(&project_name);
    if !project_dir.exists(){
        match std::fs::create_dir(format!("{}",project_name)){
            Ok(_) => println!("Project directory created"),
            Err(e) => println!("Failed to create project directory: {}", e),
        }
    }else{
        println!("Project directory already exists");
    }

    // create the config.toml file and write to it
    let mut file = File::create(format!("{}/configuration.toml",project_name)).expect("Failed to create config file");
    println!("Creating config file");
    for (key,value) in config_values.clone(){
        file.write_all(format!("{} = {}\n",key,value).as_bytes()).expect("Failed to write to config file");
    }


    // create the python file
    let python_file = String::from(&project_name) + "/" + &format!("{}.py",project_name);
    println!("Creating python file {}",python_file);
    // ask the user to enter names for each returned object
    let mut object_names = vec![];
    loop{
        println!("What is the name of the returned object? enter 'done' when done or to exit");
        let mut object_name = String::new();
        io::stdin().read_line(&mut object_name).expect("Failed to read line");
        //object_name.pop();//because reasons
        object_name.pop();
        if object_name == "done"|| object_name==""{
            break;
        }else{
            object_names.push(object_name);
        }
    }

    let mut writer = String::new();
    write!(writer,"from solid import *\n").expect("Failed to write to python file");
    write!(writer,"from solid.utils import *\n").expect("Failed to write to python file");
    write!(writer,"import os\n").expect("Failed to write to python file");
    write!(writer,"import toml\n\n").expect("Failed to write to python file");
    write!(writer,"epsilon = 0.0000001\n\n").expect("Failed to write to python file");

    write!(writer,"def {}({}):\n",project_name,config_values.iter().cloned().map(|x| x.0).collect::<Vec<String>>().join(",")).expect("Failed to write to python file");
    write!(writer,"    #start coding!\n").expect("Failed to write to python file");
    //write!(writer,"    {} = None\n",project_name).expect("Failed to write to python file");
    //same as above but for object_names
    for object_name in object_names.clone(){
        write!(writer,"    {} = None\n",object_name).expect("Failed to write to python file");
    }
    // TODO: return a list so we can iterate on one or more objects
    // write!(writer,"    return {}\n",project_name).expect("Failed to write to python file");
    // return each object_name in the list
    write!(writer,"    return {}\n",object_names.iter().cloned().collect::<Vec<String>>().join(",")).expect("Failed to write to python file");

    //render function
    write!(writer,"\n\ndef render_object(render_object, filename):\n").expect("Failed to write to python file");
    write!(writer,"    \"\"\"\n").expect("Failed to write to python file");
    write!(writer,"    creates a .stl and .scad solution for the given solidpython OpenSCAD object\n").expect("Failed to write to python file");
    write!(writer,"    PARAMETERS:\n").expect("Failed to write to python file");
    write!(writer,"        render_object: the OpenSCAD object\n").expect("Failed to write to python file");
    write!(writer,"        filename: a string for the file to be saved\n").expect("Failed to write to python file");
    write!(writer,"    \"\"\"\n").expect("Failed to write to python file");
    write!(writer,"    scad_render_to_file(render_object, filename + \".scad\", file_header=\"$fn=200;\")\n").expect("Failed to write to python file");
    write!(writer,"    # render with OpenSCAD\n").expect("Failed to write to python file");
    write!(writer,"    print(\"Openscad is now rendering the solution..\")\n").expect("Failed to write to python file");
    write!(writer,"    os.system(\"openscad -o \" + filename + \".stl \" + filename + \".scad &\")\n").expect("Failed to write to python file");
    write!(writer,"\n\n").expect("Failed to write to python file");

    // main function
    write!(writer,"if __name__ == \"__main__\":\n").expect("Failed to write to python file");
    write!(writer,"    config = toml.load(\"configuration.toml\")\n").expect("Failed to write to python file");
    //write!(writer,"    {} = {}(**config)\n",project_name,project_name).expect("Failed to write to python file");
    //same as above but for object names as return values and comma seperated
    write!(writer,"    {} = {}(**config)\n",object_names.iter().cloned().collect::<Vec<String>>().join(","),project_name).expect("Failed to write to python file");
    // same as above but give the .scad filename
    // write!(writer,"    render_object({}, \"{}\")\n",project_name,python_file).expect("Failed to write to python file");
    // call render_object for each object_name
    //write!(writer,"    for {} in {}:\n",object_names.iter().cloned().collect::<Vec<String>>().join(","),project_name).expect("Failed to write to python file");
    //instead, call for each object_name explicitly
    for object_name in object_names.clone(){
        write!(writer,"    render_object({}, \"{}\")\n",object_name,object_name).expect("Failed to write to python file");
    }
    //write!(writer,"        render_object({}, \"{}\")\n",object_names.iter().cloned().collect::<Vec<String>>().join(","),python_file).expect("Failed to write to python file");
    write!(writer,"\n").expect("Failed to write to python file");
    write!(writer,"    print(\"Solution saved to {}.stl\")\n",python_file).expect("Failed to write to python file");

    // write the python file
    let mut python_file = File::create(python_file).expect("Failed to create python file");
    python_file.write_all(writer.as_bytes()).expect("Failed to write to python file");

}
