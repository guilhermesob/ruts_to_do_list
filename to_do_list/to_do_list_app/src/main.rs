extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Entry, Window, Inhibit}; // Importe Inhibit aqui

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("To-Do List App");
    window.set_default_size(350, 70);

    let entry = Entry::new();
    entry.set_hexpand(true);
    entry.set_vexpand(false);

    let button = Button::with_label("Add Task");
    button.connect_clicked(|_| {
        let task_text = entry.get_text();
        println!("Task added: {}", task_text);
        // Aqui você pode adicionar a lógica para adicionar a tarefa à sua lista
    });

    window.add(&entry);
    window.add(&button);

    window.show_all();

    window.connect_delete_event(|_, _| { // Certifique-se de que Inhibit está sendo usado corretamente aqui
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

#![allow(unused_attributes)]

use std::path::Path;

#[macro_use]
#[path = "build/macros.rs"]
pub mod macros;

#[path = "build/common.rs"]
pub mod common;
#[path = "build/dynamic.rs"]
pub mod dynamic;
#[path = "build/static.rs"]
pub mod r#static;

/// Copies a file.
#[cfg(feature = "runtime")]
fn copy(source: &str, destination: &Path) {
    use std::fs::File;
    use std::io::{Read, Write};

    let mut string = String::new();
    File::open(source)
        .unwrap()
        .read_to_string(&mut string)
        .unwrap();
    File::create(destination)
        .unwrap()
        .write_all(string.as_bytes())
        .unwrap();
}

/// Copies the code used to find and link to `libclang` shared libraries into
/// the build output directory so that it may be used when linking at runtime.
#[cfg(feature = "runtime")]
fn main() {
    use std::env;

    if cfg!(feature = "static") {
        panic!("`runtime` and `static` features can't be combined");
    }

    let out = env::var("OUT_DIR").unwrap();
    copy("build/macros.rs", &Path::new(&out).join("macros.rs"));
    copy("build/common.rs", &Path::new(&out).join("common.rs"));
    copy("build/dynamic.rs", &Path::new(&out).join("dynamic.rs"));
}

/// Finds and links to the required libraries dynamically or statically.
#[cfg(not(feature = "runtime"))]
fn main() {
    if cfg!(feature = "static") {
        r#static::link();
    } else {
        dynamic::link();
    }

    if let Some(output) = common::run_llvm_config(&["--includedir"]) {
        let directory = Path::new(output.trim_end());
        println!("cargo:include={}", directory.display());
    }
}