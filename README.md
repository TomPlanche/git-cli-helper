# Git Commit With Rust.
## Description
A simple program to commit with a message from a txt ‘commit_text.txt’ file.

It can also generate the ‘commit_text.txt’ file from the current git situation

## Demo

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
Do you want to commit with this message? (y/n) default: y
>> n
Do you want to edit the commit message? (y/n) default: y
>> y # or return
```

If the user select yes to commit, it’ll just commit with the passed argument and the message in the ‘commit_text.txt’ file.

Else, the user is aked if he want’s to edit the commit message. If yes is selected, the program will generate the following git structure.
```
[commit_nb]

- file_modified/added_1:
		
- file_modified/added_2:
	
...
```
