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
T       ::= 'int' | '(' E ')' | B | IfStmt
B       ::= '{' E '}'
I       ::= 'if' '(' E ')' B ElseStmt
L       ::= 'else' B | ε
```

### Notação da Gramatica

- *E (Expressão):* Representa uma expressão completa, com ```soma``` e ```subtração```.
- *C (Comparação):* Trata ```comparações``` entre expressões, como ```==, !=, <, >, <=, >=```.
- *F (Fator):* Um fator pode ser um termo com ```multiplicação``` ou ```divisão```.
- *T (Termo):* Um termo pode ser um ```número inteiro,``` uma expressão entre ```parênteses```, um ```bloco de código``` ou uma instrução ```if```.
- *B (Bloco):* Define um ```bloco de código``` cercado por ```{}```.
- *I (Instrução If):* Define a estrutura ```if  else``` com ```blocos de código```.
- *L (Instrução Else):* Representa a parte opcional ```else``` de um ```if```, com um ````bloco de código````.
