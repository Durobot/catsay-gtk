# catsay-gtk
A variation of catsay-gui project, as found in the book "Practical Rust Projects Building Game, Physical Computing, and Machine Learning Applications" by Shing Lyu.

The original version of this project (and other projects from the book) is at https://github.com/apress/practical-rust-projects

As **[gtk-rs](https://gtk-rs.org/)**, used by this project, in turn uses GTK library, which is a C library (OK, a collection of libraries),  you've got to have it installed before you can build the project. On Ubuntu (and, perhaps Mint and maybe Debian) it can be done with

`sudo apt install libgtk-3-dev` // Checking which version was installed: dpkg -l libgtk-3-0

