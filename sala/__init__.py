from __future__ import unicode_literals, print_function

__version__ = '1.1'
__license__ = '''\
Copyright (C) 2011 Petri Lehtinen <petri@digip.org>

sala is free software; you can redistribute it and/or modify it under
the terms of the MIT license. See the file LICENSE distributed with
the source code for details.

The source code is available at http://pypi.python.org/pypi/sala.'''

import binascii
import errno
import getpass
import optparse
import os
import random
import subprocess
import sys

# For Python 2 compatibility
Py2 = sys.version_info[0] == 2

from sala.config import Configuration
from sala.gpg import gpg_encrypt, gpg_decrypt

if os.environ.get('SALA_TESTS_RUNNING'):
    # getpass() reads from TTY. Override this behavior in tests.
    def _simple_getpass(prompt=None):
        if prompt:
            print(prompt)
        return sys.stdin.readline().strip()
    getpass.getpass = _simple_getpass

    # To synchronize stdout and stderr output, use the same stream for
    # both
    sys.stderr = sys.stdout


def print_help():
    print('''\
Usage: sala [options] action [file...]

Store passwords and other sensitive information to plain text files.
The information is protected by GPG's symmetrical encryption.

Actions:
  init   Create a master key
  get    Read entries
  set    Create of modify entries

When using "set", files and directories are created automatically if
they don't already exist.

Options:
  -v, --version  Show version number and exit
  -h, --help     Show this help
  -r, --raw      Use a simple output format for machine processing''')
    sys.exit(2)


def print_version():
    print('sala version %s' % __version__)
    print('')
    print(__license__)
    sys.exit(0)


def ensure_files_exist(config, files):
    absent = []
    for filename in files:
        try:
            fobj = open(os.path.join(config.topdir, filename))
        except IOError:
            absent.append(filename)
        else:
            fobj.close()

    if len(absent) == 1:
        print('Error: File does not exist: %s' % absent[0], file=sys.stderr)
        return False

    elif absent:
        print('Error: The following files do not exist: %s' %
              ', '.join(absent), file=sys.stderr)
        return False

    return True


def read_passphrase(prompt, confirm=False, options=None):
    passphrase = getpass.getpass(prompt + ': ')
    if not passphrase:
        print('Empty passphrase is not allowed', file=sys.stderr)
        return False

    if options and passphrase in [str(x) for x in options]:
        return passphrase

    if confirm:
        other = getpass.getpass('Confirm: ')
        if other != passphrase:
            print('Inputs did not match', file=sys.stderr)
            return False

    return passphrase.encode('utf-8')


def make_parent_dirs(filename):
    dirname = os.path.dirname(filename)
    if dirname:
        try:
            os.makedirs(dirname)
        except OSError as exc:
            if exc.errno != errno.EEXIST:
                raise


def generate_passwords(cmd):
    p = subprocess.Popen(
        cmd,
        shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE)

    data = p.communicate()[0]

    if p.returncode != 0:
        return []
    else:
        return data.split()


def read_master_key(config):
    if not os.path.isfile(config.keyfile):
        print("Error: Run `sala init' first", file=sys.stderr)
        return False

    passphrase = read_passphrase('Enter the master passphrase')
    if not passphrase:
        return False

    key = gpg_decrypt(config.keyfile, passphrase)
    if not key:
        print('', file=sys.stderr)
        print('Error: Unable to unlock the encryption key', file=sys.stderr)
        return False

    return key


def do_init(config, files, options):
    if files:
        print_help()

    key_length = config.getint('key-length')

    if os.path.exists(config.keyfile):
        print('Error: The master key already exists', file=sys.stderr)
        return 1

    print('''\
Please pick a master passphrase. It is used to encrypt a very long
random key, which in turn is used to encrypt all the private data in
this directory.

Make sure you remember the master passphrase and that it's strong
enough for your privacy needs.
''')

    passphrase = read_passphrase('Enter the master passphrase', confirm=True)
    if passphrase is False:
        return 1

    print('')
    print('Generating a master key (%d bits)... ' % (key_length * 8), end='')

    rng = random.SystemRandom()
    key_bytes = (rng.randint(0, 255) for x in range(key_length))
    if Py2:
        # bytes is str in Python 2, there's no cleaner way to build a
        # bytestring from byte ints.
        data = b''.join(chr(x) for x in key_bytes)
    else:
        data = bytes(key_bytes)
    key = binascii.hexlify(data)

    gpg_encrypt(config, config.keyfile, passphrase, key)
    print('done')


def do_get(config, files, options):
    if not files:
        print_help()

    if not ensure_files_exist(config, files):
        return 1

    key = read_master_key(config)
    if key is False:
        return 1

    # Human-readable output
    def normal_output(filename, secret):
        print('%s: %s' % (filename, secret))
        print('')

    # Machine-readable output
    def raw_output(filename, secret):
        print(secret)

    if options.raw:
        output = raw_output
    else:
        print('')
        output = normal_output

    for filename in files:
        secret = gpg_decrypt(os.path.join(config.topdir, filename), key)
        if not secret:
            print('Error: Failed to decrypt %s\n' % filename, file=sys.stderr)
        else:
            output(filename, secret.decode('utf-8'))


def do_set(config, files, options):
    if not files:
        print_help()

    key = read_master_key(config)
    if key is False:
        return 1

    print('')

    for filename in files:
        pwlist = generate_passwords(config.get('password-generator'))
        if pwlist:
            options = range(len(pwlist))
            prompt = 'Select a number from the list ' + \
                'or type a new secret for ' + filename

            for i, pw in enumerate(pwlist):
                print('%d. %s' % (i, pw.decode('utf-8')))

            print('')
        else:
            options = None
            prompt = 'Type a new secret for ' + filename

        secret = read_passphrase(prompt, confirm=True, options=options)
        if secret is False:
            return 1

        if options:
            try:
                i = int(secret)
                secret = pwlist[i]
            except (ValueError, IndexError):
                pass

        make_parent_dirs(filename)
        gpg_encrypt(config, os.path.join(config.topdir, filename), key, secret)

        print('')


actions = {
    'init': do_init,
    'get': do_get,
    'set': do_set,
}


def main():
    global SALADIR, KEYFILE

    parser = optparse.OptionParser(
        usage='%prog action [file...]',
        add_help_option=False
        )
    parser.add_option('-h', '--help', action='store_true')
    parser.add_option('-v', '--version', action='store_true')
    parser.add_option('-C', '--saladir')
    parser.add_option('-r', '--raw', action='store_true')

    options, args = parser.parse_args()
    if options.version:
        print_version()

    if options.help or not args or len(args) < 1:
        print_help()

    if options.saladir:
        topdir = options.saladir
    else:
        topdir = os.environ.get('SALADIR', '')

    config = Configuration(topdir)

    action = args[0]
    files = args[1:]

    if action not in actions:
        files = [action] + files
        if os.path.exists(os.path.join(config.topdir, files[0])):
            do = actions['get']
        else:
            do = actions['set']
    else:
        do = actions[action]

    return do(config, files, options)
