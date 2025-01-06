use std::fs::write as write_to_file;
use std::fs::remove_file;

use std::process::ExitStatus;
use std::{
    process::Command,
};

pub(crate) fn write_plot_to_file(file_path: &str, plt_command: &str, title: Option<&str>, axes: Option<bool>, rge: Option<(f64, f64)>, path: Option<&str>) -> Result<ExitStatus> {
    let title_str = match title {Some(s) => s, None => "None"};
    let axes_str = match axes {Some(b) => if b {"True"} else {"False"}, None => "None"};
    let rge_str = match rge {Some(r) => &vec!["(", &r.0.to_string(), ", ", &r.1.to_string(), ")"].join(""), None => "None"};
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
    
    write_to_file(file_path, contents).expect("Failed to write to file.");
}

pub(crate) fn run_python_file(path: String) {
    let status = Command::new("python")
        .arg(path)
        .status();

    status.expect("Python file could not be run");
}

pub(crate) fn delete_file(path: String) {
    remove_file(path).expect("File could not be deleted");
}

