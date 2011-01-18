sala -- Simple encrypted password storage
*****************************************

sala lets you store passwords and other bits of sensitive plain-text
information to encrypted files on a directory hierarchy. The
information is protected by GnuPG's symmetrical encryption.

Copyright (C) 2011 Petri Lehtinen. sala is free software; you can
redistribute it and/or modify it under the terms of the MIT license.
See the file LICENSE distributed with the source code for details.

| Download: http://pypi.python.org/pypi/sala
| Source code: http://github.com/akheron/sala
| Author: Petri Lehtinen, http://www.digip.org

.. contents::


Basic usage
===========

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
``+``. Of course, you can come up with your own scheme, for example if
you want to hide the usernames, too.

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

The above commands can also be used on multiple files at once::

    sala set service2/@myuser service3/@otheruser
    sala get service2/@myuser service3/@otheruser

If no command is specified, sala assumes ``get`` if the first file
exists and ``set`` otherwise. That is, the command::

    sala foo/@bar

reads the secret ``foo/@bar`` if the file exists, and creates a new
secret otherwise. Note that this may not work as you expect for
multiple files, as the existence of the first file determines whether
to read or to write.


Configuration
=============

sala can be configured with an INI-style configuration file. sala
tries to read the configuration from ``~/.sala.conf`` and from
``sala.conf`` in the top directory of the password store. Neither of
the files are required. If a configuration setting is specified in
both files, the the latter takes precedence.

Here's the default configuration::

    # All configuration settings are in the [sala] section.
    [sala]

    # The cipher to use with GnuPG's symmetrical encryption.
    # Run "gpg --version" to list supported ciphers.
    cipher = AES256

    # Master key length, in bytes
    key-length = 64

    # A shell command to run to generate password suggestions
    password-generator = pwgen -nc 12 10

Changing ``cipher`` only affects secrets that are set after the
configuration setting is changed, i.e. the old secrets will not
automatically be re-encrypted.

Only ``sala init`` uses the ``key-length`` option. If you want the
master key to be of a different size, make sure the configuration file
exists before you run ``sala init``.

The ``password-generator`` option is run through the shell to generate
password suggestions. If the command fails (is not found or exits with
non-zero exit status), its output is ignored. Othewise, the output
should consist of one or more words separated with whitespace (space,
tab, newline, etc.). These words are presented to the user as password
suggestions by ``sala set``.


Under the hood
==============

sala uses GnuPG's symmetric encryption. All encrypted files are in the
GnuPG plain text (armor) format.

When the password store is initialized, a very long, truly random key
is generated and stored to the file ``.salakey``. Only this "master
key" is encrypted with your master passphrase. All the other files in
the store are encrypted with the master key.


Installation
============

Install sala by invoking::

    pip install sala

To install from source, invoke::

    python setup.py install

Requirements:

* Python_ 2.6 or newer. Currently, 3.x is not supported.
* GnuPG_
* GnuPGInterface_ for Python

Suggested packages:

* pwgen_: If found, used to suggest password to the user by default

.. _Python: http://www.python.org/
.. _GnuPG: http://www.gnupg.org/
.. _GnuPGInterface: http://py-gnupg.sourceforge.net/
.. _pwgen: http://sourceforge.net/projects/pwgen/
