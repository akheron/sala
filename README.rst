sala -- Simple encrypted password storage
*****************************************

sala lets you store passwords and other bits of sensitive plain-text
information to encrypted files on a directory hierarchy. The
information is protected by GnuPG's symmetrical encryption.


Basic usage
===========

Create an empty directory for your password store, change into it, and
invoke::

    $ sala init

This command asks for the master passphrase you want to use for the
store. It then initializes the password store by creating a long
random key and encrypting it with the master passphrase.

Passwords are stored in a directory hierarchy, like this::

    /path/to/passwords
    |-- .salakey
    |-- site1
    |   |-- +webmail
    |   |   |-- @myuser
    |   |   `-- @otheruser
    |   `-- +adminpanel
    |       `-- @admin
    `-- my-linux-box
        |-- @myuser
        `-- @root

I use a convention of naming directories after services, prefixing
user names with ``@``, and groups, categories, etc. within a service
with ``+``. Of course, you can come up with your own scheme. The file
``.salakey`` contains the master key.

Create a new password for ``site2/@myuser``::

    $ sala set site2/@myuser

This command first asks you for the master passphrase, and then the
secret that should be stored to the file ``site2/@myuser``. The
intermediate directory ``site2`` is created automatically. After this
command, the directory hierarchy looks like this::

    /path/to/passwords
    |-- .salakey
    |-- site1
    |   |-- +webmail
    |   |   |-- @myuser
    |   |   `-- @otheruser
    |   `-- +adminpanel
    |       `-- @admin
    |-- site2
        `-- @myuser
    `-- my-linux-box
        |-- @myuser
        `-- @root

To read the secret you just stored, invoke::

    $ sala get site2/@myuser

This command asks again for the master passphrase, and outputs the
secret stored in ``mysite.com/@myuser``.

The commands can also be used on multiple files at once::

    sala set site3/@myuser site4/@otheruser
    sala get site3/@myuser site4/@otheruser

Under the hood, GnuPG's symmetric encryption is used. Only the the
master key file ``.salakey`` is encrypted with your master passphrase.
All the other files in the store are encrypted with the master key.
The master key is very long, truly random string. Each encrypted file
is in GnuPG's plain text (armor) format.


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

If pwgen_ is available, it is used to suggest passwords to the user.

.. _Python: http://www.python.org/
.. _GnuPG: http://www.gnupg.org/
.. _GnuPGInterface: http://py-gnupg.sourceforge.net/
.. _pwgen: http://sourceforge.net/projects/pwgen/
