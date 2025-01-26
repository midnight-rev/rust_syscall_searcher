mod calling_conventions;
mod syscall;

use clap::Parser;
use syscall::{RsysearcherError, SyscallForArch};

/// search api.syscall.sh for informations about a specific syscall name or number
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// specify to use colors on terminal or not
    #[arg(short, long, default_value_t = false)]
    color: bool,

    /// architecture that you are working on
    #[arg(short, long)]
    arch: String,

    /// syscall number or name to search
    #[arg(short, long)]
    search: String,
}


fn main() -> Result<(), RsysearcherError> {
    let args = Args::parse();
    
    //let calling_conventions = CallingConventionForArch::get_calling_conventions_for_all_archs()?;
    //println!("{:#?}", calling_conventions);

    match SyscallForArch::search_syscall(&args.arch, &args.search, args.color){
        Ok(myresult) => {
            //println!("{:#?}", myresult);
            if myresult.is_empty() {
                eprintln!("No result found for {} on arch {}", &args.search, &args.arch);
            }
            for result in myresult.iter(){
                println!("{}\n----------------------------", result);
            }
        },
        Err(myerr) => {
            return Err(myerr);
        },
    }

    Ok(())
}