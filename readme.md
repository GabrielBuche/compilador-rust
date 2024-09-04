# Exercício Compiladores

> Foi baseado nas atividades feita em sala, e com auxilio do livro "Compiladores Princípios e técnicas e ferramentas 2° Edição"

## Enunciado

### 1 - Construa uma Gramática e uma Tabela de Símbolos para uma linguagem de programação que respeite os seguintes critérios

- Há somente o tipo de dado número inteiro;
- As variáveis não são tipadas;
- Declaração e atribuição de variáveis deve ser a mesma sintaxe (igual o Python);
- Deve realizar as operações matemáticas básicas (+, -, *, / ) com variáveis e números inteiro, respeitando a associatividade e precedência dos operadores;
- Deve permitir expressões matemáticas prioritárias entre parênteses nas operações matemáticas;
- Declaração e atribuição de variável pode haver operações matemática;
- Implementar os blocos condicionais "if" e "else";
- A condição do "if" somente irá comparar 2 valores, podendo ser variável ou números inteiro, com os operadores igual, menor, maior e diferente (definir os símbolos para os operadores).

### 2 - Implemente um Analisador Léxico e um Analisador Sintático Preditivo LL(1) para a linguagem desenvolvida

O que será avaliado?

- Se a tabela de símbolos e a gramática estão de acordo com os critérios definidos;
- Se o os algoritmos implementam as técnicas apresentadas e especificadas;
- Se o algoritmo analisará corretamente as entradas definidas na hora da avaliação em sala.

## Tabela de símbolos

| Símbolos | Lexema    | Descrição |
|----------|-----------|-----------|
| =        | Assign    | Operador de atribuição |
| +        | Plus      | Operador de adição |
| -        | Minus     | Operador de subtração |
| *        | Multiply  | Operador de multiplicação |
| /        | Divide    | Operador de divisão |
| ==       | Equal     | Operador de igualdade |
| !=       | NotEqual  | Operador de desigualdade |
| <        | Less      | Operador menor que |
| >        | Greater   | Operador maior que |
| <=       | LessEqual | Operador menor ou igual a |
| >=       | GreaterEqual | Operador maior ou igual a |
| if       | if     | Palavra-chave para condição |
| else     | else   | Palavra-chave para condição |
| (        | LParen | Abre Parêntese  |
| )        | RParen | Fecha Parêntese  |
| {        | LBrace |  Abre Chave |
| }        | RBrace | Fecha Parêntese  |
| [0-9]+   | int | Número inteiro |
| [a-zA-Z_] [a-zA-Z0-9_]* | id | Identificador de variável |

## Gramatica

```ebnf
E       ::= C E'
E'      ::= '+' C E' | '-' C E' | ε
C       ::= F C'
C'      ::= '==' F C' | '!=' F C' | '<' F C' | '>' F C' | '<=' F C' | '>=' F C' | ε
F       ::= T F'
F'      ::= '*' T F' | '/' T F' | ε
T       ::= 'int' | '(' E ')' | Block | IfStmt
Block   ::= '{' E '}'
IfStmt  ::= 'if' '(' E ')' Block ElseStmt
ElseStmt ::= 'else' Block | ε
```

### Explicação da gramatica

- **E**: Representa uma expressão que pode ser uma comparação (`C`) seguida de zero ou mais operações de adição ou subtração (`E'`).
- **E'**: Representa zero ou mais operações de adição (`+`) ou subtração (`-`) seguidas de uma comparação (`C`). Pode ser vazio (`ε`).
- **C**: Representa uma comparação que pode ser um fator (`F`) seguido de zero ou mais operações de comparação (`C'`).
- **C'**: Representa zero ou mais operações de comparação (`==`, `!=`, `<`, `>`, `<=`, `>=`) seguidas de um fator (`F`). Pode ser vazio (`ε`).
- **F**: Representa um fator que pode ser um termo (`T`) seguido de zero ou mais operações de multiplicação ou divisão (`F'`).
- **F'**: Representa zero ou mais operações de multiplicação (`*`) ou divisão (`/`) seguidas de um termo (`T`). Pode ser vazio (`ε`).
- **T**: Representa um termo que pode ser um número inteiro (`int`), uma expressão entre parênteses (`( E )`), um bloco de código (`Block`), ou uma instrução `if` (`IfStmt`).
- **Block**: Representa um bloco de código delimitado por chaves (`{ E }`).
- **IfStmt**: Representa uma instrução `if` seguida de uma expressão entre parênteses (`( E )`), um bloco de código (`Block`), e uma instrução `else` opcional (`ElseStmt`).
- **ElseStmt**: Representa uma instrução `else` seguida de um bloco de código (`Block`) ou nada (`ε`).

### Notação

- **ε**: Representa a produção vazia, ou seja, nada.
- **|**: Representa uma escolha entre alternativas.
- **()**: Usado para agrupar expressões.
- **{}**: Delimitadores de bloco de código.
- **int**: Representa um número inteiro.
- **if**: Palavra-chave para a instrução condicional.
- **else**: Palavra-chave para a instrução condicional alternativa.