#!/usr/bin/env python
#
# Copyright (C) 2011, 2012 Petri Lehtinen <petri@digip.org>
#
# sala is free software; you can redistribute it and/or modify it under
# the terms of the MIT license. See the file LICENSE distributed with
# the source code for details.
#
# The source code is available at https://github.com/akheron/sala.

'''rst2man.py from docutils, tweaked for nicer output'''

import locale
try:
    locale.setlocale(locale.LC_ALL, '')
except:
    pass

from docutils import nodes
from docutils.core import publish_cmdline, default_description
from docutils.writers import manpage

LITERAL_BLOCK_INDENT = 3

class Writer(manpage.Writer):
    def __init__(self):
        manpage.Writer.__init__(self)
        self.translator_class = Translator

class Translator(manpage.Translator):
    # Indent literal blocks
    def visit_literal_block(self, node):
        self.indent(LITERAL_BLOCK_INDENT)
        self.indent(0)
        manpage.Translator.visit_literal_block(self, node)

    def depart_literal_block(self, node):
        manpage.Translator.depart_literal_block(self, node)
        self.dedent()
        self.dedent()

    # Remove vertical gap between definition list term. This has an
    # effect when the term contains markup. (When the term is plain
    # text, the magic .sp handling in manpage.Translator would change
    # ".sp" to ".".)
    def visit_paragraph(self, node):
        self.ensure_eol()
        if isinstance(node.parent, nodes.definition):
            self.body.append('.\n')
        else:
            self.body.append('.sp\n')

description = ("Generates plain unix manual documents.  " + default_description)
publish_cmdline(writer=Writer(), description=description)
