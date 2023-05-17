# Rust Simple Shell
  
Rust Simple Shell is an extension of the Simple Shell project written entirely in the Rust language.

## Usage
* The project was developed on the remote ssh, so running it on windows may not work
  * Run `git clone https://github.com/kaiyuanlee3641/rust-simple-shell.git` on your unix remote ssh (elnux)
* Running the command `cargo run -j 1` in the root directory will start the program
  * `-j 1` is needed as remote ssh limits the amount of threads you're allowed to spawn
  * This argument restricts it to use only one thread

## Functionality
* Original capabilities of the Simple shell Project
* Pipe Commands
  * `|` will pipe one command's output into another command's input
  * For example, `ls | grep tree` will list all files in the current directory containing the string "tree"
* Redir Commands
  * `>` will write the output of the left command into a file
    * Used in the form `command > file`
    * For example, `ls > test.txt` will write a list of all files in the current directory into `test.txt`
    * `test.txt` is created if it doesn't exist
    * Existing contents inside `test.txt` is overridden
  * `>>` will append the output of the left command into a file
    * Used in the form `command >> file`
    * For example, `ls > test.txt` will append a list of all files in the current directory into `test.txt`
    * `test.txt` is created if it doesn't exist
    * Existing contents inside `test.txt` is kept, while new content is appended
  * `<` will read the file as the input into the left command
    * Used in the form `command >> file`
    * For example, `grep tree < hello.txt` will call the grep command with `hello.txt` as the input
    * This will list out all lines in `hello.txt` with the string "tree"
  * `<<` begins a "here" document
    * Used in the form `command << delimiter`
    * For example, `grep tree << EOF` will repeatedly read inputs until the "EOF" delimiter is read
    * The user is allowed to continue typing inputs until "EOF" is read
    * Then, the grep command is run on the collected input
  * More detailed reading of how redirections work: https://www.tutorialspoint.com/unix/unix-io-redirections.htm

## Design Choices
I chose to imitate most of the code flow and design of the original project. Instead of returning command tokens using pointers, I created a struct `SimpleShell` that stores the command itself. This is more OOP-based design. I also created an enum called `CommandType` that is used to differentiate between various commands. In the `parse_command` function, I split the string into separate commmands. Then, I stored the command in `cmd` based on the type of command (`CommandType::PipeCommand`, `CommandType::RedirCommand`, or `CommandType:Shellcommand`). The `exec_command` function executes the commands based on the enum type of `cmd`. 

The reason for only implementing pipes and redirections with two commands/inputs is because I did not want to force the Rust compiler to bend to undefined behaviors. The Rust compiler guarantees safety in return for difficulty in writing completely safe code. File descriptors are known in C to contain a lot of undefined and unexpected behaviors if handled improperly. Here, I piped using builtin capabilities, but it is possible to ignore the compiler by adding the `unsafe` keyword. There are probably ways to allow more than two commands, but for the scope of this project, I determined that two was enough in a simple shell. 

TLDR
* OOP-based design
* Struct `SimpleShell` and enum `CommandType`
* Adhered to Rust compiler leading to the limitation of only supporting up to two commands

## Limitations
* Pipe and Redir commands only support two commands
* Extra commands and arguments are ignored

## Links to presentation
If any of the video links don't work, download through google drive then play it on your device.

Slides: https://docs.google.com/presentation/d/1BcOyKpSTIOCMIoqFjgPujAXkVpzsQHd52goUDpSEw4c/edit?usp=sharing

Video: 
* mp4 - https://drive.google.com/file/d/1cV9sllw7Y3Uz_sjq6QEP_YRu58kQRmof/view?usp=sharing
* mkv - https://drive.google.com/file/d/17YoNO0-M4b36u_jT630PIfCW0WUt7CRy/view?usp=sharing
* streamable - https://streamable.com/iiz959

