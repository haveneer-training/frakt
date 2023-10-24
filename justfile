#!/usr/bin/env just --justfile
 
# requires 
# - brew cask install basictex 
# - brew install pandoc just

pdf:
  pandoc Subject.md -o Subject.pdf --pdf-engine=/Library/TeX/texbin/pdflatex
  
