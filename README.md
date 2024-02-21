# Git Commands With Rust.

## Description

I was tired to manually make proper commit messages so I made this Rust script to learn the basics of Rust AND solve my problem.

When I added files to git, I use to manually make a description of what I’ve done for eacch file. To make it readable I am using this structure

I also wanted to make my life easier with switching branches and stashing changes.

```
[commit_nb] TL;DR Brief description

- added_file_1

	blabla

- added_file_2

	blablou

```

(The new line after the file name is important for the markdown to work)

So I made this program in order to generate this file for me from the `git status –-porcelain` output.

### Problems

If I’ve added a file that is not needed in the commit file, I have to remove it every time.
So I've made the implementation of '.commitignore' file that works EXACTLY as the '.gitignore' file.

## Usage

```bash
> rust_binary_name [OPTIONS] <COMMAND>
```

Where the only available option is `-v` or `--verbose` to get more information about what the program is doing.

## Commands

### `-g` or `--generate`

This command will generate the commit report file from the `git status --porcelain` output, without the files in the `.commitignore` file.

### `-c` or `--commit`

This command will commit the changes with the commit message in the commit file.
Can be combined with the [push](#-p-or---push) command.

### `-p` or `--push`

This command will push the changes to the remote repository.
We can also add arguments behind the command to push the changes with specific arguments.

### `s` or `--switch`

This command will ask you to select a branch to switch to.
It has its own options:

- `-s` or `--stash`
  This option will stash the changes before switching to the branch.

- `-a` or `--apply_stash`
  This option will apply the stash after switching to the branch.

### Generate the commit file

Let's say you have added 3 files to git.

```
> git status
M src/main.rs
M src/commit.rs
M src/utils.rs
```

You can generate the commit file with the command

```
> rust_binary_name -g # or --generate
```

This will generate this file

```
[commit_nb]

- src/main.rs



- src/commit.rs



- src/utils.rs


```

(The spaces are important for the markdown to work)

### Commit the changes with the commit file message

```
[28] TL;DR Brief description

- src/main.rs

  blabla

- src/commit.rs

  blablou


```

(Once again, the spaces are important for the markdown to work)

You can commit the changes with the following commands:

- Simple commit

  ```
  > rust_binary_name -c # or --commit
  ```

  This will commit the changes with the commit message in the commit file.

- Commit and push

  ```
  > rust_binary_name -cp
  ```

  This will commit the changes with the commit message in the commit file and push the changes to the remote repository.

- Commit and push with a push arguments

  ```
  > rust_binary_name -cpa --set-upstream origin main
  ```

  This will commit the changes with the commit message from the file, push the changes with the given arguments for push.

### Added all files with exceptions

You can add all files to the commit with specific exceptions with the following command:

```
> rust_binary_name -ae ./README.md ./src/main.rs
```

## `.commitignore` file.

This files works exactly as the `.gitignore` file.
If you want to ignore a file from the commit file, just add it to the `.commitignore` file. Works for directories too :)

## Installation

To install this program, follow these steps:

- Clone the repository
- Go to the repository folder
- Run the following command
  ```
  > cargo build --release
  ```
- Alias the binary to a command of your choice in your shell config file
  - For example, in my `.config/fish/config.fish` file I have this line
    ```
    alias commit="my_path_to_my_repo/target/release/git_commit_with_rust"
    ```
  - You can also add the binary to your path
    ```
    > export PATH=$PATH:~/Documents/Programming/Rust/git_commit_with_rust/target/release
    ```
