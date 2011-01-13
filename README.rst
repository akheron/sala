***************
README for sala
***************

sala lets you store passwords and other sensitive information to plain
text files. The information is protected by GPG's symmetrical
encryption.


Requirements
============

* Python_
* GnuPG_
* GnuPGInterface_ for Python

If pwgen_ is available, it can be used to generate passwords.

.. _Python: http://www.python.org/
.. _GnuPG: http://www.gnupg.org/
.. _GnuPGInterface: http://py-gnupg.sourceforge.net/
.. _pwgen: http://sourceforge.net/projects/pwgen/


Installation
============

::

    pip install sala

or

::

    python setup.py install


Basic usage
===========

I use a convention of prefixing user names with ``@`` and groups,
services, etc. with ``+``. Passwords are stored in a directory
hierarchy with directories denoting the service, etc.

Create or change a password::

    sala set mysite.com/@username

The password is stored to the file ``mysite.com/@username``. The
``mysite.com`` directory is created automatically if it doesn't exist.

Read a password::

    sala get mysite.com/@username
