# Git Commit With Rust.
## Description

I was tired to manually make proper commit messages so I made this Rust script to learn the basics of Rust AND solve my problem.

When I added files to git, I use to manually make a description of what I’ve done for eacch file. To make it readable I am using this structure

```
[commit_nb] TL;DR Brief description

- added_file_1

	blabla

- added_file_2

	blablou

```
(The new line after the file name is important for the markdown to work)

So I made this program in order to generate this file for me from the ```git status –-porcelain``` output.

## Problems

If I’ve added a file that is not needed in the commit file, I have to remove it every time.
Si I've made the implementation of '.commitignore' file that works EXACTLY as the '.gitignore' file.



## Usage

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
