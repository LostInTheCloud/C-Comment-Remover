use std::{
    self, fs,
    fs::{DirEntry, File},
    io,
    io::prelude::*,
};

pub fn remove(file: &DirEntry)
{
    let mut output: Vec<char> = Vec::new();
    let filename: String = file.path().display().to_string();
    println!("removing comments from {}? (Y/n)", filename);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.contains("n")
    {
        return;
    }
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let arr: Vec<char> = contents.chars().collect();
    let mut i = 0;
    while i < arr.len()
    {
        if arr[i] == '/' && arr[i + 1] == '/'
        {
            i += 2;
            while arr[i] != '\n'
            {
                i += 1;
            }
        }
        if arr[i] == '/' && arr[i + 1] == '*'
        {
            i += 2;
            while arr[i] != '*' || arr[i + 1] != '/'
            {
                i += 1;
            }
            i += 2;
        }
        output.push(arr[i]);
        i += 1;
    }
    let mut file = File::create(file.path().display().to_string()).unwrap();
    let out = output.iter().collect::<String>();
    file.write_all(out.as_bytes()).unwrap();
}
