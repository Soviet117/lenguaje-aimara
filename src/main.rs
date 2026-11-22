#![allow(dead_code)]
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Palabras reservadas
    AruKeyword,
    JakhuKeyword,
    TaqaKeyword,
    UkhamaxKeyword,
    UkhamarakKeyword,
    JanukaxKeyword,
    KamaKeyword,
    TakiKeyword,
    LurawiKeyword,
    KuttaKeyword,
    PakiKeyword,
    SarantaKeyword,
    QillqaKeyword,
    UlliKeyword,
    ChiqaKeyword,
    KariKeyword,

    // Valores
    Aru(String),
    Jakhu(i64),
    Taqa(f64),

    // Otros
    Variable(String),
    Equal,
    Semicolon,
    Comma,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Mas,
    Menos,
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
            '=' => {
                self.pos += 1;
                return Some(Token::Equal);
            }
            ';' => {
                self.pos += 1;
                return Some(Token::Semicolon);
            }
            ',' => {
                self.pos += 1;
                return Some(Token::Comma);
            }
            '{' => {
                self.pos += 1;
                return Some(Token::LBrace);
            }
            '}' => {
                self.pos += 1;
                return Some(Token::RBrace);
            }
            '(' => {
                self.pos += 1;
                return Some(Token::LParen);
            }
            ')' => {
                self.pos += 1;
                return Some(Token::RParen);
            }
            '+' => {
                self.pos += 1;
                return Some(Token::Mas);
            }
            '-' => {
                self.pos += 1;
                return Some(Token::Menos);
            }
            '\'' => return Some(self.leer_texto()),
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
        c.is_alphanumeric() || c == '_' || c == '.'
    }

    fn obtener_palabra(&self, inicio: usize) -> String {
        self.texto[inicio..self.pos].iter().collect()
    }

    fn leer_texto(&mut self) -> Token {
        self.pos += 1; // comilla inicial
        let mut valor = String::new();

        while self.pos < self.texto.len() {
            let c = self.texto[self.pos];
            if c == '\'' {
                self.pos += 1;
                break;
            }
            valor.push(c);
            self.pos += 1;
        }

        Token::Aru(valor)
    }

    fn clasificar_token(&mut self, palabra: String) -> Option<Token> {
        match palabra.as_str() {
            "aru" => return Some(Token::AruKeyword),
            "jakhu" => return Some(Token::JakhuKeyword),
            "taqa" => return Some(Token::TaqaKeyword),
            "ukhamax" => return Some(Token::UkhamaxKeyword),
            "ukhamarak" => return Some(Token::UkhamarakKeyword),
            "janukax" => return Some(Token::JanukaxKeyword),
            "kama" => return Some(Token::KamaKeyword),
            "taki" => return Some(Token::TakiKeyword),
            "lurawi" => return Some(Token::LurawiKeyword),
            "kutta" => return Some(Token::KuttaKeyword),
            "paki" => return Some(Token::PakiKeyword),
            "saranta" => return Some(Token::SarantaKeyword),
            "qillqa" => return Some(Token::QillqaKeyword),
            "ulli" => return Some(Token::UlliKeyword),
            "chiqa" => return Some(Token::ChiqaKeyword),
            "kari" => return Some(Token::KariKeyword),
            _ => {}
        }

        let es_entero = palabra.chars().all(|c| c.is_ascii_digit());
        let es_flotante = palabra.matches('.').count() == 1
            && palabra.chars().all(|c| c.is_ascii_digit() || c == '.');

        if es_entero {
            let valor = palabra.parse::<i64>().expect("Error al parsear entero");
            Some(Token::Jakhu(valor))
        } else if es_flotante {
            let valor = palabra.parse::<f64>().expect("Error al parsear flotante");
            Some(Token::Taqa(valor))
        } else {
            Some(Token::Variable(palabra))
        }
    }
}

// ==========================================
// ESTRUCTURAS DEL AST
// ==========================================

#[derive(Debug, Clone)]
enum Tipo {
    Aru,
    Jakhu,
    Taqa,
}

#[derive(Debug, Clone)]
enum Expresion {
    Variable(String),
    Aru(String),
    Jakhu(i64),
    Taqa(f64),
    Suma(Box<Expresion>, Box<Expresion>),
    Resta(Box<Expresion>, Box<Expresion>),
}

#[derive(Debug, Clone)]
enum Sentencia {
    Declaracion(Tipo, String, Expresion),
    Asignacion(String, Expresion),
    Funcion(String, Vec<Sentencia>),
    Imprimir(Expresion),
    LlamadaFuncion(String),
}

#[derive(Debug)]
struct Programa {
    sentencias: Vec<Sentencia>,
}

// ==========================================
// PARSER
// ==========================================

struct Parser {
    lexer: Lexer,
    token_actual: Option<Token>,
}

impl Parser {
    fn nuevo(mut lexer: Lexer) -> Self {
        let token_actual = lexer.obtener_token();
        Parser {
            lexer,
            token_actual,
        }
    }

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

    fn parsear_programa(&mut self) -> Result<Programa, String> {
        let mut sentencias = Vec::new();
        while self.token_actual.is_some() {
            let sentencia = self.parsear_sentencia()?;
            sentencias.push(sentencia);
        }
        Ok(Programa { sentencias })
    }

    fn parsear_sentencia(&mut self) -> Result<Sentencia, String> {
        let token = self
            .token_actual
            .clone()
            .ok_or("Fin de archivo inesperado")?;

        match token {
            // 1. Declaración: aru/jakhu/taqa nombre = valor
            Token::AruKeyword | Token::JakhuKeyword | Token::TaqaKeyword => {
                let tipo = match token {
                    Token::AruKeyword => Tipo::Aru,
                    Token::JakhuKeyword => Tipo::Jakhu,
                    Token::TaqaKeyword => Tipo::Taqa,
                    _ => unreachable!(),
                };
                self.avanzar();

                let nombre = match self.token_actual.clone() {
                    Some(Token::Variable(n)) => {
                        self.avanzar();
                        n
                    }
                    Some(t) => {
                        return Err(format!(
                            "Se esperaba el nombre de la variable, pero se encontró: {:?}",
                            t
                        ));
                    }
                    None => {
                        return Err(
                            "Se esperaba el nombre de la variable, pero se llegó al final"
                                .to_string(),
                        );
                    }
                };

                if !self.coincidir(Token::Equal) {
                    return Err(format!(
                        "Error en declaración de '{}': se esperaba '='",
                        nombre
                    ));
                }

                let valor = self.parsear_expresion()?;
                let _ = self.coincidir(Token::Semicolon);

                Ok(Sentencia::Declaracion(tipo, nombre, valor))
            }

            // 2. Asignación o llamada: nombre = valor / nombre()
            Token::Variable(nombre) => {
                self.avanzar();

                if self.coincidir(Token::LParen) {
                    if !self.coincidir(Token::RParen) {
                        return Err(format!(
                            "Se esperaba ')' al llamar a la función '{}'",
                            nombre
                        ));
                    }
                    let _ = self.coincidir(Token::Semicolon);
                    return Ok(Sentencia::LlamadaFuncion(nombre));
                }

                if !self.coincidir(Token::Equal) {
                    return Err(format!(
                        "Error en asignación: se esperaba '=' después de '{}'",
                        nombre
                    ));
                }

                let valor = self.parsear_expresion()?;
                let _ = self.coincidir(Token::Semicolon);

                Ok(Sentencia::Asignacion(nombre, valor))
            }

            // 3. Función: lurawi nombre { cuerpo }
            Token::LurawiKeyword => {
                self.avanzar();

                let nombre = match self.token_actual.clone() {
                    Some(Token::Variable(n)) => {
                        self.avanzar();
                        n
                    }
                    Some(t) => {
                        return Err(format!(
                            "Se esperaba el nombre de la función, pero se encontró: {:?}",
                            t
                        ));
                    }
                    None => {
                        return Err(
                            "Se esperaba el nombre de la función, pero se llegó al final"
                                .to_string(),
                        );
                    }
                };

                if !self.coincidir(Token::LBrace) {
                    return Err(format!(
                        "Se esperaba '{{' al iniciar la función '{}'",
                        nombre
                    ));
                }

                let mut cuerpo = Vec::new();
                while self.token_actual.is_some() && self.token_actual != Some(Token::RBrace) {
                    cuerpo.push(self.parsear_sentencia()?);
                }

                if !self.coincidir(Token::RBrace) {
                    return Err(format!(
                        "Se esperaba '}}' al finalizar la función '{}'",
                        nombre
                    ));
                }

                Ok(Sentencia::Funcion(nombre, cuerpo))
            }

            // 4. Impresión: qillqa(expresión)
            Token::QillqaKeyword => {
                self.avanzar();

                if !self.coincidir(Token::LParen) {
                    return Err("Se esperaba '(' después de 'qillqa'".to_string());
                }

                let expresion = self.parsear_expresion()?;

                if !self.coincidir(Token::RParen) {
                    return Err("Se esperaba ')' para cerrar qillqa()".to_string());
                }

                let _ = self.coincidir(Token::Semicolon);

                Ok(Sentencia::Imprimir(expresion))
            }

            _ => Err(format!(
                "Token no esperado al inicio de sentencia: {:?}",
                token
            )),
        }
    }

    // Expresión con sumas y restas, de izquierda a derecha.
    fn parsear_expresion(&mut self) -> Result<Expresion, String> {
        let mut izquierda = self.parsear_primario()?;

        loop {
            let operacion = match self.token_actual {
                Some(Token::Mas) => Token::Mas,
                Some(Token::Menos) => Token::Menos,
                _ => break,
            };
            self.avanzar();

            let derecha = self.parsear_primario()?;
            izquierda = match operacion {
                Token::Mas => Expresion::Suma(Box::new(izquierda), Box::new(derecha)),
                Token::Menos => Expresion::Resta(Box::new(izquierda), Box::new(derecha)),
                _ => unreachable!(),
            };
        }

        Ok(izquierda)
    }

    fn parsear_primario(&mut self) -> Result<Expresion, String> {
        let token = self
            .token_actual
            .clone()
            .ok_or("Se esperaba un valor, pero se llegó al final")?;

        let expresion = match token {
            Token::Variable(n) => Expresion::Variable(n),
            Token::Aru(s) => Expresion::Aru(s),
            Token::Jakhu(v) => Expresion::Jakhu(v),
            Token::Taqa(v) => Expresion::Taqa(v),
            _ => {
                return Err(format!(
                    "Se esperaba un valor o variable, pero se encontró: {:?}",
                    token
                ));
            }
        };

        self.avanzar();
        Ok(expresion)
    }
}

// ==========================================
// INTÉRPRETE
// ==========================================

#[derive(Debug, Clone)]
enum Valor {
    Aru(String),
    Jakhu(i64),
    Taqa(f64),
}

type Entorno = HashMap<String, Valor>;
type Funciones = HashMap<String, Vec<Sentencia>>;

fn ejecutar_programa(programa: &Programa) -> Result<(), String> {
    let mut variables = Entorno::new();
    let mut funciones = Funciones::new();
    ejecutar_bloque(&programa.sentencias, &mut variables, &mut funciones)
}

fn ejecutar_bloque(
    sentencias: &[Sentencia],
    variables: &mut Entorno,
    funciones: &mut Funciones,
) -> Result<(), String> {
    for sentencia in sentencias {
        ejecutar_sentencia(sentencia, variables, funciones)?;
    }
    Ok(())
}

fn ejecutar_sentencia(
    sentencia: &Sentencia,
    variables: &mut Entorno,
    funciones: &mut Funciones,
) -> Result<(), String> {
    match sentencia {
        Sentencia::Declaracion(tipo, nombre, expresion) => {
            if variables.contains_key(nombre) {
                return Err(format!("La variable '{}' ya está declarada", nombre));
            }

            let valor = evaluar_expresion(expresion, variables)?;

            if !tipo_correcto(tipo, &valor) {
                return Err(format!(
                    "La variable '{}' no coincide con el tipo declarado",
                    nombre
                ));
            }

            variables.insert(nombre.clone(), valor);
            Ok(())
        }

        Sentencia::Asignacion(nombre, expresion) => {
            let valor = evaluar_expresion(expresion, variables)?;

            if !variables.contains_key(nombre) {
                return Err(format!("La variable '{}' no está declarada", nombre));
            }

            variables.insert(nombre.clone(), valor);
            Ok(())
        }

        Sentencia::Funcion(nombre, cuerpo) => {
            funciones.insert(nombre.clone(), cuerpo.clone());
            Ok(())
        }

        Sentencia::Imprimir(expresion) => {
            let valor = evaluar_expresion(expresion, variables)?;
            imprimir_valor(&valor);
            Ok(())
        }

        Sentencia::LlamadaFuncion(nombre) => {
            let cuerpo = funciones
                .get(nombre)
                .cloned()
                .ok_or_else(|| format!("La función '{}' no está definida", nombre))?;

            // Cada llamada tiene sus propias variables locales.
            let mut locales = variables.clone();
            ejecutar_bloque(&cuerpo, &mut locales, funciones)
        }
    }
}

fn evaluar_expresion(expresion: &Expresion, variables: &Entorno) -> Result<Valor, String> {
    match expresion {
        Expresion::Variable(nombre) => variables
            .get(nombre)
            .cloned()
            .ok_or_else(|| format!("La variable '{}' no está declarada", nombre)),

        Expresion::Aru(s) => Ok(Valor::Aru(s.clone())),
        Expresion::Jakhu(v) => Ok(Valor::Jakhu(*v)),
        Expresion::Taqa(v) => Ok(Valor::Taqa(*v)),

        Expresion::Suma(izquierda, derecha) => {
            let a = evaluar_expresion(izquierda, variables)?;
            let b = evaluar_expresion(derecha, variables)?;
            sumar(a, b)
        }

        Expresion::Resta(izquierda, derecha) => {
            let a = evaluar_expresion(izquierda, variables)?;
            let b = evaluar_expresion(derecha, variables)?;
            restar(a, b)
        }
    }
}

fn sumar(a: Valor, b: Valor) -> Result<Valor, String> {
    match (a, b) {
        (Valor::Jakhu(a), Valor::Jakhu(b)) => Ok(Valor::Jakhu(a + b)),
        (Valor::Jakhu(a), Valor::Taqa(b)) => Ok(Valor::Taqa(a as f64 + b)),
        (Valor::Taqa(a), Valor::Jakhu(b)) => Ok(Valor::Taqa(a + b as f64)),
        (Valor::Taqa(a), Valor::Taqa(b)) => Ok(Valor::Taqa(a + b)),
        _ => Err("Solo se pueden sumar números".to_string()),
    }
}

fn restar(a: Valor, b: Valor) -> Result<Valor, String> {
    match (a, b) {
        (Valor::Jakhu(a), Valor::Jakhu(b)) => Ok(Valor::Jakhu(a - b)),
        (Valor::Jakhu(a), Valor::Taqa(b)) => Ok(Valor::Taqa(a as f64 - b)),
        (Valor::Taqa(a), Valor::Jakhu(b)) => Ok(Valor::Taqa(a - b as f64)),
        (Valor::Taqa(a), Valor::Taqa(b)) => Ok(Valor::Taqa(a - b)),
        _ => Err("Solo se pueden restar números".to_string()),
    }
}

fn tipo_correcto(tipo: &Tipo, valor: &Valor) -> bool {
    matches!(
        (tipo, valor),
        (Tipo::Aru, Valor::Aru(_)) | (Tipo::Jakhu, Valor::Jakhu(_)) | (Tipo::Taqa, Valor::Taqa(_))
    )
}

fn imprimir_valor(valor: &Valor) {
    match valor {
        Valor::Aru(s) => println!("{}", s),
        Valor::Jakhu(v) => println!("{}", v),
        Valor::Taqa(v) => println!("{}", v),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let codigo = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("No se pudo leer el archivo")
    } else {
        "aru suti = 'Soviet'\njakhu mara = 4\ntaqa pachpa = 1.2\njakhu yupa = 5\n\njakhu qhipa = 7\n\njakhu suma_total = mara + yupa\nqillqa(suma_total)\n\njakhu resta_total = qhipa - mara\nqillqa(resta_total)\n\nlurawi suma {\n    jakhu x = 2\n    jakhu resultado = x + 3\n    qillqa(resultado)\n}\n\nlurawi apsu {\n    qillqa(yupa)\n}\n\nsuma()\napssu()".to_string()
    };

    println!("Código a analizar:\n{}", codigo);
    println!("------------------------------------------------");

    let lexer = Lexer::nuevo(&codigo);
    let mut parser = Parser::nuevo(lexer);

    match parser.parsear_programa() {
        Ok(ast) => {
            println!("AST GENERADO CON ÉXITO:\n{:#?}", ast);
            println!("------------------------------------------------");
            println!("EJECUCIÓN:");

            match ejecutar_programa(&ast) {
                Ok(()) => println!("FIN DEL PROGRAMA"),
                Err(e) => println!("ERROR DE EJECUCIÓN: {}", e),
            }
        }
        Err(e) => {
            println!("ERROR SINTÁCTICO: {}", e);
        }
    }
}
