Hoja de ruta del compilador Aimara
=================================

Estado de partida
-----------------

Hoy el proyecto recorre la primera parte del proceso y ejecuta el AST con
un intérprete sencillo:

```text
fuente .aym  ->  lexer  ->  parser  ->  AST  ->  intérprete
```

Todavía no existe la cadena completa:

```text
AST  ->  análisis semántico  ->  tabla de símbolos  ->  código intermedio
     ->  optimización  ->  ensamblador  ->  código máquina  ->  ejecutable
```

Las fases están alineadas con el sílabo del curso (unidades 1 a 5). La
columna **Estado** indica qué existe en el repositorio en este momento.

| Fase | Unidad del sílabo | Estado |
| --- | --- | --- |
| Análisis lexicográfico | Unidad 2 | Básico |
| Análisis sintáctico | Unidad 3 | Básico |
| Ejecución / intérprete | Apoyo didáctico | Básico |
| Análisis semántico | Unidad 4 | Pendiente |
| Gestión de tabla de símbolos | Unidades 1 y 4 | Pendiente |
| Traducción / código intermedio | Unidad 5 | Pendiente |
| Optimización | Unidad 5 | Pendiente |
| Generación de ensamblador | Unidad 5 | Pendiente |
| Generación de código máquina y cierre | Unidad 5 | Pendiente |

Fase 1: análisis lexicográfico
------------------------------

**Ya existe:** palabras reservadas aimaras, identificadores, enteros,
flotantes, textos simples, símbolos, operadores `+` y `-`, e ignorado de
caracteres desconocidos.

**Falta:**

- [ ] Guardar línea y columna de cada token para reportar errores con
      ubicación.
- [ ] Agregar comentarios de línea y de bloque.
- [ ] Reconocer el resto de operadores: `*`, `/`, `%`, `==`, `!=`, `<`,
      `<=`, `>`, `>=`, `&&`, `||`, `!`.
- [ ] Reconocer booleanos `chiqa` y `kari` como literales utilizables.
- [ ] Admitir números negativos y notación científica.
- [ ] Definir una política de recuperación ante errores léxicos.
- [ ] Separar el lexer en un módulo propio y exponer el flujo de tokens
      para poder probarlo de forma aislada.

Fase 2: análisis sintáctico
---------------------------

**Ya existe:** declaraciones con tipo, asignaciones, funciones sin
parámetros, llamadas sin argumentos, `qillqa(expresión)` y expresiones con
sumas y restas.

**Falta:**

- [ ] Ampliar las expresiones: multiplicación, división, comparación,
      lógica, precedencia y paréntesis.
- [ ] Implementar `ukhamax` / `janukax` / `ukhamarak` (si / sino / sino si).
- [ ] Implementar `kama` (mientras) y `taki` (para).
- [ ] Implementar `kutta` (retornar), `paki` (romper) y `saranta`
      (continuar).
- [ ] Agregar parámetros y argumentos a las funciones.
- [ ] Agregar llamadas a funciones con argumentos.
- [ ] Manejar bloques y ámbitos anidados.
- [ ] Definir la gramática EBNF completa y mantenerla en `LENGUAJE.md`.
- [ ] Mejorar la recuperación de errores sintácticos para reportar más de
      un error por compilación.
- [ ] Adjuntar la posición del token a cada nodo del AST.

Fase 3: análisis semántico
--------------------------

**Falta:**

- [ ] Construir la tabla de símbolos con ámbitos (global, función, bloque).
- [ ] Detectar variables no declaradas.
- [ ] Detectar declaraciones duplicadas en el mismo ámbito.
- [ ] Verificar tipos en declaraciones, asignaciones y operaciones.
- [ ] Verificar la compatibilidad de tipos en comparaciones y condiciones.
- [ ] Verificar la aridad y los tipos de los argumentos en llamadas.
- [ ] Verificar que las funciones que retornan un valor usen `kutta`.
- [ ] Detectar uso de variables antes de su inicialización.
- [ ] Reportar errores semánticos con línea y columna.
- [ ] Definir las reglas de conversión entre `jakhu` y `taqa`.

Fase 4: gestión de tabla de símbolos
------------------------------------

**Falta:**

- [ ] Diseñar una estructura de símbolos con nombre, tipo, ámbito,
      categoría (variable, función, parámetro), línea y columna.
- [ ] Implementar una pila de ámbitos para entrar y salir de funciones y
      bloques.
- [ ] Resolver nombres y detectar sombreado (*shadowing*).
- [ ] Vincular cada identificador del AST con su entrada en la tabla.
- [ ] Exponer la tabla de símbolos para depuración y para las fases
      posteriores.

Fase 5: traducción y código intermedio
--------------------------------------

**Falta:**

- [ ] Definir una representación intermedia, por ejemplo código de tres
      direcciones (TAC) con temporales y etiquetas.
- [ ] Traducir declaraciones, asignaciones y expresiones a TAC.
- [ ] Traducir condicionales y bucles a etiquetas y saltos.
- [ ] Traducir funciones, parámetros, llamadas y retornos.
- [ ] Implementar una tabla de temporales y etiquetas.
- [ ] Validar el TAC con un intérprete o con volcado legible.

Fase 6: optimización
-------------------

**Falta:**

- [ ] Construir el grafo de flujo de control (CFG).
- [ ] Aplicar plegado de constantes (*constant folding*).
- [ ] Aplicar propagación de copias.
- [ ] Eliminar código muerto.
- [ ] Simplificar expresiones y saltos redundantes.
- [ ] Medir el efecto de cada optimización con programas de prueba.

Fase 7: generación de ensamblador
---------------------------------

Objetivo sugerido: x86-64 sobre Linux, usando `as` y `gcc` para ensamblar
y enlazar. Es una arquitectura accesible y permite cerrar el proceso sin
escribir un ensamblador propio.

**Falta:**

- [ ] Definir la convención de llamadas (SysV AMD64) y el uso de
      registros.
- [ ] Asignar registros o usar la pila para variables y temporales.
- [ ] Generar prólogo y epílogo de funciones y marcos de pila.
- [ ] Traducir operaciones aritméticas y comparaciones a instrucciones.
- [ ] Traducir saltos, etiquetas, llamadas y retornos.
- [ ] Implementar `qillqa` y `ulli` mediante llamadas al sistema o a
      `printf`/`scanf`.
- [ ] Emitir un archivo `.s` legible y verificable.

Fase 8: código máquina y cierre
-------------------------------

**Falta:**

- [ ] Invocar `as` para producir un archivo objeto `.o`.
- [ ] Invocar `gcc` (o `ld`) para enlazar y producir un ejecutable.
- [ ] Definir el contrato de la interfaz de línea de comandos:
      `compilador archivo.aym -o salida`.
- [ ] Emitir mensajes de error y códigos de salida adecuados.
- [ ] Probar el ciclo completo con programas de ejemplo.
- [ ] Documentar la arquitectura, las decisiones y las limitaciones.
- [ ] Agregar pruebas automatizadas de extremo a extremo.
- [ ] Preparar la sustentación del trabajo final.

Orden recomendado de trabajo
----------------------------

1. Operadores y expresiones con precedencia.
2. Condicionales y bucles.
3. Funciones con parámetros y llamadas.
4. Tabla de símbolos y análisis semántico.
5. Código intermedio.
6. Optimización básica.
7. Ensamblador x86-64.
8. Ensamblado, enlace y pruebas de extremo a extremo.

Criterio de cierre
------------------

Se considerará que el compilador está completo cuando pueda tomar un
programa `.aym` con variables, expresiones, condicionales, bucles y
funciones, validarlo semánticamente, generar ensamblador x86-64, producir
un ejecutable y ejecutarlo con el resultado esperado, reportando errores
con línea y columna en cada fase.
