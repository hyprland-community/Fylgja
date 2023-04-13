# 🔧 Fylgja
> *In Norse Mythology, [ˈfylɡjɑ],  is a supernatural being or spirit which accompanies a person in connection to their fate or fortune.*
> *Pronounced "FUL-yah".*


My personal swiss army knife for cross-system system management and multi-language project development. Boasting a collection of scripts and tools for managing and developing projects, this repository is a one-stop shop for all your system management and development needs.


## Concepts

### 1. Package Management

I intend for this project to be a wrapper around package managements such as `pacman`, `apt`, `nix` etc. This means this progam should install what I want, when I want, and how I want. Perhaps there will be an algorithm to determine which package manager to use with an override option.

Fylgja is meant to be a wrapper around many package managers that exist across a myriad of systems, such as `pacman`, `apt` or `nix`. The main idea is thatonce you initiate Fylgja to install a package, it should figure out your current package manager based on your distro (with an option to override) and install the package you want it to. One interface for everything.


### 2. System Management

The idea behind "system management" is to be able to back-up and restore my system. This might include system files, user files, and application files. This is a very broad concept, and I'm not sure how I want to implement it. What I am sure I will implement is a wrapper around git to easily manage dotfiles.

"System Management" is as vague as I could get I suppose. The concept I have in mind for syste management is to be able to back-up and restore systems using good ol' tar, or if the user so chooses: BTRFS snapshots. The backups should contain system files, user files and application files where applicable.

This is a very broad concept, and I am not *yet* sure how I want to implement it. What I am sure of, however, is that it will have a simple wrapper around git to easily manage my dotfiles non non-nixos distros.

### 3. Project Initialization

This should simply simply initialize a project with a template for the specified language and optionally enable git. This should be a wrapper around git and a template repository.

Nix integration is also a desirable goal.

### 4. TODO Management

This could be a wrapper around todo applications. I'm not sure how I want to implement this yet.


## To-do

- [x] initialize project
- [x] find a suitable name for the project
- [x] help and version commands
  - [ ] use help and version constants in the help menu from cargo file 
- [ ] project initialization
  - [x] initial command 
  - [x] git initialization in projects
  - [ ] language support (needs to be looked into)
    - [ ] go 
    - [ ] rust 
    - [ ] javascript 
    - [ ] typescript 
    - [ ] java 
    - [ ] ruby 
    - [ ] C/C++
- [ ] package management 
  - [ ] wrap basic package managers
  - [ ] search algoritm
  - [ ] override flag
  - [ ] flatpak support (?)


## Further concepts

While the initial concepts have the priority, I might create some "repositories" for things like system management scripts or project initialization templates for things such as Docker. These will be separate repositories, but will be linked to this repository.

## Contributing 

I don't know what exactly this project is meant to be, but I am open to suggestions, POC code, ideas and anything else. If you have any ideas, please open an issue or a pull request. I will try to respond as soon as possible.

If you want to insult my Rust knowledge, also go ahead. It's my first ever "project" in Rust.
