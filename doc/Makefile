PYTHON = python
RST2MAN = $(PYTHON) rst2man.py
RSTMAN2HTML = $(PYTHON) rstman2html.py
SOURCE = sala.1.rst
MANPAGE = sala.1
HTML = sala.1.html

all: man html

man: $(MANPAGE)
html: $(HTML)

$(MANPAGE): $(SOURCE)
	$(RST2MAN) $^ $@

$(HTML): $(SOURCE)
	$(RSTMAN2HTML) $^ $@

clean:
	rm -f $(MANPAGE) $(HTML)

distclean: clean

.PHONY: doc man html clean distclean
