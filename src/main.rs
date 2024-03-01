#![allow(non_snake_case)]
use std::env;
use std::process;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    
    println!("Rendering Jagged Alliance 2 animations\n");
    
    //  Count amount of generated script files
    let currentDir = env::current_dir().unwrap();

    let mut extractPath = currentDir.clone();
    extractPath.push("renderGeneratedScripts");
    let dirExists: bool = extractPath.is_dir();
    if !dirExists
    {
        println!("Directory for generated scripts not found at: {}", extractPath.to_string_lossy());
        process::exit(2)
    }

    let mut count = 0;
    loop{
        let scriptFilename = config.animationFilename.replace(".py", &format!("{}.py", count));
        let mut pythonFile = currentDir.clone();
        pythonFile.push("renderGeneratedScripts");
        pythonFile.push(format!("{}", scriptFilename));

        let result = Path::new(&pythonFile).exists();
        match result {
            true => {count += 1; continue},
            false => break
        }
    }
    if count == 0
    {
        println!("No generated render scripts {} found at: {}", config.animationFilename, extractPath.to_string_lossy());
        process::exit(3);
    }


    // Create and check filepaths 
    let mut processes: Vec<process::Child> = Vec::new();
    let mut blendfileDir = currentDir.clone();
    blendfileDir.push(&config.blendFilename);
    if blendfileDir.exists() == false {
        println!("Blender file not found at: {}", blendfileDir.to_string_lossy());
        process::exit(4);
    }

    for i in 0..count
    {
        let scriptFilename = config.animationFilename.replace(".py", &format!("{}.py", i));
        let mut pythonFile = currentDir.clone();
        pythonFile.push("renderGeneratedScripts");
        pythonFile.push(format!("{}", scriptFilename));
        if pythonFile.exists() == false {
            println!("Python script not found at: {}", pythonFile.to_string_lossy());
            process::exit(5);
        }
    

        let blenderPath = format!("{}\\blender.exe", &config.blenderDir);
        if Path::new(&blenderPath).exists() == false {
            println!("Blender executable not found at: {}", &blenderPath);
            process::exit(6);
        }


        // Call blender
        let com = process::Command::new(blenderPath)
        .args(&[
            "-b", blendfileDir.to_str().unwrap(),
            "-P", pythonFile.to_str().unwrap()
            ])
            .stdout(process::Stdio::null())
            .spawn().unwrap();
        processes.push(com);
    }


    // Wait for the renders to finish
    for child in processes
    {
        // child.wait().expect("blender.exe wasn't running");
        let res = child.wait_with_output();
        match res
        {
            Ok(status) => 
            {
                println!("{:?}", status);
            }
            Err(e) => 
            {
                println!("Error for process: {}", e);
            }
        }
    }

    println!("Rendering complete!");
    process::exit(0)
}


struct Config {
    blenderDir: String,
    blendFilename: String,
    animationFilename: String
}
impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 4 {
            let errString = String::from("Not enough arguments!\nArguments must be path to Blender, blender filename and filename for python script\nEg. ") + &args[0] + ".exe \"C:\\Blender\\Blender 3.0\" \"JA2 2.9_033.blend\" \"batchrender-rifleAnims.py\"";
            return Err(errString);
        }

        let blenderDir = args[1].clone();
        let blendFilename = args[2].clone();
        let animationFilename = args[3].clone();

        Ok(Config {blenderDir, blendFilename, animationFilename})
    }
}
