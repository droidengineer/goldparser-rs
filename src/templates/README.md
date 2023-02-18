# GOLD Parser Rust Skeleton Templates


One of the key obstacles for those using a specific implementation of the Engine is interacting with a table of rules and symbols. Each rule and symbol is to be uniquely identified by a table index. If a rule, for instance, has an index of 10 in the parse tables, the developer must use this value in their programs.
Manually typing each constant definition can be both tedious and problematic - given that a single incorrect constant could be difficult to debug. For most programming languages and scripting languages, the number of rules can easily exceed a hundred.

Program Templates are designed to resolve this issue. Essentially, program templates are a type of tool designed to help the programmer create a "skeleton program" which contains the source code that is necessary to use a particular implementation of the Engine. For instance, if an Engine is created for the Java Programming Language, a Program Template can be used to create a basic skeleton program to use it. This skeleton program would contain the necessary declarations and function calls to the Engine. In other words, Program Templates help a programmer use an Engine.

When a developer creates a new implementation of the Engine, they can create a template containing the bare-minimum code needed to interact with it. Of course, programmers who use a particular implementation of the Engine can create their own templates for whatever reason they need.

Some implementations of the Engine can work with multiple programming languages. These include, for instance, a version created and compiled to a Microsoft ActiveX (COM) object. This can be "plugged" into various development suites such as Microsoft Visual Basic 6, Microsoft C++ 6 and Borland Delphi 7.

Program templates are implemented as simple text files that contain a number of preprocessor-type tags. These tags are used to designate template attributes and to mark where lists will be inserted containing the grammar's symbols and rules. The format of the tags designed to be versatile so that the they can be used to create lists of constants, case statements, or whatever the developer needs.
>From http://goldparser.org/doc/templates/index.htm

----------------
`convoluted_parser.pgt` is the top-level skeleton program template for rust.
`convoluted_ruolehandlers.pgt` is a break-out file enumerating a rulehanlder
for each production in your grammar.
