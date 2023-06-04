# AI can save you from copying and pasting on terminal.

````shell
❯ cargo publish
    Updating [crates.io](http://crates.io) index
warning: manifest has no documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
error: 6 files in the working directory contain changes that were not yet committed into git:

demo.cast
header.txt
iris.csv
iris.data
iris-with-header.csv
iris-with-header-mean.csv

to proceed despite this and include the uncommitted changes, pass the `--allow-dirty` flag
❯ fill rm files listed above
I see that you want to remove the uncommitted changes in your working directory. You can use the `rm` command followed by the file names to remove them. Here's the command that you can use:

```sh
rm demo.cast header.txt iris.csv iris.data iris-with-header.csv iris-with-header-mean.csv
```

If you want to remove all the uncommitted changes and proceed with `cargo publish`, you can use the `--allow-dirty` flag with the `cargo publish` command. Here's the modified command:

```sh
cargo publish --allow-dirty
```

Please note that using `--allow-dirty` is not recommended unless you're absolutely sure that you want to publish with uncommitted changes in your working directory.
❯ rm demo.cast header.txt iris.csv iris.data iris-with-header.csv iris-with-header-mean.csv
````

# AI knows the options you may have never heard of.

````shell
❯ ask create ffmpeg command that rotates input movie 90degree counterclockwise
The `ffmpeg` command to rotate a video by 90 degrees counterclockwise is as follows:

```
ffmpeg -i input.mp4 -vf "transpose=2" output.mp4
```

In this command, replace `input.mp4` with the filename of your input video and `output.mp4` with the filename for your output video. The `-vf` option stands for "video filter," and the `transpose=2` filter rotates the video 90 degrees counterclockwise.
````

# AI can customize the answer to your environment.

Let's ask:

```shell
❯ ask write a function which list all directories and show as completion candidates
```

If you're using Zsh:
````shell
To list all directories and show them as completion candidates, you can define the following function in your shell configuration file (e.g. ~/.zshrc):
listdirs() {
    compadd -- $(find . -type d ! -name . -maxdepth 1 -print0 | xargs -0 -n1 basename)
}
This function uses the compadd command to add all the directories found by the find command as completion candidates. The xargs command is used to pass the list of directories as arguments to basename, which strips the leading directory paths from the names.
After adding this function definition to your shell configuration file, you can source the file or open a new shell to use this function. Then, you can type listdirs followed by a space, and press tab to see the list of directories as completion candidates.
````

If you're using Bash:
````shell
To write a function that lists all directories and shows them as completion candidates, you can add the following lines to your .bashrc or .bash_profile file:
function mydirs() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY=( $(compgen -d -- "${cur}") )
    return 0
}

complete -F mydirs -d ls
This will define a new function called mydirs which will use the compgen command to generate a list of directory names and set them as the completion candidates for the current command. The complete command at the end will tell bash to use this function for directory completion when the ls command is typed in. Now, when you type ls and tab for completion, you will only see directories as options.
````
