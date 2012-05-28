# This file is sourced in the tests. It sets up the environment for
# the tests and provides helper functions.

# Tell sala that we are running tests
export SALA_TESTS_RUNNING=1

# Run sala correctly
export PYTHONPATH=$TESTDIR/../

if [ -n "$COVERAGE" ]; then
    alias sala="coverage run --branch -a $TESTDIR/../bin/sala"
else
    alias sala="python $TESTDIR/../bin/sala"
fi

# Cleans up the test directory
cleanup() {
    rm -rf -- * .* >/dev/null 2>&1 || true
}

# Writes sala configuration
write_config() {
    local file=$1
    : ${file:=.sala/config}
    test -d `dirname $file` || mkdir `dirname $file`
    echo "[sala]" > $file
    while read key value; do
        echo "${key} = ${value}" >> $file
    done
}

# Encrypts stdin with GPG
gpg_encrypt() {
    local key=$(mktemp tmp.XXXXXXXXXX)
    printf "%s" "$2" >$key
    printf "%s" "$3" | \
        gpg --no-tty --batch --no-default-keyring --passphrase-fd 9 \
        --symmetric --output $1 9<$key
    rm -f -- $key
}

# Decrypts a GPG encrypted file
gpg_decrypt() {
    printf "%s" "$2" | \
        gpg --no-tty --batch --no-default-keyring --passphrase-fd 0 \
        --decrypt "$1"
}

# Encrypts a file like sala does. First, decrypts the master key and
# then encrypts the given file with the master key.
encrypt_secret() {
    local key=$(gpg_decrypt .sala/key "$2" 2>/dev/null)
    gpg_encrypt "$1" "$key" "$3"
}


# Decrypts a file written by sala. First, decrypts the master key and
# then decrypts the given file with the master key.
decrypt_secret() {
    local key=$(gpg_decrypt .sala/key "$2" 2>/dev/null)
    gpg_decrypt "$1" "$key" 2>/dev/null
}


# Helper to initialize the password store, with password generator set
# to empty in the config.

init_password_store() {
    local pass=$1
    write_config <<EOF
password-generator
EOF
    sala init >/dev/null 2>&1 <<EOF
$pass
$pass
EOF
}
