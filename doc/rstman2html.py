#!/usr/bin/env python

'''rst2html.py from docutils, tweaked for manpage style HTML output'''

try:
    import locale
    locale.setlocale(locale.LC_ALL, '')
except:
    pass

from docutils import nodes
from docutils.core import publish_cmdline, default_description
from docutils.writers import html4css1

class Writer(html4css1.Writer):
    def __init__(self):
        html4css1.Writer.__init__(self)
        self.translator_class = Translator

class Translator(html4css1.HTMLTranslator):
    # HTML5
    xml_declaration = '<!-- %s -->'
    doctype = '<!DOCTYPE html>\n'
    head_prefix_template = '<html lang="%s"><!-- %s -->\n<head>\n'
    content_type = '<meta charset="%s">\n'
    stylesheet_link = '<link rel="stylesheet" href="%s" type="text/css">\n'

    def __init__(self, document):
        settings = {
            'embed_stylesheet': False,
            'stylesheet_path': 'manpage.css',
            'initial_header_level': 2,
        }
        for k, v in settings.items():
            setattr(document.settings, k, v)

        self._docinfo = {}
        self._in_doc_title = False
        self._in_doc_subtitle = False
        self._doc_title = ''
        self._doc_subtitle = ''

        html4css1.HTMLTranslator.__init__(self, document)

    def visit_title(self, node):
        if isinstance(node.parent, nodes.document):
            self._in_doc_title = True
        else:
            html4css1.HTMLTranslator.visit_title(self, node)

    def depart_title(self, node):
        if isinstance(node.parent, nodes.document):
            self._in_doc_title = False
        else:
            html4css1.HTMLTranslator.depart_title(self, node)

    def visit_subtitle(self, node):
        if isinstance(node.parent, nodes.document):
            self._in_doc_subtitle = True
        else:
            html4css1.HTMLTranslator.visit_subtitle(self, node)

    def depart_subtitle(self, node):
        if isinstance(node.parent, nodes.document):
            self._in_doc_subtitle = False
        else:
            html4css1.HTMLTranslator.depart_subtitle(self, node)

    def visit_docinfo_item(self, node, name, *a, **kw):
        self._docinfo[name] = node.astext()

    def visit_field_name(self, node):
        if self.in_docinfo:
            self._field_name = node.astext()
            raise nodes.SkipNode
        else:
            html4css1.HTMLTranslator.visit_field_name(self, node)

    def visit_field_body(self, node):
        if self.in_docinfo:
            name_normalized = self._field_name.lower().replace(' ', '_')
            self.visit_docinfo_item(node, name_normalized)
            raise nodes.SkipNode
        else:
            html4css1.HTMLTranslator.visit_field_body(self, node)

    def visit_Text(self, node):
        if self._in_doc_title:
            self._doc_title = node.astext()
        elif self._in_doc_subtitle:
            self._doc_subtitle = node.astext()
        else:
            html4css1.HTMLTranslator.visit_Text(self, node)

    def _section(self, title, text, target=None):
        (target or self.body).extend([
            '<div class="section" id="%s">' % title.lower(),
            '<h2>',
            self.encode(title),
            '</h2>',
            '<p>',
            self.encode(text),
            '</p>',
            '</div>',
        ])

    def _docinfo_section(self, item):
        self._section(item.upper(), self._docinfo[item])

    def depart_document(self, node):
        self.body_pre_docinfo = [
            '<div class="header">',
            '<div class="left">',
            self.encode('%s(%s)' % (
                self._doc_title.upper(),
                self._docinfo['manual_section'],
            )),
            '</div>',
            '<div class="center">',
            self.encode(self._docinfo['manual_group']),
            '<div class="right">',
            self.encode('%s(%s)' % (
                self._doc_title.upper(),
                self._docinfo['manual_section'],
            )),
            '</div></div></div>',
        ]
        self._section(
            'NAME',
            '%s - %s' % (self._doc_title, self._doc_subtitle),
            self.body_pre_docinfo,
        )

        self.docinfo = []

        if 'author' in self._docinfo:
            self._docinfo_section('author')

        if 'copyright' in self._docinfo:
            self._docinfo_section('copyright')

        html4css1.HTMLTranslator.depart_document(self, node)


    ### HTML5 tweaks ###

    def visit_literal(self, node):
        # <tt> -> <code>

        self.body.append(
            self.starttag(node, 'code', '', CLASS='docutils literal'))
        text = node.astext()
        for token in self.words_and_spaces.findall(text):
            if token.strip():
                # Protect text like "--an-option" and the regular expression
                # ``[+]?(\d+(\.\d*)?|\.\d+)`` from bad line wrapping
                if self.sollbruchstelle.search(token):
                    self.body.append('<span class="pre">%s</span>'
                                     % self.encode(token))
                else:
                    self.body.append(self.encode(token))
            elif token in ('\n', ' '):
                # Allow breaks at whitespace:
                self.body.append(token)
            else:
                # Protect runs of multiple spaces; the last space can wrap:
                self.body.append('&nbsp;' * (len(token) - 1) + ' ')
        self.body.append('</code>')
        # Content already processed:
        raise nodes.SkipNode

    def visit_option_list(self, node):
        # valign="top", frame="void", rules="none" removed

        self.body.append(
              self.starttag(node, 'table', CLASS='docutils option-list'))
        self.body.append('<col class="option" />\n'
                         '<col class="description" />\n'
                         '<tbody>\n')


description = ('Generates manpage style HTML documents.  ' +
               default_description)
publish_cmdline(writer=Writer(), description=description)
