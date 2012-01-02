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
  
  Copyright (C) 2011 Petri Lehtinen <petri@digip.org>
  
  sala is free software; you can redistribute it and/or modify it under
  the terms of the MIT license. See the file LICENSE distributed with
  the source code for details.
  
  The source code is available at http://pypi.python.org/pypi/sala.
