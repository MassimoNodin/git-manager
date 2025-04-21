# git-manager

A simple command-line tool to manage multiple Git user configurations (user.name and user.email) easily. Switch between profiles like 'work' and 'personal' with a single command.

## Installation

1.  Go to the [Releases page](https://github.com/MassimoNodin/git-manager/releases).
2.  Download the latest binary for your operating system (Linux, macOS, or Windows).
3.  Rename the downloaded file to `git-manager` (or `git-manager.exe` on Windows).
4.  Make the file executable (on Linux/macOS): `chmod +x git-manager`.
5.  Place the executable in a directory that is part of your system's `PATH` (e.g., `/usr/local/bin` or `~/bin`).

## Usage

`git-manager` helps you manage different Git identities by storing them as profiles.

### Add a Profile

Store a new user name and email combination under a specific profile name.

```bash
git-manager add --profile <profile-name> --name "Your Name" --email "your.email@example.com"

# Example: Add a 'work' profile
git-manager add --profile work --name "Work Name" --email "work.email@company.com"

# Example: Add a 'personal' profile
git-manager add --profile personal --name "Personal Name" --email "personal.email@provider.com"
```

### List Profiles

View all the profiles you have saved. The currently active profile (used for global Git config) will be marked with an asterisk (`*`).

```bash
git-manager list
```

Example output:

```
Available git account profiles:
* work (Work Name <work.email@company.com>)
  personal (Personal Name <personal.email@provider.com>)
```

### Use a Profile

Switch the global Git `user.name` and `user.email` configuration to match the selected profile.

```bash
git-manager use <profile-name>

# Example: Switch to the 'personal' profile
git-manager use personal
```

This command will update your global Git configuration (`~/.gitconfig`).

## Status

This tool is currently in **alpha**. Basic functionality is implemented, but expect potential bugs or changes. Please report any issues you encounter on the [GitHub Issues page](https://github.com/MassimoNodin/git-manager/issues).

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
