#!/usr/bin/env just --justfile
 
pdf:
  pandoc Subject.md -o Subject.pdf --pdf-engine=/Library/TeX/texbin/pdflatex
  
