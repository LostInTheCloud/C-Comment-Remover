use std::{
    env,
    fs::{self, DirEntry},
    io,
    path::Path,
};

mod codeonly_c;
mod codeonly_rs;
mod codeonly_py;

fn main()
{
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    visit_dirs(Path::new(&args[1]), &handle_file).unwrap();
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()>
{
    if dir.is_dir()
    {
        for entry in fs::read_dir(dir)?
        {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir()
            {
                visit_dirs(&path, cb)?;
            }
            else
            {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn handle_file(file: &DirEntry)
{
    let filename: String = file.path().display().to_string().split("/").last().unwrap().to_string();
    let parts: Vec<&str> = filename.split(".").collect();
    if parts.len() == 2
    {
        match *parts.last().unwrap()
        {
            "c" => interactive(file, &codeonly_c::remove),
            "rs" => interactive(file, &codeonly_rs::remove),
            "py" => interactive(file, &codeonly_py::remove),
            _ => return,
        }
    }
}

fn interactive(file: &DirEntry, fun: &dyn Fn(&DirEntry))
{
    let filename: String = file.path().display().to_string();
    println!("removing comments from {}? (Y/n)", filename);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if !input.contains("n")
    {
        fun(file);
    }
}
