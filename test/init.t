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

  $ cat .salakey | head -n 1
  -----BEGIN PGP MESSAGE-----

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

