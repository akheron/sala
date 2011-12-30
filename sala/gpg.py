from __future__ import unicode_literals, print_function

from contextlib import contextmanager
import errno
import os
import subprocess
import sys
import tempfile


# For Python 2.6 compatibility
@contextmanager
def nested(cm1, cm2):
    with cm1:
        with cm2:
            yield


def close_fds(*args):
    for fds in args:
        for fd in fds:
            try:
                os.close(fd)
            except OSError as exc:
                if exc.errno != errno.EBADF:
                    raise


class GnuPG(object):
    '''GPG command line wrapper in the spirit of GnuPGInterface'''
    default_args = ['--batch', '--no-tty']

    def __init__(self, args=[]):
        self.args = args
        self.handles = {}

    def run(self, args=[], create_fhs=[], attach_fhs={}):
        if self.handles:
            raise RuntimeError('Already running')

        standard_handles = {
            'stdin': (sys.stdin, 'wb'),
            'stdout': (sys.stdout, 'rb'),
            'stderr': (sys.stderr, 'rb'),
        }

        gpg_handles = {
            'passphrase': 'wb',
        }

        self.handles = {}
        self.parent_fds = []
        child_fds = []
        fd_args = []
        std_fds = {}

        # If not given, attach standard streams
        for name, value in standard_handles.items():
            if name not in create_fhs and name not in attach_fhs:
                fobj, mode = value
                attach_fhs[name] = fobj

        for name in create_fhs:
            if name in standard_handles:
                _, mode = standard_handles[name]
            elif name in gpg_handles:
                mode = gpg_handles[name]
            else:
                close_fds(self.parent_fds, child_fds)
                raise ValueError('Invalid handle: %s' % name)

            pipe_r, pipe_w = os.pipe()
            if mode == 'wb':
                for_parent = pipe_w
                for_child = pipe_r
            elif mode == 'rb':
                for_parent = pipe_r
                for_child = pipe_w
            else:
                assert 0

            if name in standard_handles:
                std_fds[name] = for_child
            else:
                fd_args += ['--%s-fd' % name, str(for_child)]

            self.handles[name] = os.fdopen(for_parent, mode)
            self.parent_fds.append(for_parent)
            child_fds.append(for_child)

        for name, fobj in attach_fhs.items():
            if name in standard_handles:
                _, mode = standard_handles[name]
                target_fd = fobj.fileno()
            elif name in gpg_handles:
                mode = gpg_handles[name]
                target_fd = None
            else:
                close_fds(self.parent_fds, child_fds)
                raise ValueError('Invalid handle %s' % name)

            # Determine the file descriptor
            if hasattr(fobj, 'fileno') and callable(fobj.fileno):
                source_fd = fobj.fileno()
                self.handles[name] = fobj
            elif isinstance(fobj, int):
                source_fd = fobj
                self.handles[name] = os.fdopen(source_fd, mode)
            else:
                close_fds(self.parent_fds, child_fds)
                raise ValueError('No file descriptor to attach for %s' % name)

            if name in standard_handles:
                std_fds[name] = target_fd
            else:
                fd_args += ['--%s-fd' % name, str(source_fd)]

        def preexec():
            close_fds(self.parent_fds)

        cmdline = ['gpg'] + fd_args + self.default_args + self.args + args
        self.p = subprocess.Popen(cmdline, preexec_fn=preexec,
                                  close_fds=False, **std_fds)

        close_fds(child_fds)

    def wait(self):
        self.handles = {}
        close_fds(self.parent_fds)
        return self.p.wait()


def gpg_encrypt(config, filename, passphrase, content):
    stderr = tempfile.TemporaryFile()
    target = open(filename + '.tmp', 'w')

    with nested(stderr, target):
        gnupg = GnuPG([
            '--armor',
            '--cipher-algo', config.get('cipher'),
        ])
        gnupg.run(
            ['--symmetric'],
            create_fhs=['stdin', 'passphrase'],
            attach_fhs={
                'stdout': target,
                'stderr': stderr,
            })

        gnupg.handles['passphrase'].write(passphrase)
        gnupg.handles['passphrase'].close()

        gnupg.handles['stdin'].write(content)
        gnupg.handles['stdin'].close()

    try:
        gnupg.wait()
    except IOError as exc:
        print(exc, file=sys.stderr)
        os.remove(filename + '.tmp')
    else:
        os.rename(filename + '.tmp', filename)


def gpg_decrypt(filename, passphrase):
    stderr = tempfile.TemporaryFile()
    source = open(filename)

    with nested(stderr, source):
        gnupg = GnuPG(['--armor'])

        gnupg.run(
            ['--decrypt'],
            create_fhs=['stdout', 'passphrase'],
            attach_fhs={
                'stdin': source,
                'stderr': stderr,
            })

        gnupg.handles['passphrase'].write(passphrase)
        gnupg.handles['passphrase'].close()

        content = gnupg.handles['stdout'].read()
    try:
        gnupg.wait()
    except IOError:
        return b''
    else:
        return content
