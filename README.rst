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

* Python_ 2.6, 2.7, or 3.2 (or newer 3.x).
* GnuPG_

Suggested packages:

* pwgen_: With the default configuration, if ``pwgen`` is installed,
  it's used to suggest good passwords to the user

.. _Python: http://www.python.org/
.. _GnuPG: http://www.gnupg.org/
.. _pwgen: http://sourceforge.net/projects/pwgen/


Development
===========

The test suite is in the ``test/`` directory. Use the ``run-tests.py``
script to run the test suite. For more info, invoke::

    python run-tests.py --help

cram_ 0.5 or newer is required to run the tests. If coverage_ is
installed, the script can also show test coverage.

There documentation in the ``doc/`` directory. To build the
documentation source to a regular manpage and a HTML manpage, invoke
::

    make -C doc

in the top directory. Docutils_ 0.7 or newer is required.

.. _cram: http://bitbucket.com/brodie/cram
.. _coverage: http://nedbatchelder.com/code/coverage/
.. _Docutils: http://docutils.sf.net/
