import contextlib
import os
import tempfile

import GnuPGInterface

def gpg_encrypt(config, filename, passphrase, content):
    logger = tempfile.TemporaryFile()
    stderr = tempfile.TemporaryFile()
    target = open(filename + '.tmp', 'w')

    with contextlib.nested(logger, stderr, target):
        gnupg = GnuPGInterface.GnuPG()
        gnupg.options.armor = 1
        gnupg.options.meta_interactive = 0
        gnupg.options.extra_args = ['--cipher-algo', config.get('cipher')]

        p = gnupg.run(
            ['--symmetric'],
            create_fhs=['stdin', 'passphrase'],
            attach_fhs={
                'stdout': target,
                'stderr': stderr,
                'logger': logger,
            })

        p.handles['passphrase'].write(passphrase)
        p.handles['passphrase'].close()

        p.handles['stdin'].write(content)
        p.handles['stdin'].close()

    try:
        p.wait()
    except IOError, exc:
        print >>sys.stderr, exc
        os.remove(filename + '.tmp')
    else:
        os.rename(filename + '.tmp', filename)


def gpg_decrypt(filename, passphrase):
    logger = tempfile.TemporaryFile()
    stderr = tempfile.TemporaryFile()
    source = open(filename)

    with contextlib.nested(logger, stderr, source):
        gnupg = GnuPGInterface.GnuPG()
        gnupg.options.armor = 1
        gnupg.options.meta_interactive = 0

        p = gnupg.run(
            ['--decrypt'],
            create_fhs=['stdout', 'passphrase'],
            attach_fhs={
                'stdin': source,
                'stderr': stderr,
                'logger': logger,
            })

        p.handles['passphrase'].write(passphrase)
        p.handles['passphrase'].close()

        content = p.handles['stdout'].read()
        p.handles['stdout'].close()

    try:
        p.wait()
    except IOError:
        return ''
    else:
        return content
