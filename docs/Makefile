LATEX ?= pdflatex
LATEXFLAGS :=

.DEFAULT_GOAL = paper.pdf

paper.pdf: paper.tex headers.tex body.tex works_cited.bib
	$(LATEX) $(LATEXFLAGS) $<
	@rm -f paper.{out,aux,log}

clean:
	@rm -f paper.{out,aux,log,pdf}

.PHONY = clean
