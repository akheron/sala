from __future__ import unicode_literals, print_function

import os
import stat
import subprocess

_DEFAULT_HOOKS = {
    'post-set': '''\
#!/bin/sh
#
# An example post-set hook that commits your changes to git.
# To enable this hook, rename this file to "post-set".
#
# Receives the filename as the first argument.

git add $1 && git commit -m "Save $1."
''',
}


def init_hooks(hooksdir):
    for hook, value in _DEFAULT_HOOKS.items():
        hookfile = os.path.join(hooksdir, '{0}.sample'.format(hook))
        with open(hookfile, 'w') as f:
            f.write(value)
        mode = os.stat(hookfile)[0]
        os.chmod(hookfile, mode | stat.S_IXUSR)


def run_hooks(state, action, config, *args, **kwargs):
    hook_name = '{0}-{1}'.format(state, action)

    if hook_name not in _hook_actions:
        return

    # We need absolute path here as we're going to set the CWD to
    # `config.topdir`
    executable = os.path.join(os.path.abspath(config.hooksdir), hook_name)

    if not os.path.isfile(executable):
        return

    if not os.access(executable, os.X_OK):
        return

    return _hook_actions[hook_name](executable, config, *args, **kwargs)


def _run_hook(config, hook, *params):
    args = [hook] + list(params)

    # Overwrite SALADIR to provide correct SALADIR to the executable
    env = os.environ.copy()
    env['SALADIR'] = os.path.abspath(config.topdir)

    return subprocess.Popen(args, env=env, cwd=config.topdir or None)


def post_set(hook, config, filename):
    process = _run_hook(config, hook, filename)
    return process.wait() == 0


_hook_actions = {
    'post-set': post_set,
}
