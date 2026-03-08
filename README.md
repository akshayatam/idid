# idid (Short for "I Did") 

`idid` is a tiny CLI utility written in Rust that logs the software I install and remove on my Linux system. 

As I use and daily Arch (btw), I frequently experiment with different tools, terminals, editors, and utilities. Over time, I realized that I had no reliable way to track: 

- What I installed
- When I installed it
- How I installed it (pacman, yay, Flatpak, source, etc.)
- Why I removed something later

`idid` solves that problem by acting as a **personal software activity journal**.

Instead of relying on fragmented system logs, I can simply run a command after installing or removing something and maintain a clean, readable history.

---

## Example Usage

```bash
idid install wezterm flatpak
idid install neovim pacman
idid install catppuccin-theme source
idid remove ghostty
```

This produces a simple log file: 

```csv
timestamp,action,package,source
2026-03-07 19:26:29,install,wezterm,flatpak
2026-03-07 19:26:41,install,neovim,pacman
2026-03-07 19:27:12,install,ghostty,source
2026-03-07 19:27:21,remove,ghostty,Unknown
```

You can also view the history using the `idid history` command:

```bash
idid history
```

## Why I build this 

As someone who experiments heavily with linux tools, I often find myself installing software in multiple ways: 

- Pacman packages
- Flatpak apps
- Source code builds
- Custom themes

System logs do exist for some but those are scattered across different files and formats and are generally not designed for human tracking.

`idid` provides a unified log that tracks all of these activities, making it easy to see what I've installed and when.

---

## Project Goals 

My goal is not to build another package manager. Instead, the goal is to provide a simple, personal activity journal that tracks what I install and remove on my system. 

The design principles are: 

- Extremely lightweight 
- Fast CLI 
- Human-readable output
- Minimal dependencies 
- Easy to extend 

I chose Rust because it produces a fast, portable single binary and is well suited for building command-line tools. Also, I am learning Rust and with a mainly ML background, I wanted to learn a systems programming language and build something practical that I would use. 

---

## Installation

Clone the repository:
```bash
# Clone the repository
git clone https://github.com/akshayatam/idid.git 
cd idid
```

Build the binary:
```bash
cargo build --release
```

Install the binary (optional):
```bash
sudo cp target/release/idid /usr/local/bin/
```

OR 

Move it somewhere in your PATH:
```bash
mv target/release/idid /path/to/bin/
```

Now you can run: 
```bash
idid install wezterm flatpak
```

---

## Future Improvements 

The project intentionally starts simple as I am still learning Rust and building this would make me more comfortable with the language (specifically the concept of ownership). 

Some of the features I plan to add in the future include: 

- Better history view 
- Adding notes for more context (e.g. why I installed something) 
- Active package view to see what's currently installed 
- Filtering and searching through installed packages 
- XDG-compliant storage for logs instead of only inside the repository 
- Export options for history and logs 

--- 

## License 

MIT License
