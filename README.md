<div align="center">
<h1> RetardCalc </h1>
<img src="doc/logo.png" width="500">
</div>

# Introduction
**RetardCalc** is my first rust project which I made just for fun. Don't expect much from it. <br>
This project uses Abstract Syntax Tree (AST) to evaluate expressions. Proccess is pretty simple: <br>
1. lexer::tokenize Parses string into Tokens array.
2. parser.parse() Builds AST Nodes from Tokens and links them.
3. evaluator::eval() Evaluates AST Nodes and returns result.

# Screenshot
<img src="doc/screenshot.png">
