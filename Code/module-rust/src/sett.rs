use std::os::raw::c_char;
use std::ffi::CStr;

#[derive(Clone)]
pub struct Command {
    func: bool,
    name: String,
    value: String,
}

fn get_file(path: String) -> String {
    let v = reqwest::blocking::get(path);
    if v.is_err() {
        return "err".to_string();
    }
    let t = v.unwrap().text().unwrap();
    pars(t.clone());
    t
}

fn pars(code: String) -> Vec<Command> {
    let lines = code.split("\n");
    let mut commands = Vec::new();
    for line in lines {
        let args: Vec<_> = line.split(";").collect();
        if line.starts_with("//") {
            continue;
        }
        if args.len() != 3 {
            continue;
        }
        if args[0] != "do" && args[0] != "set" {
            continue;
        }
        commands.push(Command{func: args[0] == "do", name: args[1].to_string(), value: args[2].to_string()})
    }
    // let co = commands.clone();
    // for command in co {
    //     println!("{:?};{};{}", command.func, command.name, command.value);
    // }
    commands
}

pub fn compile(commands: Vec<Command>) -> String {
    let mut result = String::new();
    for command in commands {
        let f = {if command.func {"1"} else {"0"}};
        result.push_str(format!("{};{};{}\n", f, command.name, command.value).as_str());
    }
    result
}
/*
1;send;hello - do;send;hello
0;ip;127.0.0.1 - set;ip;127.0.0.1
https://raw.githubusercontent.com/Kolya142/my-prog-settings/main/test.sett
*/
pub fn Request(path: String) -> Vec<Command> {
    pars(get_file(path))
}