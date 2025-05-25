# Plex tool

This aims to be a little tool for doing little things that you might want to check when managing a plex lib. 
To start with these include: 
* Given a folder full of media files, crate folders that match the name of each file and then move the files into those folders.
* Given a directory, scan all of the files and make sure that none of them have non-utf8 names
* Say Hello.


# ideas
* would be nice to have more feedback while scanning large directory trees
* would be nice to have some sort of --check or --preview flag for items that modify the file tree


# Build
* linux: `cargo build --release`
* windows: `cargo build --release --target x86_64-pc-windows-gnu`



# Future
TUI with [ratatui](https://github.com/ratatui/ratatui)
