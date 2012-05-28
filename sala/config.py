from __future__ import print_function

import os
import shutil
import sys

try:
    import configparser
except ImportError:
    # Python 2
    import ConfigParser as configparser


class Configuration(object):
    DEFAULTS = {
        'cipher': 'AES256',
        'key-length': 64,
        'password-generator': 'pwgen -nc 12 10',
    }

    def __init__(self, topdir):
        self.topdir = topdir
        self.saladir = os.path.join(topdir, '.sala')
        self.keyfile = os.path.join(self.saladir, 'key')
        self.hooksdir = os.path.join(self.saladir, 'hooks')

        # Backwards compatibility migrations
        self.compat('sala.conf', 'config')
        self.compat('.salakey', 'key')

        self.parser = configparser.RawConfigParser()

        self.parser.add_section('sala')
        for k, v in self.DEFAULTS.items():
            self.parser.set('sala', k, v)

        xdg_config_home = os.environ.get('XDG_CONFIG_HOME')
        if xdg_config_home is None:
            xdg_config_home = os.path.expanduser('~/.config')

        config_files = [
            os.path.expanduser('~/.sala.conf'),
            os.path.join(xdg_config_home, 'sala.conf'),
            os.path.join(topdir, '.sala', 'config'),
        ]

        self.parser.read(config_files)

    def __getattr__(self, key):
        # Proxies ConfigParser getters like this:
        #
        #   config.getint(x) -> config.parser.getint('sala', x)
        #

        if key not in ['get', 'getint', 'getfloat', 'getboolean']:
            raise AttributeError(key)

        return lambda x: getattr(self.parser, key)('sala', x)

    def compat(self, old, new):
        # Rename old to .sala/new if old exists and .sala/new does not

        old = os.path.join(self.topdir, old)
        new = os.path.join(self.saladir, new)

        if os.path.exists(old) and not os.path.exists(new):
            if not os.path.exists(self.saladir):
                print('Note: Creating directory %s' % self.saladir,
                      file=sys.stderr)
                os.mkdir(self.saladir)

            print('Note: Moving %s to %s' % (old, new))
            shutil.move(old, new)
