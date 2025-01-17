use std::fs::write as write_to_file;
use std::fs::remove_file;

use std::process::Command;

fn write_plot_to_file(file_path: &str, plt_command: &str, title: Option<&str>, axes: Option<bool>, rge: Option<((f64, f64), (f64, f64))>, path: Option<&str>) {
    let title_str = match title {Some(s) => &vec!["\"",  s, "\""].join(""), None => "None" };
    let axes_str = match axes {Some(b) => if b {"True"} else {"False"}, None => "None"};
    let rge_str = match rge {Some(r) => &vec!["((", &r.0.0.to_string(), ", ", &r.0.1.to_string(), "), (", &r.1.0.to_string(), ", ", &r.1.1.to_string(), "))"].join(""), None => "None"};
    let path_str = match path {Some(p) => p, None => "None"};

    let contents = format!("
from matplotlib import pyplot as plt

# Data replaced in file
title = {title}
axes = {axes}
rge = {rge}
path = {path}

# Plot, as determined by plot type in the rust file
plt.{plot_command}

# Automatic
if title is not None:
    plt.suptitle(title)

if axes is not None:
    if axes:
        pass
    else:
        plt.cla()

if rge is not None:
    plt.xlim(rge[0])
    plt.ylim(rge[1])

if path is not None:
    plt.savefig(path)
else:
    plt.show()",
    title=title_str, axes=axes_str, rge=rge_str, path=path_str, plot_command=plt_command);
    
    write_to_file(file_path, contents).expect("Could not write plot to file.")
}

fn run_python_file(file_path: String) {
    Command::new("python")
        .arg(file_path)
        .status()
        .expect("Python file could not be run");
}

fn delete_file(file_path: String) {
    remove_file(file_path)
        .expect("File could not be deleted");
}


// TODO: change to pub(crate)
pub fn python_plot(plt_command: &str, file_path: Option<&str>, title: Option<&str>, axes: Option<bool>, rge: Option<((f64, f64), (f64, f64))>, image_save_path: Option<&str>) {
    let file_path_str = file_path.unwrap_or("matplotlib_python_file.py");

    write_plot_to_file(file_path_str, plt_command, title, axes, rge, image_save_path);
    run_python_file(file_path_str.to_string());
    // delete_file(file_path_str.to_string());
}

