# Syntax

## Statements

```.
dat myData : I32 = 14;

<data-def-stmt>
    : dat <id-expr> : <path-expr> = <expr>
```

## Expressions

```.
<id-expr>
    :  <id>

<path-expr>
    : <path-expr> . <id>
    | <id>   

<BinopExpr>
    : <expr> <binop-token> <factor>
    
<factor>
    : <num>
    | <path-expr>
```

## Tokens

```.
<lit-tok>
    : T
    | F
    | <[0..9]>
    | <[0..9]> . <[0..9]>
    | ' <char> '
    | " <[char]> "
    
<binop-tok>
    : +
    | -
    | *
    | /
    | %
```