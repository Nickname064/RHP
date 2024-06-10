# RHP TODO List

## [ ] Parser
- [X] Get functional HTML parser
- [ ] Fix bugs with parser consuming wrong tag ends

## [ ] Custom elements
- [X] Implement basic Custom Elements
- [X] Implement Recursive Custom Elements
  - [X] Avoid infinite recursive algorithm
- [X] Give elements easy access to their parents
- [ ] Implement child selectors
  - [ ] Implement basic queries
    - [ ] Implement attr-value queries
  - [X] Implement env queries (check parent)

## Language
- [ ] Add import statement (only import custom selectors)
- [ ] Add include statement (import selectors and text contents)
  
## Dev QOL
- [ ] Improve parser to add error handling
- [ ] Write Linter and Language Server
  
## Versatility
- [ ] Add compatibility with other HTML-like formats (HAML)


## Bug fixes

- [X] custom self-closed elements can take child text