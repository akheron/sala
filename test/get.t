  $ . $TESTDIR/lib.sh

Invalid arguments:

  $ sala get
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

No password store initialized:

  $ touch @bar
  $ sala get @bar
  Error: Run `sala init' first
  [1]

  $ cleanup

File does not exist:

  $ init_password_store testpassword
  $ sala get foo/@bar
  Error: File does not exist: foo/@bar
  [1]

Multiple files don't exist:

  $ sala get foo/@bar foo/@baz
  Error: The following files do not exist: foo/@bar, foo/@baz
  [1]

  $ cleanup

Decrypt a secret that was encrypted with a different key:

  $ init_password_store testpassword
  $ gpg_encrypt @bar unknown_key secret
  $ sala get @bar <<EOF
  > testpassword
  > EOF
  Enter the master passphrase: 
  
  Error: Failed to decrypt @bar
  
  $ cleanup

Decrypt a secret:

  $ init_password_store testpassword
  $ encrypt_secret @bar testpassword secret
  $ sala get @bar <<EOF
  > testpassword
  > EOF
  Enter the master passphrase: 
  
  @bar: secret
  
  $ cleanup

Decrypt a secret, with raw output format:

  $ init_password_store testpassword
  $ encrypt_secret @bar testpassword secret
  $ sala get -r @bar <<EOF
  > testpassword
  > EOF
  Enter the master passphrase: 
  secret
  $ cleanup

Decrypt multiple secrets at once:

  $ init_password_store testpassword
  $ mkdir foo
  $ encrypt_secret foo/@bar testpassword secret1
  $ encrypt_secret foo/@baz testpassword secret2
  $ encrypt_secret buzz testpassword secret3
  $ sala get foo/@bar foo/@baz buzz <<EOF
  > testpassword
  > EOF
  Enter the master passphrase: 
  
  foo/@bar: secret1
  
  foo/@baz: secret2
  
  buzz: secret3
  
  $ cleanup

Decrypt multiple secrets at once, with raw output format:

  $ init_password_store testpassword
  $ mkdir foo
  $ encrypt_secret foo/@bar testpassword secret1
  $ encrypt_secret foo/@baz testpassword secret2
  $ encrypt_secret buzz testpassword secret3
  $ sala get -r foo/@bar foo/@baz buzz <<EOF
  > testpassword
  > EOF
  Enter the master passphrase: 
  secret1
  secret2
  secret3
  $ cleanup

Empty master passphrase given:

  $ init_password_store testpassword
  $ encrypt_secret @bar testpassword secret
  $ sala get @bar <<EOF
  > EOF
  Enter the master passphrase: 
  Empty passphrase is not allowed
  [1]

  $ cleanup

Implicit get:

  $ init_password_store testpassword
  $ encrypt_secret @bar testpassword secret
  $ sala @bar <<EOF
  > testpassword
  > EOF
  Enter the master passphrase: 
  
  @bar: secret
  
  $ cleanup

Implicit get with multiple files:

  $ init_password_store testpassword
  $ encrypt_secret @bar testpassword secret1
  $ encrypt_secret @baz testpassword secret2
  $ sala @bar @baz <<EOF
  > testpassword
  > EOF
  Enter the master passphrase: 
  
  @bar: secret1
  
  @baz: secret2
  
  $ cleanup

Implicit get with one file missing:

  $ init_password_store testpassword
  $ encrypt_secret @foo testpassword secret1
  $ encrypt_secret @baz testpassword secret2
  $ sala @foo @bar @baz
  Error: File does not exist: @bar
  [1]

  $ cleanup
