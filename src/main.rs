#![allow(dead_code)]
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Palabras reservadas del lenguaje Aimara
    AruKeyword,       // aru: texto
    JakhuKeyword,     // jakhu: entero
    TaqaKeyword,      // t'aqa: flotante
    UkhamaxKeyword,   // ukhamax: si
    UkhamarakKeyword, // ukhamarak: sino si
    JanukaxKeyword,   // janukax: sino
    KamaKeyword,      // kama: mientras
    TakiKeyword,      // taki: para
    LurawiKeyword,    // lurawi: funcion
    KuttaKeyword,     // kutt'a: retornar
    PakiKeyword,      // p'aki: romper
    SarantaKeyword,   // saranta: continuar
    QillqaKeyword,    // qillqa: imprimir
    UlliKeyword,      // ulli: leer
    ChiqaKeyword,     // chiqa: verdadero
    KariKeyword,      // k'ari: falso

    // Valores
    Texto(String),
    Entero(u32),
    Flotante(f32),

    // Otros
    Variable(String),
    Equal,
    Semicolon,
    Comma,
    LBrace,
    RBrace,
    LParen,
    RParen,
}

struct Lexer {
    texto: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn nuevo(entrada: &str) -> Self {
        Lexer {
            texto: entrada.chars().collect(),
            pos: 0,
        }
    }

    fn obtener_token(&mut self) -> Option<Token> {
        self.saltar_espacios();

        if self.esta_al_final() {
            return None;
        }

        let c = self.texto[self.pos];
        match c {
            '=' => { self.pos += 1; return Some(Token::Equal); }
            ';' => { self.pos += 1; return Some(Token::Semicolon); }
            ',' => { self.pos += 1; return Some(Token::Comma); }
            '{' => { self.pos += 1; return Some(Token::LBrace); }
            '}' => { self.pos += 1; return Some(Token::RBrace); }
            '(' => { self.pos += 1; return Some(Token::LParen); }
            ')' => { self.pos += 1; return Some(Token::RParen); }
            _ => {}
        }

        let inicio = self.pos;
        self.avanzar_mientras_sea_valido();

        if inicio == self.pos {
            self.pos += 1; // saltar carácter no reconocido
            return self.obtener_token();
        }

        let palabra = self.obtener_palabra(inicio);
        self.clasificar_token(palabra)
    }

    fn saltar_espacios(&mut self) {
        while self.pos < self.texto.len() && self.texto[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn esta_al_final(&self) -> bool {
        self.pos >= self.texto.len()
    }

    fn avanzar_mientras_sea_valido(&mut self) {
        while self.pos < self.texto.len() && self.es_caracter_valido(self.texto[self.pos]) {
            self.pos += 1;
        }
    }

    fn es_caracter_valido(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '\'' || c == '.'
    }

    fn obtener_palabra(&self, inicio: usize) -> String {
        self.texto[inicio..self.pos].iter().collect()
    }

    fn clasificar_token(&mut self, palabra: String) -> Option<Token> {
        if self.termina_con_comilla() {
            return Some(Token::Texto(palabra));
        }

        match palabra.as_str() {
            "aru" => return Some(Token::AruKeyword),
            "jakhu" => return Some(Token::JakhuKeyword),
            "t'aqa" => return Some(Token::TaqaKeyword),
            "ukhamax" => return Some(Token::UkhamaxKeyword),
            "ukhamarak" => return Some(Token::UkhamarakKeyword),
            "janukax" => return Some(Token::JanukaxKeyword),
            "kama" => return Some(Token::KamaKeyword),
            "taki" => return Some(Token::TakiKeyword),
            "lurawi" => return Some(Token::LurawiKeyword),
            "kutt'a" => return Some(Token::KuttaKeyword),
            "p'aki" => return Some(Token::PakiKeyword),
            "saranta" => return Some(Token::SarantaKeyword),
            "qillqa" => return Some(Token::QillqaKeyword),
            "ulli" => return Some(Token::UlliKeyword),
            "chiqa" => return Some(Token::ChiqaKeyword),
            "k'ari" => return Some(Token::KariKeyword),
            _ => {}
        }

        if palabra.contains('.') {
            let valor = palabra.parse::<f32>().expect("Error al parsear flotante");
            Some(Token::Flotante(valor))
        } else if palabra.chars().all(|c| c.is_ascii_digit()) {
            let valor = palabra.parse::<u32>().expect("Error al parsear entero");
            Some(Token::Entero(valor))
        } else {
            Some(Token::Variable(palabra))
        }
    }

    fn termina_con_comilla(&self) -> bool {
        self.pos > 0 && self.texto[self.pos - 1] == '\''
    }
}

// ==========================================
// ESTRUCTURAS DEL AST
// ==========================================

#[derive(Debug)]
enum Tipo {
    Aru,
    Jakhu,
    Taqa,
}

#[derive(Debug)]
enum Expresion {
    Variable(String),
    Texto(String),
    Entero(u32),
    Flotante(f32),
}

#[derive(Debug)]
enum Sentencia {
    // Declaracion: Tipo de dato, nombre de la variable y su valor inicial
    Declaracion(Tipo, String, Expresion),
    // Asignacion: Nombre de variable y su nuevo valor
    Asignacion(String, Expresion),
    // Funcion: Nombre y la lista de sentencias dentro de sus llaves
    Funcion(String, Vec<Sentencia>),
    // Llamada: Imprimir (qillqa) una variable
    Llamada(String),
}

#[derive(Debug)]
struct Programa {
    sentencias: Vec<Sentencia>,
}

// ==========================================
// PARSER CON VALIDACIÓN
// ==========================================

struct Parser {
    lexer: Lexer,
    token_actual: Option<Token>,
}

impl Parser {
    fn nuevo(mut lexer: Lexer) -> Self {
        let token_actual = lexer.obtener_token();
        Parser { lexer, token_actual }
    }

    // Avanza al siguiente token generado por el lexer
    fn avanzar(&mut self) {
        self.token_actual = self.lexer.obtener_token();
    }

    fn coincidir(&mut self, token_esperado: Token) -> bool {
        if let Some(ref t) = self.token_actual {
            if std::mem::discriminant(t) == std::mem::discriminant(&token_esperado) {
                self.avanzar();
                return true;
            }
        }
        false
    }

    // Parsea todo el texto y retorna Result. Si encuentra un error, se detiene inmediatamente.
    fn parsear_programa(&mut self) -> Result<Programa, String> {
        let mut sentencias = Vec::new();
        while self.token_actual.is_some() {
            let sentencia = self.parsear_sentencia()?;
            sentencias.push(sentencia);
        }
        Ok(Programa { sentencias })
    }

    // Valida la sintaxis del lenguaje
    fn parsear_sentencia(&mut self) -> Result<Sentencia, String> {
        let token = self.token_actual.clone().ok_or("Fin de archivo inesperado")?;

        match token {
            // 1. Declaraciones de variables: aru/jakhu/t'aqa nombre = valor
            Token::AruKeyword | Token::JakhuKeyword | Token::TaqaKeyword => {
                let tipo = match token {
                    Token::AruKeyword => Tipo::Aru,
                    Token::JakhuKeyword => Tipo::Jakhu,
                    Token::TaqaKeyword => Tipo::Taqa,
                    _ => unreachable!(),
                };
                self.avanzar(); // Saltamos el tipo de variable

                // Esperamos un identificador (Variable)
                let nombre = match self.token_actual.clone() {
                    Some(Token::Variable(n)) => {
                        self.avanzar();
                        n
                    }
                    Some(t) => return Err(format!("Se esperaba el nombre de la variable, pero se encontró: {:?}", t)),
                    None => return Err("Se esperaba el nombre de la variable, pero se llegó al fin del archivo".to_string()),
                };

                // Esperamos el signo '='
                if !self.coincidir(Token::Equal) {
                    return Err(format!(
                        "Error en declaración de '{}': se esperaba el símbolo '=' después del nombre, pero se encontró: {:?}",
                        nombre, self.token_actual
                    ));
                }

                // Esperamos el valor de la variable
                let valor = self.parsear_expresion()?;

                // Permitir punto y coma ';' opcional
                let _ = self.coincidir(Token::Semicolon);

                Ok(Sentencia::Declaracion(tipo, nombre, valor))
            }

            // 4. Asignaciones directas a variables: nombre = valor
            Token::Variable(nombre) => {
                self.avanzar(); // Saltamos el nombre

                // Esperamos el signo '='
                if !self.coincidir(Token::Equal) {
                    return Err(format!(
                        "Error en asignación: se esperaba '=' después de '{}', pero se encontró: {:?}",
                        nombre, self.token_actual
                    ));
                }

                let valor = self.parsear_expresion()?;

                let _ = self.coincidir(Token::Semicolon);

                Ok(Sentencia::Asignacion(nombre, valor))
            }

            // 2. Funciones: lurawi nombre { cuerpo }
            Token::LurawiKeyword => {
                self.avanzar(); // Saltamos 'lurawi'

                // Esperamos el nombre de la función
                let nombre = match self.token_actual.clone() {
                    Some(Token::Variable(n)) => {
                        self.avanzar();
                        n
                    }
                    Some(t) => return Err(format!("Se esperaba el nombre de la función, pero se encontró: {:?}", t)),
                    None => return Err("Se esperaba el nombre de la función, pero se llegó al fin del archivo".to_string()),
                };

                // Esperamos llave abierta '{'
                if !self.coincidir(Token::LBrace) {
                    return Err(format!("Se esperaba '{{' al iniciar la función '{}', pero se encontró: {:?}", nombre, self.token_actual));
                }

                // Parseamos todo lo que está dentro de las llaves
                let mut cuerpo = Vec::new();
                while self.token_actual.is_some() && self.token_actual != Some(Token::RBrace) {
                    cuerpo.push(self.parsear_sentencia()?);
                }

                // Esperamos llave cerrada '}'
                if !self.coincidir(Token::RBrace) {
                    return Err(format!("Se esperaba '}}' al finalizar la función '{}', pero se llegó al fin del archivo", nombre));
                }

                Ok(Sentencia::Funcion(nombre, cuerpo))
            }

            // 3. Llamadas: qillqa(variable)
            Token::QillqaKeyword => {
                self.avanzar(); // Saltamos 'qillqa'

                // Esperamos '('
                if !self.coincidir(Token::LParen) {
                    return Err(format!("Se esperaba '(' después de 'qillqa', pero se encontró: {:?}", self.token_actual));
                }

                // Esperamos la variable
                let variable = match self.token_actual.clone() {
                    Some(Token::Variable(v)) => {
                        self.avanzar();
                        v
                    }
                    Some(t) => return Err(format!("Se esperaba una variable dentro de qillqa(), pero se encontró: {:?}", t)),
                    None => return Err("Se esperaba una variable dentro de qillqa(), pero se llegó al fin del archivo".to_string()),
                };

                // Esperamos ')'
                if !self.coincidir(Token::RParen) {
                    return Err(format!("Se esperaba ')' para cerrar la llamada a qillqa, pero se encontró: {:?}", self.token_actual));
                }

                let _ = self.coincidir(Token::Semicolon);

                Ok(Sentencia::Llamada(variable))
            }

            _ => Err(format!("Token no esperado al inicio de sentencia: {:?}", token)),
        }
    }

    // Parsea los valores a la derecha de '='
    fn parsear_expresion(&mut self) -> Result<Expresion, String> {
        let token = self.token_actual.clone().ok_or("Se esperaba un valor, pero se llegó al fin del archivo")?;
        let expresion = match token {
            Token::Variable(n) => Expresion::Variable(n),
            Token::Texto(s) => Expresion::Texto(s),
            Token::Entero(v) => Expresion::Entero(v),
            Token::Flotante(v) => Expresion::Flotante(v),
            _ => return Err(format!("Se esperaba un valor o variable, pero se encontró: {:?}", token)),
        };
        self.avanzar();
        Ok(expresion)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let codigo = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("No se pudo leer el archivo")
    } else {
        "aru suti = 'Soviet'\njakhu mara = 4\nt'aqa pachpa = 1.2\njakhu yupa = 5\n\njakhu qhipa = 7\n\nlurawi suma {\n    jakhu x = 2\n    qillqa(x)\n}\n\nlurawi resta {\n    qillqa(qhipa)\n}".to_string()
    };

    println!("Código a analizar:\n{}", codigo);
    println!("------------------------------------------------");

    let lexer = Lexer::nuevo(&codigo);
    let mut parser = Parser::nuevo(lexer);

    match parser.parsear_programa() {
        Ok(ast) => {
            println!("AST GENERADO CON ÉXITO:\n{:#?}", ast);
        }
        Err(e) => {
            println!("ERROR SINTÁCTICO: {}", e);
        }
    }
}
