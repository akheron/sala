sala - encrypted plaintext password store
*****************************************

Sala lets you store passwords and other bits of sensitive plain-text
information to encrypted files on a directory hierarchy. The
information is protected by GnuPG's symmetrical encryption.

Usage
=====

Passwords are stored in a directory hierarchy, each file containing
one secret.

Commands:

``sala init``
    Initialize a password store

``sala get FILE``
    Read a secret

``sala set FILE``
    Create or modify a secret

``sala FILE``
    Read or modify, depending on whether the file exists or not

Options:

-v, --version     Show version information
-h, --help        Show help
-C DIR            Use a password store in DIR instead of current directory
-r, --raw         Use a simple output format for machine processing

If the ``SALADIR`` environment variable is set, use a password store
in this directory instead of the current directory.


Tutorial
========

Passwords are stored in a directory hierarchy, each file containing
one secret, like this::

    /path/to/passwords
    |-- example-service.com
    |   |-- +webmail
    |   |   |-- @myuser
    |   |   `-- @otheruser
    |   `-- +adminpanel
    |       `-- @admin
    `-- my-linux-box
        |-- @myuser
        `-- @root

I use a convention of naming directories after services and using
``@username`` as the file name. If a service has groups, categories,
subservices, etc., I use subdirectories whose names are prefixed with
``+``. This naming scheme is not enforced by sala, and you can come up
with your own scheme, for example if you want to hide the usernames,
too.

To create a new password store, first create an empty directory,
change into it, and invoke::

    $ sala init

This command asks for the master passphrase you want to use for the
store. It then initializes the password store by creating a long
random key and encrypting it with the master passphrase.

Create a new password for ``service/@myuser``::

    $ sala set service/@myuser

This command first asks you for the master passphrase, and then the
secret that should be stored to the file ``service/@myuser``. The
intermediate directory ``service`` is created automatically.

To read the secret you just stored, invoke::

    $ sala get service/@myuser

This command asks again for the master passphrase, and outputs the
secret.

All the files are just normal files, so you can safely remove or
rename files if you want to.

If no command is specified, sala assumes ``get`` if the file exists
and ``set`` otherwise. That is, the command::

    $ sala foo/@bar

reads the secret ``foo/@bar`` if the file exists, and creates a new
secret otherwise.


Configuration
=============

Sala can be configured with a TOML configuration file. Sala tries to
read its configuration files in this order:

* ``~/.sala.toml``

* ``~/.config/sala.toml`` (more specifically
  ``$XDG_CONFIG_HOME/sala.toml``)

* ``.sala/config`` in the top directory of the password store

None of the files are required. If a configuration setting is
specified in more than one file, the latter file (in the list above)
takes precedence.

Here's the default configuration::

    # The cipher to use with GnuPG's symmetrical encryption.
    # Run "gpg --version" to list supported ciphers.
    cipher = "AES128"

    # Master key length, in bytes
    key-length = 64

Changing ``cipher`` only affects secrets that are set after the
configuration setting is changed. Old secrets will not automatically
be re-encrypted.

Only ``sala init`` uses the ``key-length`` option. If you want the
master key to be of a different size, make sure the configuration file
exists before you run ``sala init``.

The ``password-generator`` command is run to generate password
suggestions. If the command fails (is not found or exits with non-zero
exit status), its output is ignored. Othewise, the output should
consist of one or more words separated by whitespace (space, tab,
newline, etc.). These words are presented to the user as password
suggestions by ``sala set``. For example, the following line in the
config will use ``pwgen`` to generate a list of 10 password
suggestions, 16 charaters each::

    password-generator = "pwgen -nc 16 10"


Bash completion
===============

A bash completion script is available in
``contrib/sala-completion.bash``. When enabled, it provides tab
completion for files and directories in ``$SALADIR``, or in the
current directory if ``SALADIR`` has not been defined. Setting
``SALADIR`` allows you to use sala with tab completion regardless of
the current working directory of your shell.

To enable bash completion, load the completion script::

    $ export SALADIR=/path/to/passwords
    $ . /path/to/sala/contrib/bash-completion.sala

If you want to later disable the completion in the same shell session,
invoke::

    $ complete -o default sala


Under the hood
==============

Sala uses GnuPG's symmetric encryption. All encrypted files are in the
GnuPG plain text (armor) format.

When the password store is initialized, a very long, truly random key
is generated and stored to the file ``.sala/key``. Only this "master
key" is encrypted with your master passphrase. All the other files in
the store are encrypted with the master key.


Building and developing
=======================

Sala is written in Rust, so you'll need to have Rust installed.

Run the following commands to build sala::

    $ git clone https://github.com/akheron/sala
    $ cd sala
    $ cargo build --release

The result is a single binary ``./target/release/sala``.

Run the test suite::

    $ cargo test --all
