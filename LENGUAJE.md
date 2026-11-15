Lenguaje Aimara
===============

Este documento describe el vocabulario y la gramática que el compilador
reconoce actualmente, además de las convenciones de escritura del lenguaje.

Ortografía
----------

El aimara usa el apóstrofo para marcar consonantes eyectivas, por ejemplo
en `t'aqa`, `kutt'a`, `p'aki` y `k'ari`. Por eso el lexer permite el
apóstrofo dentro de los identificadores y palabras reservadas. También se
admiten letras Unicode, como la `ñ`.

Vocabulario
-----------

### Tipos de dato

| Aimara | Español | Ejemplo |
| --- | --- | --- |
| `aru` | texto / cadena | `aru suti = 'Soviet'` |
| `jakhu` | entero | `jakhu mara = 4` |
| `t'aqa` | flotante / fracción | `t'aqa pachpa = 1.2` |

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
| `lurawi` | función | implementada sin parámetros |
| `kutt'a` | retornar | reservada, aún no parseada |
| `p'aki` | romper | reservada, aún no parseada |
| `saranta` | continuar | reservada, aún no parseada |

### Entrada, salida y booleanos

| Aimara | Español | Estado |
| --- | --- | --- |
| `qillqa` | imprimir / escribir | implementada con 0 o 1 argumento |
| `ulli` | leer | reservada, aún no parseada |
| `chiqa` | verdadero | reservada, aún no parseada |
| `k'ari` | falso | reservada, aún no parseada |

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
             | impresion ;

declaracion  = ("aru" | "jakhu" | "t'aqa") IDENT "=" expresion ";"? ;
asignacion   = IDENT "=" expresion ";"? ;
funcion      = "lurawi" IDENT "{" sentencia* "}" ;
impresion    = "qillqa" "(" expresion? ")" ";"? ;

expresion    = IDENT | TEXTO | ENTERO | FLOTANTE ;
```

Reglas léxicas
--------------

- **Espacios:** se ignoran espacios, tabuladores y saltos de línea.
- **Identificadores:** letras, dígitos, `_`, apóstrofo y punto. El punto
  dentro de un identificador todavía no significa acceso a atributos.
- **Textos:** se escriben entre comillas simples, por ejemplo `'hola'`.
  Se reconocen los escapes `\n`, `\t`, `\\` y `\'`.
- **Enteros:** secuencias de dígitos sin signo, por ejemplo `42`.
- **Flotantes:** dígitos con un único punto decimal, por ejemplo `1.2`.
- **Símbolos:** `=`, `;`, `,`, `{`, `}`, `(`, `)`.
- **Comentarios:** todavía no se admiten.
- **Errores:** un carácter desconocido o un texto sin comilla de cierre
  produce un error léxico.

Limitaciones actuales
---------------------

- No hay operadores aritméticos, de comparación ni lógicos.
- No se parsean `ukhamax`, `kama`, `taki`, `kutt'a`, `p'aki` ni `saranta`.
- Las funciones no reciben parámetros ni argumentos.
- No existen llamadas a funciones definidas por el usuario.
- No hay tabla de símbolos ni comprobación de tipos.
- Los errores no incluyen línea ni columna.
- Los booleanos `chiqa` y `k'ari` están reservados, pero no forman parte
  de las expresiones.
- No se pueden escribir números negativos ni notación científica.

Equivalencias con la versión Ashaninka
--------------------------------------

| Ashaninka | Aimara |
| --- | --- |
| `asanki` | `aru` |
| `toy` | `jakhu` |
| `kametsa` | `t'aqa` |
| `pai` | `ukhamax` |
| `pai_te` | `ukhamarak` |
| `kam` | `kama` |
| `kara` | `taki` |
| `tas` | `lurawi` |
| `paw` | `kutt'a` |
| `iroq` | `p'aki` |
| `irok` | `saranta` |
| `pawa` | `qillqa` |
| `toya` | `ulli` |

Además se agregaron `janukax` (sino), `chiqa` (verdadero) y `k'ari`
(falso), que no existían en la versión anterior.
