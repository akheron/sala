#!bash
#
# Copyright (C) 2011, 2012 Petri Lehtinen <petri@digip.org>
#
# sala is free software; you can redistribute it and/or modify it under
# the terms of the MIT license. See the file LICENSE distributed with
# the source code for details.
#
# The source code is available at https://github.com/akheron/sala.

_sala() {
    local IFS=$'\n' dir=${SALADIR:-.} cur

    if ! type _get_comp_words_by_ref >/dev/null 2>&1; then
        cur=${COMP_WORDS[$COMP_CWORD]}
    else
        _get_comp_words_by_ref cur
    fi

    # Check that we really have a sala repo
    [ -d "$dir/.sala" -o -f "$dir/.salakey" ] || return

    # Skip dotfiles, e.g. .sala/
    local names=$(cd $dir && compgen -f -- "$cur" | grep -v '^\.')

    COMPREPLY=($(for name in $names; do
        [ -d "$dir/$name" ] \
            && echo "${name}/" \
            || echo "$name "  # Add a space after filenames
    done))
}

complete -o nospace -F _sala sala
