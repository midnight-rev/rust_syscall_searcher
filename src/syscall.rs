use serde_json;
use crate::calling_conventions::CallingConventionForArch;

#[derive(Debug)]
pub struct RsysearcherError(String);

impl std::fmt::Display for RsysearcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rsysearcher Error: {}", self.0)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SyscallForArch {
    color: bool,
    calling_convention: CallingConventionForArch,
    arch: String,
    nr: String,
    name: String,
    refs: String,
    _return: String,
    arg0: String,
    arg1: String,
    arg2: String,
    arg3: String,
    arg4: String,
    arg5: String,
}


impl SyscallForArch {

    /// returns a colorized string or not (based on the field color)
    fn colorize_or_not(&self, slice: &str) -> String {
        if self.color {
            format!("\x1b[92m{}\x1b[0m", slice)
        } else {
            slice.to_string()
        }
    }

    fn get_prototype(&self) -> String {
        let mut prototype: String = format!("{}(", &self.name);
        
        for arg in [&self.arg0, &self.arg1, &self.arg2, &self.arg3, &self.arg4, &self.arg5]{
            if !arg.is_empty(){
                prototype = format!("{}{}, ", prototype, &self.colorize_or_not(arg));
            }
        }

        prototype = format!("{}\x08\x08);", prototype);
        prototype
    }

    /// arg must be "arg0", "arg1", "arg2", "arg3", "arg4" or "arg5"
    fn convert_arg_to_register(&self, arg: &str) -> Result<&str, RsysearcherError> {
        match arg {
            "arg0" => Ok(&self.calling_convention.arg0),
            "arg1" => Ok(&self.calling_convention.arg1),
            "arg2" => Ok(&self.calling_convention.arg2),
            "arg3" => Ok(&self.calling_convention.arg3),
            "arg4" => Ok(&self.calling_convention.arg4),
            "arg5" => Ok(&self.calling_convention.arg5),
            _ => Err(RsysearcherError(format!("Arg {} not found", arg)))
        }
    }

    fn get_calling_convention_for(arch: &str, vector_of_calling_conventions: &Vec<CallingConventionForArch>) -> Result<CallingConventionForArch, RsysearcherError> {
        for calling_convention in vector_of_calling_conventions {
            if calling_convention.get_arch().eq(arch) {
                return Ok(calling_convention.to_owned());
            }
        }
        return Err(RsysearcherError(format!("Calling convention not found for arch {}", arch)));
    }

    pub fn search_syscall(arch: &str, syscall_name_or_number: &str, color: bool) -> Result<Vec<SyscallForArch>, RsysearcherError> {
        const SYSCALL_BASE_URL: &str = "https://api.syscall.sh/v1/syscalls/";
        let search_url: String = format!("{SYSCALL_BASE_URL}{syscall_name_or_number}");
    
        let mut vector_of_results = Vec::<SyscallForArch>::new();
        let vector_of_calling_conventions = CallingConventionForArch::get_calling_conventions_for_all_archs().unwrap();
    
        let response = reqwest::blocking::get(search_url).unwrap();
        let results_json: Vec<serde_json::Value> = serde_json::from_str(&response.text().unwrap()).unwrap();
    
        for item in results_json.iter(){
            //println!("arch = {} ::: item_arch = {} | is equal: {}", arch, item["arch"].as_str().unwrap(), arch == item["arch"]);

            if arch == item["arch"].as_str().unwrap() || arch == "all" {
                let calling_convention = match SyscallForArch::get_calling_convention_for(item["arch"].as_str().unwrap(), &vector_of_calling_conventions) {
                    Ok(callconv) => callconv,
                    Err(err) => panic!("FATAL ERROR: {}", err),
                };

                let syscall = SyscallForArch {
                    color,
                    calling_convention,
                    arch: item["arch"].as_str().unwrap().to_owned(),

                    nr: item["nr"].to_string(),

                    name: match item["name"].as_str() {
                        Some(name) => name.to_owned(),
                        None => String::from(""),
                    },
                    refs: match item["refs"].as_str() {
                        Some(refs) => refs.to_owned(),
                        None => String::from(""),
                    },
                    _return: match item["return"].as_str() {
                        Some(_return) => _return.to_owned(),
                        None => String::from(""),
                    },
                    arg0: match item["arg0"].as_str() {
                        Some(arg0) => arg0.to_owned(),
                        None => String::from(""),
                    },
                    arg1: match item["arg1"].as_str() {
                        Some(arg1) => arg1.to_owned(),
                        None => String::from(""),
                    },
                    arg2: match item["arg2"].as_str() {
                        Some(arg2) => arg2.to_owned(),
                        None => String::from(""),
                    },
                    arg3: match item["arg3"].as_str() {
                        Some(arg3) => arg3.to_owned(),
                        None => String::from(""),
                    },
                    arg4: match item["arg4"].as_str() {
                        Some(arg4) => arg4.to_owned(),
                        None => String::from(""),
                    },
                    arg5: match item["arg5"].as_str() {
                        Some(arg5) => arg5.to_owned(),
                        None => String::from(""),
                    },
                };
    
                vector_of_results.push(syscall);
            }
        }
    
    
        Ok(vector_of_results)
    }
    
}

impl std::fmt::Display for SyscallForArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nr_hex = match self.nr.parse::<u16>() {
            Ok(nr_hex) => nr_hex,
            Err(_) => panic!("Error parsing {} as hex", self.nr),
        };
        // First Section
        let mut fmt_string = format!(
"
Architecture: {}
Name: {}
Syscall Number: {} ( 0x{:x} )

{} <- {}
",
            &self.colorize_or_not(&self.arch),
            &self.colorize_or_not(&self.name),
            &self.colorize_or_not(&self.nr),
            nr_hex,
            &self.colorize_or_not(&self.calling_convention.nr),
            &self.colorize_or_not(&self.nr)
        );

        // TODO: improve

        if &self.arg0 != "" {
            fmt_string += format!("{} <- {}\n", &self.convert_arg_to_register("arg0",).unwrap(), &self.colorize_or_not(&self.arg0)).as_str();
        }
        if &self.arg1 != "" {
            fmt_string += format!("{} <- {}\n", &self.convert_arg_to_register("arg1").unwrap(), &self.colorize_or_not(&self.arg1)).as_str();
        }
        if &self.arg2 != "" {
            fmt_string += format!("{} <- {}\n", &self.convert_arg_to_register("arg2").unwrap(), &self.colorize_or_not(&self.arg2)).as_str();
        }
        if &self.arg3 != "" {
            fmt_string += format!("{} <- {}\n", &self.convert_arg_to_register("arg3").unwrap(), &self.colorize_or_not(&self.arg3)).as_str();
        }
        if &self.arg4 != "" {
            fmt_string += format!("{} <- {}\n", &self.convert_arg_to_register("arg4").unwrap(), &self.colorize_or_not(&self.arg4)).as_str();
        }
        if &self.arg5 != "" {
            fmt_string += format!("{} <- {}\n", &self.convert_arg_to_register("arg5").unwrap(), &self.colorize_or_not(&self.arg5)).as_str();
        }

        fmt_string += format!("
Prototype: {},
Return is on: {}
", &self.get_prototype(), &self.colorize_or_not(&self.calling_convention._return)
).as_str();

        write!(f, "{}", fmt_string)
    }
}

