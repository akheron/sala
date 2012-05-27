  $ . $TESTDIR/lib.sh

Invalid arguments:

  $ sala init foo
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

Initialize a password store:

  $ sala init <<EOF
  > testpassword
  > testpassword
  > EOF
  Please pick a master passphrase. It is used to encrypt a very long
  random key, which in turn is used to encrypt all the private data in
  this directory.
  
  Make sure you remember the master passphrase and that it's strong
  enough for your privacy needs.
  
  Enter the master passphrase: 
  Confirm: 
  
  Generating a master key (512 bits)... done

  $ cat .sala/key | head -n 1
  -----BEGIN PGP MESSAGE-----

  $ test -d .sala/hooks

  $ test -f .sala/hooks/post-set.sample

  $ cat .sala/hooks/post-set.sample
  #!/bin/sh
  
  # This is a sample post-set hook for sala that commits your changes
  # to git. To activate, remove .sample and make the file executable.
  
  # post-set receives the filename as a parameter.
  
  # git add $1 && git commit -m "Save $1."


Initialize with an empty password:

  $ cleanup
  $ sala init <<EOF
  > 
  > EOF
  Please pick a master passphrase. It is used to encrypt a very long
  random key, which in turn is used to encrypt all the private data in
  this directory.
  
  Make sure you remember the master passphrase and that it's strong
  enough for your privacy needs.
  
  Enter the master passphrase: 
  Empty passphrase is not allowed
  [1]

Initialize with mismatching passwords:

  $ cleanup
  $ sala init <<EOF
  > testpassword
  > wrong
  > EOF
  Please pick a master passphrase. It is used to encrypt a very long
  random key, which in turn is used to encrypt all the private data in
  this directory.
  
  Make sure you remember the master passphrase and that it's strong
  enough for your privacy needs.
  
  Enter the master passphrase: 
  Confirm: 
  Inputs did not match
  [1]

Initialize twice:

  $ cleanup
  $ sala init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF

  $ sala init
  Error: The master key already exists
  [1]

  $ cleanup

Initialize a store using SALADIR:

  $ mkdir store
  $ SALADIR=store sala init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF

  $ cat store/.sala/key | head -n 1
  -----BEGIN PGP MESSAGE-----

  $ test -f .sala/key
  [1]

  $ cleanup

Initialize a store using -C:

  $ mkdir store
  $ sala -C store init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF

  $ cat store/.sala/key | head -n 1
  -----BEGIN PGP MESSAGE-----

  $ test -f .sala/key
  [1]
