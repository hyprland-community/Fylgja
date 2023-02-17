# 🔧 Sysdev

My personal swiss army knife for cross-system system management and multi-language project development. Boasting a collection of scripts and tools for managing and developing projects, this repository is a one-stop shop for all your system management and development needs.

*very creative name, I know*

## Concepts

### Package Management

I intend for this project to be a wrapper around package managements such as `pacman`, `apt`, `nix` etc. This means this progam should install what I want, when I want, and how I want. Perhaps there will be an algorithm to determine which package manager to use with an override option.

### System Management

The idea behind "system management" is to be able to back-up and restore my system. This might include system files, user files, and application files. This is a very broad concept, and I'm not sure how I want to implement it. What I am sure I will implement is a wrapper around git to easily manage dotfiles.

### Project Initialization

This should simply simply initialize a project with a template for the specified language and optionally enable git. This should be a wrapper around git and a template repository.

### TODO Management

This could be a wrapper around todo applications. I'm not sure how I want to implement this yet.

## Further concepts

While the initial concepts have the priority, I might create some "repositories" for things like system management scripts or project initialization templates for things such as Docker. These will be separate repositories, but will be linked to this repository.
