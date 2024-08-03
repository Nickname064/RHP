# RHP TODO List

## [ ] Parser
- [X] Get functional HTML parser
- [X] Fix bugs with parser consuming wrong tag ends
- [ ] Add doctypes and comments
- [ ] Self-closable tags don't HAVE to self-close
  - TIP : [ ] Rewrite parser to be iterative instead of recursive

## [ ] Custom elements
- [X] Implement basic Custom Elements
- [ ] Enforce a hyphen in every custom element
- [X] Implement Recursive Custom Elements
  - [X] Avoid infinitely recursive custom elements
- [X] Give elements easy access to their parents
- [ ] Implement child selectors
  - [ ] Implement basic queries
    - [ ] Implement attr-value queries
  - [X] Implement env queries (check parent)

## Language
- [ ] Add import statement (only import custom selectors)
- [ ] Add include statement (import selectors and text contents)
  
## Dev QOL
- [X] Improve parser to add error handling
- [ ] Write Linter and Language Server
  
## Versatility
- [ ] Add compatibility with other HTML-like formats (HAML)


## Bug fixes

- [X] custom self-closed elements can take child text