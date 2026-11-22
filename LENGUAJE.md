Lenguaje Aimara
===============

Este documento describe el vocabulario y la gramática que el compilador
reconoce actualmente, además de las convenciones de escritura del lenguaje.

Escritura sin apóstrofos
------------------------

El aimara suele usar el apóstrofo para marcar consonantes eyectivas, pero
en este lenguaje todas las palabras reservadas se escriben sin él para
facilitar el tipeo en teclados convencionales. Por eso se usan `taqa`,
`kutta`, `paki` y `kari`, no sus formas con apóstrofo.

El lexer ya no acepta el apóstrofo dentro de identificadores ni de
palabras reservadas: solo se usa como delimitador de textos. También se
admiten letras Unicode, como la `ñ`.

Vocabulario
-----------

### Tipos de dato

| Aimara | Español | Ejemplo |
| --- | --- | --- |
| `aru` | texto / cadena | `aru suti = 'Soviet'` |
| `jakhu` | entero | `jakhu mara = 4` |
| `taqa` | flotante / fracción | `taqa pachpa = 1.2` |

### Control de flujo

| Aimara | Español | Estado |
| --- | --- | --- |
| `ukhamax` | si | reservada, aún no parseada |
| `ukhamarak` | sino si | reservada, aún no parseada |
| `janukax` | sino | reservada, aún no parseada |
| `kama` | mientras | reservada, aún no parseada |
| `taki` | para | reservada, aún no parseada |

### Funciones y control

| Aimara | Español | Estado |
| --- | --- | --- |
| `lurawi` | función | implementada sin parámetros y se puede llamar |
| `kutta` | retornar | reservada, aún no parseada |
| `paki` | romper | reservada, aún no parseada |
| `saranta` | continuar | reservada, aún no parseada |

### Entrada, salida y booleanos

| Aimara | Español | Estado |
| --- | --- | --- |
| `qillqa` | imprimir / escribir | implementada con una expresión |
| `ulli` | leer | reservada, aún no parseada |
| `chiqa` | verdadero | reservada, aún no parseada |
| `kari` | falso | reservada, aún no parseada |

Gramática actual
----------------

La siguiente es una gramática EBNF de lo que el parser acepta hoy. `EOF`
representa el final del archivo, `?` indica un elemento opcional y `*`
indica repetición.

```ebnf
programa     = sentencia* EOF ;
sentencia    = declaracion
             | asignacion
             | funcion
             | impresion
             | llamada ;

declaracion  = ("aru" | "jakhu" | "taqa") IDENT "=" expresion ";"? ;
asignacion   = IDENT "=" expresion ";"? ;
funcion      = "lurawi" IDENT "{" sentencia* "}" ;
impresion    = "qillqa" "(" expresion ")" ";"? ;
llamada      = IDENT "(" ")" ";"? ;

expresion    = primario (("+" | "-") primario)* ;
primario     = IDENT | TEXTO | ENTERO | FLOTANTE ;
```

Reglas léxicas
--------------

- **Espacios:** se ignoran espacios, tabuladores y saltos de línea.
- **Identificadores:** letras, dígitos, `_` y punto. No se admite el
  apóstrofo. El punto dentro de un identificador todavía no significa
  acceso a atributos.
- **Textos:** se escriben entre comillas simples, por ejemplo `'hola'`.
  No se procesan escapes.
- **Enteros:** secuencias de dígitos sin signo, por ejemplo `42`.
- **Flotantes:** dígitos con un único punto decimal, por ejemplo `1.2`.
- **Símbolos:** `=`, `;`, `,`, `{`, `}`, `(`, `)`.
- **Comentarios:** todavía no se admiten.
- **Caracteres desconocidos:** se ignoran para mantener el lexer simple.

Ejecución
---------

Después de analizar el programa, el AST se ejecuta con un intérprete
sencillo:

- Las declaraciones crean variables en un entorno.
- Las asignaciones cambian el valor de una variable existente.
- `lurawi` guarda una función; `nombre()` la ejecuta.
- Cada llamada usa un entorno local, así las variables creadas dentro de
  la función no se mezclan con las globales.
- `qillqa(expresión)` evalúa e imprime el valor.
- `+` y `-` funcionan con enteros y flotantes.

Limitaciones actuales
---------------------

- Solo hay sumas (`+`) y restas (`-`); no hay multiplicación, división,
  comparaciones ni operaciones lógicas.
- No se parsean `ukhamax`, `kama`, `taki`, `kutta`, `paki` ni `saranta`.
- Las funciones no reciben parámetros y las llamadas son sin argumentos.
- No hay tabla de símbolos de compilación ni análisis semántico completo.
- Los errores no incluyen línea ni columna.
- Los booleanos `chiqa` y `kari` están reservados, pero no forman parte
  de las expresiones.
- No se pueden escribir números negativos ni notación científica.
- Los textos sin comilla de cierre se leen hasta el final del archivo.
- La ejecución es un intérprete: no se genera ensamblador ni código máquina.

Equivalencias con la versión Ashaninka
--------------------------------------

| Ashaninka | Aimara |
| --- | --- |
| `asanki` | `aru` |
| `toy` | `jakhu` |
| `kametsa` | `taqa` |
| `pai` | `ukhamax` |
| `pai_te` | `ukhamarak` |
| `kam` | `kama` |
| `kara` | `taki` |
| `tas` | `lurawi` |
| `paw` | `kutta` |
| `iroq` | `paki` |
| `irok` | `saranta` |
| `pawa` | `qillqa` |
| `toya` | `ulli` |

Además se agregaron `janukax` (sino), `chiqa` (verdadero) y `kari`
(falso), que no existían en la versión anterior.
