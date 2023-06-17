<div align="center">

![Icon](./icon.svg)

</div>

# loop

Loop execution of simple commands.

## 🏃 Usage

### Execute command indefinitely

If no options are specified, it runs indefinitely.

```console
$ loop echo Hello loop!
Hello loop!
Hello loop!
Hello loop!
Hello loop!
Hello loop!
     : (Continue until "Ctrl + C" is pressed)
```

### Execute command a specified number of times

The `--count` option can be used to specify the number of executions.

```console
$ loop --count 3 echo Hello loop!
Hello loop!
Hello loop!
Hello loop!
```

### Control the command execution interval

The `--interval` option specifies the pause time between the last command and the next command.

```console
$ loop --interval 0.5 echo Hello loop!
Hello loop!
Hello loop!
Hello loop!
Hello loop!
Hello loop!
     : (Continue until "Ctrl + C" is pressed)
```

## 💥 Installation

Install command.

```console
cargo install --git https://github.com/yutotnh/loop.git
```

### Set up completion

It generates completion scripts using [clap_complete](https://crates.io/crates/clap_complete).

#### Bash

```console
source <(loop --completion bash)
```

#### Zsh

```console
source <(loop --completion zsh)
```

#### Fish

```console
source (loop --completion fish | psub)
```

## 📖 Help

```console
Loop execution of simple commands

Usage: loop [OPTIONS] [COMMAND]...

Arguments:
  [COMMAND]...  Commands to execute

Options:
      --count <COUNT>       Number of times to greet [default: inf]
      --shell <SHELL>       Shell to run [default: sh]
      --interval <SECONDS>  Specify update interval [default: 0]
      --completion <SHELL>  Generate shell completion script for loop [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                Print help
  -V, --version             Print version
```
