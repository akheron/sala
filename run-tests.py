#!/usr/bin/env python

from optparse import OptionParser
from subprocess import Popen
import os
import sys


def run_command(cmdline):
    proc = Popen(cmdline, shell=True)
    proc.communicate()
    return proc.returncode


def main():
    parser = OptionParser()
    parser.add_option(
        '-c', '--coverage',
        action='store_true',
        help='Measure code coverage')

    options, args = parser.parse_args()

    if args:
        parser.print_help()
        return 2

    if run_command('which cram >/dev/null') != 0:
        print >>sys.stderr, 'Error: cram is not installed'
        return 1

    if options.coverage:
        if run_command('which coverage >/dev/null') != 0:
            print >>sys.stderr, 'Error: coverage is not installed'
            return 1

    if options.coverage:
        run_command('coverage erase')
        os.environ['COVERAGE'] = 'yes'
        os.environ['COVERAGE_FILE'] = os.path.abspath('.coverage')

    run_command('cram test')

    if options.coverage:
        run_command('coverage report -m')


if __name__ == '__main__':
    sys.exit(main() or 0)
