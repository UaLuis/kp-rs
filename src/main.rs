use std::{fs, process::Command, env};
use clap::Parser;
use colored::*;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Options {
    /// Tools: meson/configure
    #[arg(short, long, required = true)]
    tool: String,

    /// URL to archive
    #[arg(short, long, required = true)]
    url: String,

    /// Add the options for meson/configure
    #[arg(short, long)]
    options: Option<String>,
}

fn downloadUnzipArchive(url: &str) {
    //let home_dir = env::var("HOME").expect("Не вдалося отримати домашній каталог");
    //println!("{} {}", "HOME DIR:".red(), home_dir);
    //let sources_dir = format!("{}/sources", home_dir);

    let dir_out = env::set_current_dir("/sources").expect("Не вдалося змінити каталог");    
    print!("{:?}", dir_out);
    
    let archive_name = "src.tar.gz";
    let wget_out = Command::new("wget").arg("-O").arg(archive_name).arg(url).output();
    let tar_out = Command::new("tar").arg("-xf").arg(archive_name).output();
    let _ = fs::remove_file(&archive_name);

    println!("{} {:?}\n", "LOG OF WGET:".red(), wget_out);
    println!("{} {:?}\n", "LOG OF TAR:".red(), tar_out);
}

fn build(type_tool: &String, options: &Option<String>) {
    for entry in fs::read_dir(".").expect("failed to read directory") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let _ = env::set_current_dir(&path.to_str().unwrap());
            fs::create_dir_all("build").unwrap();
            let _ = env::set_current_dir("build");
            
            if type_tool == "meson" {
                
                let mut meson_command = Command::new("meson");
                meson_command.arg("setup").arg("--prefix=/usr");
                
                if let Some(opts) = options {
                    meson_command.arg(opts);
                }
                
                let meson_out = meson_command.output().expect("failed to execute process");
                let ninja_build = Command::new("ninja").output().expect("failed to execute process");
                let ninja_install = Command::new("ninja").arg("install").output().expect("failed to execute process");
                
                println!("{} {:?}\n", "LOG OF Meson:".red(), meson_out);
                println!("{} {:?}\n", "LOG OF Ninja:".red(), ninja_build);
                println!("{} {:?}\n", "LOG OF Ninja Install:".red(), ninja_install);
                
            } else if type_tool == "configure" {
                
                let mut configure_command = Command::new("./configure");
                configure_command.arg("--prefix=/usr");
                
                if let Some(opts) = options {
                    configure_command.arg(opts);
                }
                
                let configure_out = configure_command.output().expect("failed to execute process");
                let make_build = Command::new("make").arg("-j$(nproc)").output().expect("failed to execute process");
                let make_install = Command::new("make").arg("install").output().expect("failed to execute process");
                

                println!("{} {:?}\n", "LOG OF Configure:".red(), configure_out);
                println!("{} {:?}\n", "LOG OF Make:".red(), make_build);
                println!("{} {:?}\n", "LOG OF Make Install:".red(), make_install);
                
            }
        }
    }
}
fn main() {
    let options = Options::parse();
    
    downloadUnzipArchive(&options.url);
    build(&options.tool, &options.options);
        
    
    

}