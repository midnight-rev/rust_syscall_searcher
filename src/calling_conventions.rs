use anyhow::Result;
use serde_json;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CallingConventionForArch {
    pub arch: String,
    pub nr: String,
    pub _return: String,
    pub arg0: String,
    pub arg1: String,
    pub arg2: String,
    pub arg3: String,
    pub arg4: String,
    pub arg5: String,
}

impl CallingConventionForArch {
    pub fn get_arch(&self) -> &str {
        &self.arch
    }

    pub fn get_calling_conventions_for_all_archs() -> Result<Vec::<CallingConventionForArch>>{
        const CALLING_CONVENTIONS_API_URL: &str = "https://api.syscall.sh/v1/conventions";
        let mut vector_of_conventions = Vec::<CallingConventionForArch>::new();
    
        let conventions_json = reqwest::blocking::get(CALLING_CONVENTIONS_API_URL)?;
        let conventions_json: Vec<serde_json::Value> = serde_json::from_str(&conventions_json.text()?)?;
    
        for item in conventions_json.iter() {
            let convention = CallingConventionForArch{
                arch: match item["arch"].as_str() {
                    Some(arch) => arch.to_owned(),
                    None => String::from(""),
                },

                nr: match item["nr"].as_str() {
                    Some(nr) => nr.to_owned(),
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
    
            vector_of_conventions.push(convention);
        }
        
        Ok(vector_of_conventions)
    }
}

