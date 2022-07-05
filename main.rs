
use std::process::{Command};
use std::env;
use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::process;


struct PM {
    command: String,
    install: String,
    remove: String,
    refresh: String,
    update: String,
    autoremove: String,
    search: String
}


fn main() -> io::Result<()> {

    // get user's distro info by /etc/os-release
    let release = get_distro();

    // if user's distro is not supported, inform the user and exit
    if release.eq("not found") {
        println!("dpm couldn't detect your distro");
        process::exit(1);
    }

    // initialize the struct
    let mut current_pm = PM { command: String::from(""), install: String::from(""), 
                                remove: String::from(""), refresh: String::from(""), 
                                update: String::from(""), autoremove: String::from(""),
                                search:String::from("")};

    // match distro ID to its package manager and its basic commands
    match (release).as_str() {
        "neon" =>  {current_pm.command = String::from("pkcon");
                        current_pm.install = String::from("install");
                        current_pm.remove = String::from("remove");
                        current_pm.refresh = String::from("refresh");
                        current_pm.update = String::from("update");
                        current_pm.autoremove = String::from("autoremove");
                        current_pm.search = String::from("search");},

        "ubuntu" | "debian" |"zorin" | "pop" | "Deepin" | "elementary" | "linuxmint" => 
                        {current_pm.command = String::from("apt");
                        current_pm.install = String::from("install");
                        current_pm.remove = String::from("remove");
                        current_pm.refresh = String::from("update");
                        current_pm.update = String::from("upgrade");
                        current_pm.autoremove = String::from("autoremove");
                        current_pm.search = String::from("search");},

        "centos" | "fedora" | "rocky" =>
                        {current_pm.command = String::from("dnf");
                        current_pm.install = String::from("install");
                        current_pm.remove = String::from("remove");
                        current_pm.refresh = String::from("check-update");
                        current_pm.update = String::from("upgrade");
                        current_pm.autoremove = String::from("autoremove");
                        current_pm.search = String::from("search");},

        "manjaro" | "arch" | "endeavourOS" | "garuda" =>
                        {current_pm.command = String::from("pacman");
                        current_pm.install = String::from("-S");
                        current_pm.remove = String::from("-R");
                        current_pm.refresh = String::from("-Syy");
                        current_pm.update = String::from("-Syu");
                        current_pm.autoremove = String::from("-Scc");
                        current_pm.search = String::from("-Ss");},

        "alpine" =>
                        {current_pm.command = String::from("apk");
                        current_pm.install = String::from("add");
                        current_pm.remove = String::from("del");
                        current_pm.refresh = String::from("update");
                        current_pm.update = String::from("upgrade");
                        current_pm.autoremove = String::from("cache clean");
                        current_pm.search = String::from("search");},

        _ => current_pm.command = String::from("none")
    }
    
    // get the arguements
    let args: Vec<String> = env::args().collect();

    let current_action;

    // arguement[0] is the program name, so arguement[1] is the command.
    // check is command given by the user is included in the supported ones
    match (args[1].clone()).as_str() {
        "install" => current_action = current_pm.install,
        "remove" => current_action = current_pm.remove,
        "refresh" => current_action = current_pm.refresh,
        "update" => current_action = current_pm.update,
        "autoremove" => current_action = current_pm.autoremove,
        "search" => current_action = current_pm.search,

        // If no applicable command is given by the user, execute it as given by the user so the correct response
        // is given by package manager
        _ => current_action = String::from(args[1].clone())
        
    }


    // in KDE neon, pkcon has no option for autoremove, but apt does
    if current_action.eq("autoremove") && current_pm.command.eq("pkcon"){
        current_pm.command = String::from("apt");
    }

    // create command using distro's package manager
    let mut child = Command::new(current_pm.command);

    // set as first arguement the action to perform, i.e. install
    child.arg(current_action);
    
    // add the rest arguements to current terminal command
    for n in 2..args.len(){
        child.arg(args[n].clone());
    }

    // execute 
    child.spawn()?
         .wait()
         .expect("error");

    Ok(())
}

// parse /etc/os-release file and return the ID value
fn get_distro() ->  String{
    let mut return_value = String::from("not found");

    // if the /etc/os-release file is not found, notify the user 
    // that dpm was not able to detect his/her's distro
    let file = match File::open("/etc/os-release") {
        Err(_why) => return String::from("not found"),
        Ok(file) => file,
    };

    // get file content
    let file = BufReader::new(file);
    
    // search for the ID field, line by line until its found
    for line in file.lines().filter_map(|result| result.ok()) {
        // be sure to read ID and not VERSION_ID
        if line.contains("ID=") && !(line.contains("VERSION")) {
            // skip the "ID=" part of the string
            let value_index =  line.find('=').unwrap()+1;
            return_value = line[value_index..].to_string();
            break;
        }
    }

    return_value
}