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

So I made this program in order to generate this file for me from the ```git status –-porcelain``` output.

## Problems

If I’ve added a file that is not needed in the commit file, I have to remove it every time.
Si I've made the implementation of '.commitignore' file that works EXACTLY as the '.gitignore' file.



## Usage

commit_text.txt

```
[123] TL;DR Fixes

- file1.tsx:
    Blabla

- file2.rs:
    Bablou
```

```bash
rust_binary_name [normal git commit arguments]
```

```
commit_message.txt found ✅
Commit Message:
------------------------------------------------
[123] TL;DR Fixes

- file1.tsx:
    Blabla

- file2.rs:
    Bablou
------------------------------------------------
Do you want to commit with this message? [Y/n]
>> n
Do you want to edit the commit message? [Y/n]
>> y # or return
```

If the user select yes to commit, it’ll just commit with the passed argument and the message in the ‘commit_text.txt’ file.

Else, the user is aked if he wants to edit the commit message. If yes is selected, the program will generate the following file.
```
[commit_nb]

- file_modified/added_1:
		
- file_modified/added_2:
	
...
```
