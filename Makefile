MANPAGE = sala.1
HTML = sala.1.html

clean:
	rm -f $(MANPAGE) $(HTML)

distclean: clean

man: $(MANPAGE)
html: $(HTML)

$(MANPAGE): $(MANPAGE).rst
	rst2man $(MANPAGE).rst $@

$(HTML): $(MANPAGE).rst
	rst2html $(MANPAGE).rst $@

.PHONY: clean distclean
