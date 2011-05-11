  $ . $TESTDIR/lib.sh

Change cipher to BLOWFISH:

  $ write_config <<EOF
  > cipher BLOWFISH
  > EOF

  $ sala init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF

  $ gpg_decrypt .salakey testpassword 2>&1 | head -n 1
  gpg: BLOWFISH encrypted data

  $ cleanup

Change key length:

  $ write_config <<EOF
  > key-length 3
  > EOF

  $ sala init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF

3 bytes -> 6 hexadecimal characters:

  $ gpg_decrypt .salakey testpassword 2>/dev/null; echo
  [0-9a-f]{6} (re)

  $ cleanup

Change password generator:

  $ write_config <<EOF
  > password-generator echo foo >out.txt
  > EOF

  $ sala init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF

  $ sala set secret >/dev/null 2>&1 <<EOF
  > testpassword
  > foo
  > foo
  > EOF

  $ cat out.txt
  foo

  $ cleanup

Test configuration override order. Use fake $HOME and $XDG_CONFIG_HOME
environment variables:

  $ mkdir store config home
  $ export HOME=$(pwd)/home
  $ export XDG_CONFIG_HOME=$(pwd)/config

  $ write_config "$HOME/.sala.conf" <<EOF
  > cipher CAST5
  > EOF
  $ write_config "$XDG_CONFIG_HOME/sala.conf" <<EOF
  > cipher 3DES
  > EOF

  $ cd store
  $ write_config sala.conf <<EOF
  > cipher BLOWFISH
  > EOF

Initialize with all config files in place. The one inside the store
(with BLOWFISH cipher) should take precedence.

  $ sala init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF
  $ gpg_decrypt .salakey testpassword 2>&1 | head -n 2 | tail -n 1
  gpg: BLOWFISH encrypted data

Remove the config from store, and initialize again. This time
$XDG_CONFIG_HOME/sala.conf (with 3DES cipher) should take precedence.

  $ rm sala.conf
  $ rm .salakey
  $ sala init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF
  $ gpg_decrypt .salakey testpassword 2>&1 | head -n 1
  gpg: 3DES encrypted data

Remove $XDG_CONFIG_HOME/sala.conf and initialize once more. Now,
$HOME/.sala.conf (with CAST5 cipher) should be active.

  $ rm $XDG_CONFIG_HOME/sala.conf
  $ rm .salakey
  $ sala init >/dev/null 2>&1 <<EOF
  > testpassword
  > testpassword
  > EOF
  $ gpg_decrypt .salakey testpassword 2>&1 | head -n 1
  gpg: CAST5 encrypted data
