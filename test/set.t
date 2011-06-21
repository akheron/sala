  $ . $TESTDIR/lib.sh

Invalid arguments:

  $ sala set
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

Simple secret creation:

  $ init_password_store testpassword
  $ sala set foo/@bar <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for foo/@bar: 
  Confirm: 
  
  $ decrypt_secret foo/@bar testpassword
  secret (no-eol)

  $ cleanup

Set many secrets with one command:

  $ init_password_store testpassword
  $ sala set foo/@bar foo/@baz bar/@quux <<EOF
  > testpassword
  > secret1
  > secret1
  > secret2
  > secret2
  > secret3
  > secret3
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for foo/@bar: 
  Confirm: 
  
  Type a new secret for foo/@baz: 
  Confirm: 
  
  Type a new secret for bar/@quux: 
  Confirm: 
  
  $ decrypt_secret foo/@bar testpassword
  secret1 (no-eol)
  $ decrypt_secret foo/@baz testpassword
  secret2 (no-eol)
  $ decrypt_secret bar/@quux testpassword
  secret3 (no-eol)

  $ cleanup

Set a secret in the top directory:

  $ init_password_store testpassword
  $ sala set foo <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for foo: 
  Confirm: 
  
  $ decrypt_secret foo testpassword
  secret (no-eol)

  $ cleanup

Set a secret, using SALADIR:

  $ mkdir store
  $ (cd store && init_password_store testpassword)
  $ SALADIR=store sala set foo <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for foo: 
  Confirm: 
  
  $ (cd store && decrypt_secret foo testpassword)
  secret (no-eol)

  $ test -f foo
  [1]

  $ cleanup

Set a secret, using -C:

  $ mkdir store
  $ (cd store && init_password_store testpassword)
  $ sala -C store set foo <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for foo: 
  Confirm: 
  
  $ (cd store && decrypt_secret foo testpassword)
  secret (no-eol)

  $ test -f foo
  [1]

  $ cleanup

Invalid master passphrase:

  $ init_password_store testpassword
  $ sala set foo/@bar <<EOF
  > wrongpassword
  > EOF
  Enter the master passphrase: 
  
  Error: Unable to unlock the encryption key
  [1]

  $ ! test -f foo/@bar
  $ cleanup

Secrets don't match:

  $ init_password_store testpassword
  $ sala set foo/@bar <<EOF
  > testpassword
  > secret
  > wrong
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for foo/@bar: 
  Confirm: 
  Inputs did not match
  [1]

  $ ! test -f foo/@bar
  $ cleanup

Select password suggestion from the list

  $ init_password_store testpassword
  $ write_config <<EOF
  > password-generator echo foo bar baz
  > EOF

  $ sala set foo/@bar <<EOF
  > testpassword
  > 1
  > EOF
  Enter the master passphrase: 
  
  0. foo
  1. bar
  2. baz
  
  Select a number from the list or type a new secret for foo/@bar: 
  
  $ decrypt_secret foo/@bar testpassword
  bar (no-eol)

  $ cleanup

Implicit set:

  $ init_password_store testpassword
  $ sala @bar <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for @bar: 
  Confirm: 
  
  $ decrypt_secret @bar testpassword
  secret (no-eol)

  $ cleanup

Implicit set with multiple files, one of which already exists:

  $ init_password_store testpassword
  $ touch @bar
  $ sala @foo @bar @baz <<EOF
  > testpassword
  > secret1
  > secret1
  > secret2
  > secret2
  > secret3
  > secret3
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for @foo: 
  Confirm: 
  
  Type a new secret for @bar: 
  Confirm: 
  
  Type a new secret for @baz: 
  Confirm: 
  
  $ decrypt_secret @foo testpassword
  secret1 (no-eol)
  $ decrypt_secret @bar testpassword
  secret2 (no-eol)
  $ decrypt_secret @baz testpassword
  secret3 (no-eol)

  $ cleanup

Implicit set with SALADIR:

  $ mkdir store
  $ (cd store && init_password_store testpassword)
  $ SALADIR=store sala @bar <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for @bar: 
  Confirm: 
  
  $ (cd store && decrypt_secret @bar testpassword)
  secret (no-eol)

  $ test -f @bar
  [1]

  $ cleanup

Implicit set with -C:

  $ mkdir store
  $ (cd store && init_password_store testpassword)
  $ sala -C store @bar <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for @bar: 
  Confirm: 
  
  $ (cd store && decrypt_secret @bar testpassword)
  secret (no-eol)

  $ test -f @bar
  [1]

  $ cleanup
