  $ . $TESTDIR/lib.sh

Non-executable post-set is not run:
  $ init_password_store testpassword
  $ mv .sala/hooks/post-set.sample .sala/hooks/post-set
  $ chmod -x .sala/hooks/post-set
  $ sala set foo <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for foo: 
  Confirm: 
  

SALADIR is available in post-set:
  $ cleanup
  $ mkdir store
  $ write_config store/.sala/config << EOF
  > password-generator
  > EOF
  $ SALADIR=store sala init >/dev/null 2>&1 << EOF
  > testpassword
  > testpassword
  > EOF
  $ cat > store/.sala/hooks/post-set << EOF
  > #!/bin/sh
  > echo "hooked" > \$SALADIR/\$1
  > EOF
  $ chmod +x store/.sala/hooks/post-set
  $ SALADIR=store sala set foo <<EOF
  > testpassword
  > secret
  > secret
  > EOF
  Enter the master passphrase: 
  
  Type a new secret for foo: 
  Confirm: 
  
  $ cat store/foo
  hooked
