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
    Error(String),
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
            '\'' => return Some(self.leer_texto()),
            _ => {}
        }

        let inicio = self.pos;
        self.avanzar_mientras_sea_valido();

        if inicio == self.pos {
            let desconocido = self.texto[self.pos];
            self.pos += 1;
            return Some(Token::Error(format!(
                "carácter no reconocido: '{}'",
                desconocido
            )));
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

    // Lee un texto entre comillas simples y devuelve su contenido sin comillas.
    fn leer_texto(&mut self) -> Token {
        self.pos += 1; // comilla inicial
        let mut valor = String::new();

        while let Some(&c) = self.texto.get(self.pos) {
            match c {
                '\'' => {
                    self.pos += 1;
                    return Token::Texto(valor);
                }
                '\\' => {
                    self.pos += 1;
                    let Some(&escape) = self.texto.get(self.pos) else {
                        return Token::Error("cadena de texto sin comilla de cierre".to_string());
                    };
                    valor.push(match escape {
                        'n' => '\n',
                        't' => '\t',
                        '\\' => '\\',
                        '\'' => '\'',
                        otro => otro,
                    });
                    self.pos += 1;
                }
                _ => {
                    valor.push(c);
                    self.pos += 1;
                }
            }
        }

        Token::Error("cadena de texto sin comilla de cierre".to_string())
    }

    fn clasificar_token(&mut self, palabra: String) -> Option<Token> {
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

        let es_entero = palabra.chars().all(|c| c.is_ascii_digit());
        let es_flotante = palabra.matches('.').count() == 1
            && palabra.chars().all(|c| c.is_ascii_digit() || c == '.');

        if es_entero {
            let valor = palabra.parse::<u32>().expect("Error al parsear entero");
            Some(Token::Entero(valor))
        } else if es_flotante {
            let valor = palabra.parse::<f32>().expect("Error al parsear flotante");
            Some(Token::Flotante(valor))
        } else {
            Some(Token::Variable(palabra))
        }
    }
}

// ==========================================
// ESTRUCTURAS DEL AST
// ==========================================

#[derive(Debug, PartialEq)]
enum Tipo {
    Aru,
    Jakhu,
    Taqa,
}

#[derive(Debug, PartialEq)]
enum Expresion {
    Variable(String),
    Texto(String),
    Entero(u32),
    Flotante(f32),
}

#[derive(Debug, PartialEq)]
enum Sentencia {
    // Declaracion: Tipo de dato, nombre de la variable y su valor inicial
    Declaracion(Tipo, String, Expresion),
    // Asignacion: Nombre de variable y su nuevo valor
    Asignacion(String, Expresion),
    // Funcion: Nombre y la lista de sentencias dentro de sus llaves
    Funcion(String, Vec<Sentencia>),
    // Impresion: qillqa puede recibir una expresion o ningun argumento
    Impresion(Option<Expresion>),
}

#[derive(Debug, PartialEq)]
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
        Parser {
            lexer,
            token_actual,
        }
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

    fn verificar_error_lexico(&self) -> Result<(), String> {
        if let Some(Token::Error(mensaje)) = &self.token_actual {
            return Err(format!("ERROR LÉXICO: {}", mensaje));
        }
        Ok(())
    }

    // Parsea todo el texto y retorna Result. Si encuentra un error, se detiene inmediatamente.
    fn parsear_programa(&mut self) -> Result<Programa, String> {
        let mut sentencias = Vec::new();
        while self.token_actual.is_some() {
            self.verificar_error_lexico()?;
            let sentencia = self.parsear_sentencia()?;
            sentencias.push(sentencia);
        }
        Ok(Programa { sentencias })
    }

    // Valida la sintaxis del lenguaje
    fn parsear_sentencia(&mut self) -> Result<Sentencia, String> {
        self.verificar_error_lexico()?;
        let token = self
            .token_actual
            .clone()
            .ok_or("Fin de archivo inesperado")?;

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
                    Some(t) => {
                        return Err(format!(
                            "Se esperaba el nombre de la variable, pero se encontró: {:?}",
                            t
                        ));
                    }
                    None => return Err(
                        "Se esperaba el nombre de la variable, pero se llegó al fin del archivo"
                            .to_string(),
                    ),
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
                let nombre =
                    match self.token_actual.clone() {
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
                        None => return Err(
                            "Se esperaba el nombre de la función, pero se llegó al fin del archivo"
                                .to_string(),
                        ),
                    };

                // Esperamos llave abierta '{'
                if !self.coincidir(Token::LBrace) {
                    return Err(format!(
                        "Se esperaba '{{' al iniciar la función '{}', pero se encontró: {:?}",
                        nombre, self.token_actual
                    ));
                }

                // Parseamos todo lo que está dentro de las llaves
                let mut cuerpo = Vec::new();
                while self.token_actual.is_some() && self.token_actual != Some(Token::RBrace) {
                    cuerpo.push(self.parsear_sentencia()?);
                }

                // Esperamos llave cerrada '}'
                if !self.coincidir(Token::RBrace) {
                    return Err(format!(
                        "Se esperaba '}}' al finalizar la función '{}', pero se llegó al fin del archivo",
                        nombre
                    ));
                }

                Ok(Sentencia::Funcion(nombre, cuerpo))
            }

            // 3. Impresión: qillqa(expresion) o qillqa()
            Token::QillqaKeyword => {
                self.avanzar(); // Saltamos 'qillqa'

                // Esperamos '('
                if !self.coincidir(Token::LParen) {
                    return Err(format!(
                        "Se esperaba '(' después de 'qillqa', pero se encontró: {:?}",
                        self.token_actual
                    ));
                }

                // El argumento es opcional para permitir qillqa()
                let argumento = if self.token_actual == Some(Token::RParen) {
                    None
                } else {
                    Some(self.parsear_expresion()?)
                };

                // Esperamos ')'
                if !self.coincidir(Token::RParen) {
                    return Err(format!(
                        "Se esperaba ')' para cerrar la llamada a qillqa, pero se encontró: {:?}",
                        self.token_actual
                    ));
                }

                let _ = self.coincidir(Token::Semicolon);

                Ok(Sentencia::Impresion(argumento))
            }

            _ => Err(format!(
                "Token no esperado al inicio de sentencia: {:?}",
                token
            )),
        }
    }

    // Parsea los valores a la derecha de '='
    fn parsear_expresion(&mut self) -> Result<Expresion, String> {
        self.verificar_error_lexico()?;
        let token = self
            .token_actual
            .clone()
            .ok_or("Se esperaba un valor, pero se llegó al fin del archivo")?;
        let expresion = match token {
            Token::Variable(n) => Expresion::Variable(n),
            Token::Texto(s) => Expresion::Texto(s),
            Token::Entero(v) => Expresion::Entero(v),
            Token::Flotante(v) => Expresion::Flotante(v),
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let codigo = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("No se pudo leer el archivo")
    } else {
        include_str!("../main.aym").to_string()
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
            if e.starts_with("ERROR LÉXICO") {
                println!("{}", e);
            } else {
                println!("ERROR SINTÁCTICO: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lexemas(codigo: &str) -> Vec<Token> {
        let mut lexer = Lexer::nuevo(codigo);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.obtener_token() {
            tokens.push(token);
        }
        tokens
    }

    fn parsear(codigo: &str) -> Result<Programa, String> {
        Parser::nuevo(Lexer::nuevo(codigo)).parsear_programa()
    }

    #[test]
    fn reconoce_todas_las_palabras_reservadas_aimaras() {
        let entrada = "aru jakhu t'aqa ukhamax ukhamarak janukax kama taki \
                       lurawi kutt'a p'aki saranta qillqa ulli chiqa k'ari";
        let esperado = vec![
            Token::AruKeyword,
            Token::JakhuKeyword,
            Token::TaqaKeyword,
            Token::UkhamaxKeyword,
            Token::UkhamarakKeyword,
            Token::JanukaxKeyword,
            Token::KamaKeyword,
            Token::TakiKeyword,
            Token::LurawiKeyword,
            Token::KuttaKeyword,
            Token::PakiKeyword,
            Token::SarantaKeyword,
            Token::QillqaKeyword,
            Token::UlliKeyword,
            Token::ChiqaKeyword,
            Token::KariKeyword,
        ];

        assert_eq!(lexemas(entrada), esperado);
    }

    #[test]
    fn reconoce_literales_y_simbolos() {
        let entrada = "aru x = 'hola' 42 1.5 ; , { } ( )";
        let esperado = vec![
            Token::AruKeyword,
            Token::Variable("x".to_string()),
            Token::Equal,
            Token::Texto("hola".to_string()),
            Token::Entero(42),
            Token::Flotante(1.5),
            Token::Semicolon,
            Token::Comma,
            Token::LBrace,
            Token::RBrace,
            Token::LParen,
            Token::RParen,
        ];

        assert_eq!(lexemas(entrada), esperado);
    }

    #[test]
    fn procesa_escapes_en_los_textos() {
        assert_eq!(
            lexemas("'a\\'b\\n'"),
            vec![Token::Texto("a'b\n".to_string())]
        );
    }

    #[test]
    fn parsea_declaraciones_con_los_tres_tipos() {
        let ast = parsear("aru suti = 'Soviet'\njakhu mara = 4\nt'aqa pachpa = 1.2")
            .expect("el programa debería ser válido");

        assert_eq!(
            ast,
            Programa {
                sentencias: vec![
                    Sentencia::Declaracion(
                        Tipo::Aru,
                        "suti".to_string(),
                        Expresion::Texto("Soviet".to_string()),
                    ),
                    Sentencia::Declaracion(Tipo::Jakhu, "mara".to_string(), Expresion::Entero(4),),
                    Sentencia::Declaracion(
                        Tipo::Taqa,
                        "pachpa".to_string(),
                        Expresion::Flotante(1.2),
                    ),
                ],
            }
        );
    }

    #[test]
    fn parsea_una_funcion_con_qillqa_sin_argumentos() {
        let ast = parsear("lurawi suma {\n    jakhu x = 2\n    qillqa(x)\n    qillqa()\n}")
            .expect("la función debería ser válida");

        assert_eq!(
            ast,
            Programa {
                sentencias: vec![Sentencia::Funcion(
                    "suma".to_string(),
                    vec![
                        Sentencia::Declaracion(Tipo::Jakhu, "x".to_string(), Expresion::Entero(2),),
                        Sentencia::Impresion(Some(Expresion::Variable("x".to_string()))),
                        Sentencia::Impresion(None),
                    ],
                )],
            }
        );
    }

    #[test]
    fn qillqa_admite_textos_literales() {
        assert_eq!(
            parsear("qillqa('hola')").expect("la impresión debería ser válida"),
            Programa {
                sentencias: vec![Sentencia::Impresion(Some(Expresion::Texto(
                    "hola".to_string()
                )))],
            }
        );
    }

    #[test]
    fn las_palabras_ashaninkas_ya_no_son_reservadas() {
        let esperado = vec![
            Token::Variable("asanki".to_string()),
            Token::Variable("toy".to_string()),
            Token::Variable("kametsa".to_string()),
            Token::Variable("tas".to_string()),
            Token::Variable("pawa".to_string()),
        ];

        assert_eq!(lexemas("asanki toy kametsa tas pawa"), esperado);
    }

    #[test]
    fn reporta_caracteres_desconocidos_como_error_lexico() {
        let error = parsear("jakhu x = 1 @ 2").unwrap_err();
        assert!(
            error.contains("ERROR LÉXICO"),
            "error inesperado: {}",
            error
        );
        assert!(
            error.contains("carácter no reconocido"),
            "error inesperado: {}",
            error
        );
    }

    #[test]
    fn reporta_textos_sin_comilla_de_cierre() {
        let error = parsear("aru suti = 'hola").unwrap_err();
        assert!(
            error.contains("ERROR LÉXICO"),
            "error inesperado: {}",
            error
        );
        assert!(
            error.contains("sin comilla de cierre"),
            "error inesperado: {}",
            error
        );
    }

    #[test]
    fn el_ejemplo_principal_es_valido() {
        let codigo = include_str!("../main.aym");
        assert!(
            parsear(codigo).is_ok(),
            "main.aym debería analizarse sin errores"
        );
    }
}
