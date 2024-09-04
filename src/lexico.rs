// Estrou criando meus token que sao todos que meu lexer vai reconhecer 

// importação das libis para tratrem o input de arquivos
use std::fs;
use std::env;

#[derive(Debug, PartialEq)]
pub enum Token {
    Assign,         // =
    Plus,           // +
    Minus,          // -
    Multiply,       // *
    Divide,         // /
    Equal,          // ==
    NotEqual,       // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    If,             // if
    Else,           // else
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    Int(i32),       // [0-9]+
    Identifier(String), // [a-zA-Z_][a-zA-Z0-9_]*
    EOF,            // End of file/input
}

// Estrutura do meu lexer que contem:
// input que é a string de entrada que sera analisada
// position que é a posicao atual no input 
// current_char que é o caractere atual que esta sendo analisado

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}


// implementacao do lexer
impl Lexer {

    
    //funçao que avaança o ponteiro
    fn advance(&mut self) {
        self.position += 1;
        self.read_char();
    }

    // função que retorna o caractere sem avançar o ponteiro de leitura
    fn peek_char(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.position + 1).unwrap())
        }
    }

    fn read_char(&mut self) {
        // se a posição atual for maior ou igual ao tamanho do input
        // então o caractere atual é None
        self.current_char = if self.position >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.position).unwrap())
        };
    }


    fn read_number(&mut self) -> i32 {
        let start = self.position;
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position ].parse().unwrap()
    }

    // função que le identificadores e retorna o tipo string ou if else
    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
               self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    // função que pula os espaços em branco do input
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Funcao que le o proximo caractere do input e avanca a posicao
    pub fn new(input: String) -> Self {
        let mut lexer: Lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    // next_token é a função que retorna o próximo token do input
    // diferenciando o "=" atribuição do "==" igualdade

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace(); // remove espaços em branco
        let token = match self.current_char {

            // diferencia os meus simbolos compostos dos nao compostos
            // | != | == | <= | >= | = | < | > | ! |
            // pula duas vezes por que na função peek_char ele so olha o proximo elemento, 
            // entao caso seja um simbolo composto ele pula duas vezes
            // caso contrario ele pula uma vez só
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    self.advance();
                    Token::Equal
                } else {
                    self.advance();
                    Token::Assign
                }
            }
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    self.advance();
                    Token::LessEqual
                } else {
                    self.advance();
                    Token::Less
                }
            }
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    self.advance();
                    Token::GreaterEqual
                } else {
                    self.advance();
                    Token::Greater
                }
            }
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.advance(); 
                    self.advance();
                    Token::NotEqual
                } else {
                    panic!("Unexpected character: '!'")
                }
            }

            // atribui o token ao indentificador
            // + | - | * | / | ( | ) | { | }
            Some('+') => { 
                self.advance();
                Token::Plus 
            },
            Some('-') => { 
                self.advance();
                Token::Minus 
            },
            Some('*') => { 
                self.advance();
                Token::Multiply 
            },
            Some('/') => { 
                self.advance();
                Token::Divide 
            },
            Some('(') => { 
                self.advance();
                Token::LParen 
            },
            Some(')') => { 
                self.advance();
                Token::RParen 
            },
            Some('{') => { 
                self.advance();
                Token::LBrace 
            },
            Some('}') => { 
                self.advance();
                Token::RBrace 
            },

            // se o carcter atual for um digito chama o red_number para ler o numero e retorar o tipo int
            Some(c) if c.is_digit(10) => {
                let number = self.read_number();
                Token::Int(number)
            }

            // se o carcter for uma letra ele chama o read_identifier para ler o identificador e verificar se é  if e else
            Some(c) if c.is_alphabetic() => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "if" => Token::If,
                    "else" => Token::Else,
                    _ => Token::Identifier(identifier),
                }
            }

            // se o caractere atual for None retorna o token EOF
            None => Token::EOF,
            _ => panic!("Unexpected character: {:?}", self.current_char),
        };
        token
    }
} 

// inicia o lexema com a string de entrada
// Usa o loop para obter os tokens do lexema ate o EOF que meu criterio de parada
// e imprime os tokens
// Função principal que lê o arquivo e imprime os tokens
fn main() {
    // Obtem o caminho do arquivo da linha de comando  o arquivo foi passado na flag do cargo run -- nome-do-arquivo
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Le o conteúdo do arquivo
    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Inicializa o lexer com o conteúdo do arquivo
    let mut lexer = Lexer::new(input);

    // Processar e imprimir os tokens
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF { 
            break;
        }
    }
}