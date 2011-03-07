sala - encrypted plaintext password store
*****************************************

Sala lets you store passwords and other bits of sensitive plain-text
information to encrypted files on a directory hierarchy. The
information is protected by GnuPG's symmetrical encryption.

Basic usage
===========

Passwords are stored in a directory hierarchy, each file containing
one secret.

Commands:

``sala init``
    Initialize a password store

``sala get FILE...``
    Read secret(s)

``sala set FILE...``
    Create or modify secret(s)

``sala FILE...``
    Read or modify, depending on whether the first file exists or not

For more information, see sala(1) and http://www.digip.org/sala/.


Installation
============

Install sala by invoking::

    pip install sala

To install from source, invoke::

    python setup.py install

Requirements:

* Python_ 2.5 or newer. Currently, 3.x is not supported.
* GnuPG_

Suggested packages:

* pwgen_: With the default configuration, if ``pwgen`` is installed,
  it's used to suggest good passwords to the user

.. _Python: http://www.python.org/
.. _GnuPG: http://www.gnupg.org/
.. _pwgen: http://sourceforge.net/projects/pwgen/
