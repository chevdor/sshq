# sshq

This command line utility allows querying your ssh config.

## Install

    cargo install sshq

## Usage

## Main

    sshq 0.1.2
    Chevdor <chevdor@gmail.com>
    `sshq` parses your ssh config and present the information back to you

    USAGE:
        sshq [OPTIONS] <SUBCOMMAND>

    OPTIONS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

    SUBCOMMANDS:
        help      Print this message or the help of the given subcommand(s)
        list      The `list` command returns the list of entries
        search    The `search` command searches for a given pattern

## List

    sshq-list 0.1.2
    Chevdor <chevdor@gmail.com>
    The `list` command returns the list of entries

    USAGE:
        sshq list [OPTIONS]

    OPTIONS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

## Search

    sshq-search 0.1.2
    Chevdor <chevdor@gmail.com>
    The `search` command searches for a given pattern

    USAGE:
        sshq search [OPTIONS] [PATTERN]

    ARGS:
        <PATTERN>    Search pattern

    OPTIONS:
        -h, --help       Print help information
        -j, --json       Output as json
        -V, --version    Print version information

## Use cases

You may find the following shell function convenient as it presents the list of available hosts and allow fuzzy search to connect to one.
With this function defined (in your `.bashrc` for instance), you may invoke `co` if you have no idea and then use fuyy search to find the right server, pass a fuzzy pattern such as `co srv123` (it will even allow typos…​), and it will connect directly if there is a single hit to your pattern.

    function co() {
        SEARCH=${@:-''};
         if [ $SEARCH ]; then
            hit=$(sshq list | fzf -i -1 --query "$SEARCH" --preview 'sshq search {}')
        else
            hit=$(sshq list | fzf -i -1 --preview 'sshq search {}')
        fi
        echo "Connecting to $hit... you may need to insert your Yubikey..."
        ssh $hit
    }
