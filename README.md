# sshq

This command line utility allows querying your ssh config.

## Install

    cargo install sshq

## Usage

## Main

    `sshq` parses your ssh config and present the information back to you

    Usage: sshq [OPTIONS] <COMMAND>

    Commands:
      list    The `list` command returns the list of entries
      search  The `search` command searches for a given pattern
      help    Print this message or the help of the given subcommand(s)

    Options:
      -j, --json     Output as json
      -h, --help     Print help information
      -V, --version  Print version information

## List

    The `list` command returns the list of entries

    Usage: sshq list [OPTIONS]

    Options:
      -j, --json     Output as json
      -h, --help     Print help information
      -V, --version  Print version information

## Search

    The `search` command searches for a given pattern

    Usage: sshq search [OPTIONS] [PATTERN]

    Arguments:
      [PATTERN]  Search pattern

    Options:
      -j, --json     Output as json
      -h, --help     Print help information
      -V, --version  Print version information

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
