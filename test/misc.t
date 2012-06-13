  $ . $TESTDIR/lib.sh

Help:

  $ sala -h
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
    -r, --raw      Use a simple output format for machine processing
  [2]

Version:

  $ sala -v
  sala version 1.2
  
  Copyright (C) 2011, 2012 Petri Lehtinen <petri@digip.org>
  
  sala is free software; you can redistribute it and/or modify it under
  the terms of the MIT license. See the file LICENSE distributed with
  the source code for details.
  
  The source code is available at https://github.com/akheron/sala.

Backwards compatibility: Check that .salakey is renamed to .sala/key:

  $ echo "foobar" > .salakey
  $ sala get a  # This triggers the moving, even though if fails
  NOTE: Creating directory .sala
  NOTE: Moving .salakey to .sala/key
  Error: File does not exist: a
  [1]
  $ ! test -e .salakey
  $ test -d .sala
  $ test "$(cat .sala/key)" = "foobar"

  $ cleanup

Backwards compatibility: Check that sala.conf is renamed to .sala/config:

  $ init_password_store testpassword
  $ rm -f .sala/config
  $ echo "[sala]" > sala.conf
  $ sala get a  # This triggers the moving, even though if fails
  NOTE: Moving sala.conf to .sala/config
  Error: File does not exist: a
  [1]
  $ ! test -e sala.conf
  $ test -d .sala
  $ test "$(cat .sala/config)" = "[sala]"

  $ cleanup
