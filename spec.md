
# Neoshell Language Specification

 - [1. Syntax](#1-syntax)
   - [1.1. Commands](#11-commands)
     - [1.1.1. Piping](#111-piping)
	 - [1.1.2. Arguments](#112-arguments)
   - [1.2. Blocks](#12-blocks)
   - [1.3. Expressions](#13-expressions)
   - [1.4. Descriptors](#14-descriptors)
     - [1.4.1. Argument descriptors](#141-argument-descriptors)
	   - [1.4.1.1. Positional argument descriptors](#1411-positional-argument-descriptor)
	   - [1.4.1.2. Positional list argument descriptors](#1411-positional-list-argument-descriptor)
	   - [1.4.1.3. Flag argument descriptors](#1413-flag-argument-descriptor)
	   - [1.4.1.4. Option argument descriptors](#1414-option-argument-descriptor)
	   - [1.4.1.5. List argument descriptors](#1415-list-argument-descriptor)
	   - [1.4.1.6. Choice argument descriptors](#1416-choice-argument-descriptor)
	   - [1.4.1.7. Named choice argument descriptors](#1417-named-choice-argument-descriptor)
	   - [1.4.1.8. Multi-choice argument descriptors](#1418-multi-choice-argument-descriptor)
	   - [1.4.1.9. Named multi-choice argument descriptors](#1419-named-multi-choice-argument-descriptor)
   - [1.5. Literals](#15-literals)
	 - [1.5.1. Strings](#151-strings)
	 - [1.5.2. Numbers](#152-numbers)
	   - [1.5.2.1. Integers](#1521-integers)
	   - [1.5.2.2. Floats](#1522-floats)
   - [1.6. Packages](#16-packages)
   - [1.7. Variables](#17-variables)
   - [1.8. Placeholders](#18-placeholders)
 - [2. Language's constructs](#2-language-s-constructs)
   - [2.1. Commands](#21-commands)
     - [2.1.1. Execution times](#211-execution-times)
	 - [2.1.2. Invocation algorithm](#212-invocation-algorithm)
 - [3. Neoshell infrastructure commands](#3-neoshell-infrastructure-commands)
 - [4. Neoshell standard library commands](#4-neoshell-standard-library-commands)


## 1. Syntax

A Neoshell script is written in a UTF-8 encoded file. All of the production
rules only accepts ASCII character unless stated otherwise.

Whitespace characters are only used to provide a way to separate tokens, and as
such does not carry any meaning on themselves. A file can be written by using
only spaces `U+0020` or newlines `U+000A` and still carry the same meaning.

Comments are interpreted as whitespace and follow Bash's syntax. They start with
a `#` `U+0023` and continues until the end of the line. As such, there is no
inline comments. All comments are ignored and as such aren't registered as
documentation. Neoshell supports documenting elements by other means.

A script is simply composed of a list of commands separated by whitespace. This
specification follows the given syntax descriptors :

 - `name` : mandatory item in the syntax, where `name` refers to the production
   rule's name of that item.
 - `[name]` : optional item in the syntax.
 - `name...` : Represents a list of item named `name`, separated by whitespace.


### 1.1. Commands

A command is formed of a `command-name` item, followed by an optional list of
argument and ended by a `;` `U+003B` : `command-name [args...] ;` The name of a
command can be a simple identifier following regex rule `[a-zA-Z_][a-zA-Z0-9_\-]*`.
It can also be a path which uses `::` as a path separator and is widely used
when using packages.

The `command-name` can be an identifier path, but it can also be a variable
or an evaluated block, only when the resulting value is an instantiated command.

If the command is the only command in a block, the ending `;` can be omitted.
These blocks are called one-command blocks.

#### 1.1.1. Piping

Commands can generates or consumes a stream of data and the latter can be
passed around by using the pipe operator `|>`. A command generating a
stream is called a generator, and a command consuming a stream is a
consumer.

Example :
```
read "my-file.txt" |> search "term" |> write "my-search-result.txt";
```

#### 1.1.2. Arguments

When invoking a command receiving arguments, the latter can be formated in
different ways based on their descriptors. Positional arguments are just values
passed to the command without special syntax unlike non-positional arguments
which all have special syntaxes. Here is the exhaustive list of all
non-positional arguments and their syntax :

- Flag `/name` : passes a binary (by default, `false` if the argument is
  omitted, `true` when specified) value to the command.
- Option `name = value` : passes an arbitrary value of a given type to the
  command.
- List `name[] = v0,v1,...vN` : passes an arbitrary value of a given type to
  the command.
- Choice `name -> value` : passes a value to the command. This argument can
  takes a specified list of values, and specifying a value outside of that
  list results in a compilation error.
- Named choice `name -> value` : works like a choice argument, but uses a
  name-value pair instead. This kind of argument can be described as "static",
  as a block or a variable can't contain the name of a value of a named choice.
- Multi-choice `name => v0,v1,...,vN` : works like a choice argument, but
  accepts a list of potential values.
- Named multi-choice `name => n0,n1,...nN` : works like a multi-choice and
  a named choice.

Positional arguments can't be repeated (except if they are positional list
arguments which works like variadic arguments in C). Non-positional arguments
can be repeated, and each kind of argument can have it's own behavior regarding
its repetitions :

- Repeating a flag argument corresponds to just having that flag specified once.
  As such, it is considered a mistake to repeat a flag and will generate a
  warning.
- Repeating an option corresponds to just having that option set by the last
  occurrence of that argument (the argument takes the value of the latest
  occurrence). As such, it is considered a mistake to repeat an option and will
  generate a warning.
- Repeating a list corresponds to concatenating all of the occurrences of that
  argument. Thus, this is a valid usage and can be use to increase readability.
- Repeating a choice corresponds to just having that choice set by the last
  occurrence of that argument (the argument takes the value of the latest
  occurrence). As such, it is considered a mistake to repeat a choice and will
  generate a warning.
- Repeating a named choice is equivalent to repeating a choice argument.
- Repeating a multi-choice is equivalent to repeating a list argument.
- Repeating a named multi-choice is equivalent to repeating a list argument.


### 1.2. Blocks

A block is just a list of commands in a sub-scope. They are formed by using
curly braces : `{ commands... }`.

A block can be passed around in two manners : as argument, or as evaluated
value. Most of the time, the Neoshell infrastructure can infer the correct usage
when a block is passed to a CT, MAC or RT command, but the programmer may wish
to manually specify the usage :

- `&{ commands... }` : This syntax tells the compiler the block must be
  passed as argument. Useful for callbacks or lambda-like usages.
- `!{ commands... }` : This syntax tells the compiler the block must be executed
  and its result passed as an argument.

When the syntax does not explicitly defines the usage of a block, the Neoshell
infrastructure infers the usage like so :

- If it's an argument to a CT or MAC command : the block is passed as argument.
- Otherwise, the block is evaluated and the result used as an argument.

On top of that, the compiler can uses a command's argument descriptors to infers
the correct usage.


### 1.3. Expressions

An expression allows to write infix-formed expressions for logic, bitwise and
mathematical operators. They are always surrounded by parenthesis :
`( expression )`.

Exhaustive list of operators, in ascending precedence :

| Precedence | Operator(s)          | Description    |
| ---------- | -------------------- | -------------- |
| 1          | `or`                 | Logical OR     |
| 2          | `and`                | Logical AND    |
| 3          | `\|`                 | Bitwise OR     |
| 4          | `^`                  | Bitwise XOR    |
| 5          | `&`                  | Bitwise AND    |
| 6          | `==`, `!=`           | Equality       |
| 7          | `<`, `<=`, `>`, `>=` | Comparison     |
| 8          | `<<`, `>>`           | Bitwise shifts |
| 9          | `+`, `-`             | Terms          |
| 10         | `*`, `/`, `%`        | Factors        |

It's possible to call commands from expressions by using blocks. In that case,
the Neoshell compiler always interpret the block as evaluated.

```
(1 + { sin (2 * $PI * $x) })
```


### 1.4. Descriptors

A descriptor is used to describes command's usages. A descriptor is formed with
the following syntax : `' descriptor... ;`

There is 5 different kinds of descriptors :

 - Static descriptors, formed as `" word "`. It tells the Neoshell's command
   invocation algorithm there must be `word` at that position in the command's
   arguments.
 - Mandatory descriptors, formed as `< argument-descriptor >`. It tells the
   Neoshell's invocation algorithm there must be an argument fitting
   `argument-descriptor`.
 - Optional descriptor, formed as `[ argument-descriptor... ]`. It tells the
   Neoshell's invocation algorithm there can be an argument fitting
   `argument-descriptor`. This syntax allows multiple descriptor in the same
   place : `[ desc0; desc1; ...; descN ]`. In that case, if `desc0` is present
   in the command's arguments, then `desc1`, `desc2`, ..., `descN` must also
   be specified.
 - Pipe input descriptor, formed as `|>type`. It tells Neoshell's invocation
   algorithm the commands accepts a streaming input of type `type`. If omitted,
   the commands can not receive any input of that kind.
 - Pipe output descriptor, formed as `type|>`. It tells Neoshell's invocation
   algorithm the commands generates a streaming output of type `type`. If omitted,
   the commands can not generate any output of that kind.


#### 1.4.1. Argument descriptors

Argument descriptors describe the pattern and the usage of the arguments of a
command. The arguments are divided into two kinds : positional and
non-positional.

Positional arguments are always enclosed in `mandatory-descriptor` or in
`optional-descriptor`. If enclosed in the former, no default value can be
defined, and if enclosed in the latter, a default value must be specified.

Non-positional arguments are not enclosed and as such are always handled as
optional arguments. Unless stated otherwise, all non-positional arguments
requires a default value to be specified in the descriptor.


##### 1.4.1.1. Positional argument descriptors

A `positional-descriptor`, formed as `identifier type [value]` describes an
argument `identifier` of type `type` at the current position with a default
value of `value`.


##### 1.4.1.2. Positional list argument descriptors

A `positional-list-descriptor`, formed as `identifier... type [value]` describes
an argument `identifier` of type `[type]` with a default value of `value`. Here,
`identifier...` does not denote a list of items, and this is the actual
syntax of the descriptor. Furthermore, `[type]` does not denote an optional
item, but denotes an array of type `type`.

Only one `positional-list-descriptor` can be used for a command. Each positional
argument which does not fit a `positional-descriptor` are collected by the
single `positional-list-descriptor` if specified. They work similarly to
variadic functions is C or Python.


##### 1.4.1.3. Flag argument descriptors

A `flag-descriptor`, formed as `/identifier [default enabled]` describes a non
positional argument which can take two values :

 - When omitted, the flag takes the value of `default` ;
 - When specified, the flag takes the value of `enabled` ;

By default, `default` is set as `false` and `enabled` is set as `true`. When
`default` is specified in the descriptor, `enabled` must be specified too. Both
must have the same type.


##### 1.4.1.4. Option argument descriptors

A `option-descriptor`, formed as `name = type [default]` describes a
non-positional argument which can take any value of a given type.

`default` might be omitted when `type` supports a default value. Otherwise, it
must always be specified.


##### 1.4.1.5. List argument descriptors

A `list-descriptor`, formed as `name[] = type [default]` describes a
non-positional argument which is a list of values of a given type. `type`
describes the value of an item list, not of the list itself, thus the resulting
type of the argument is `[type]`.

When `default` is omitted, the default value of the argument is an empty list.


##### 1.4.1.6. Choice argument descriptors

A `choice-descriptor`, formed as `name -> v0,v1,...,vN default` describes a
non-positional argument which can take a value from a designated list. The value
list, denoted by `v0,v1,...,vN` takes at least two items, and all items must be
of the same type. Spaces are allowed around comma.

`default` must always be specified with a value from the value list.


##### 1.4.1.7. Named choice argument descriptors

A `named-choice-descriptor`, formed as `name -> n0=v0,n1=v1,...,nN=vN default`
describes a non-positional argument which can take a value from a designated
list. It works in the same way as a `choice-descriptor`, but it is defined by
a value-name pair list. When a command using a `named-choice-descriptor`, only
the name of the wanted value has to be specified. In the command itself, the
argument takes the value corresponding to the given name.

`default` must always be specified with a name from the name-value pair list.


##### 1.4.1.8. Multi-choice argument descriptors

A `multi-choice-descriptor`, formed as `name => v0,v1,...,vN [default]`
describes a non-positional argument which is a list where items are took from
a designated list. The value list, denoted by `v0,v1,...,vN` takes at least
two items and all the items must be of the same type. Spaces are allowed around
comma. The resulting type of the argument is a list.

When `default` is omitted, the default value of the argument is an empty list.
When specified, it must be a list composed of items available in the value list.


##### 1.4.1.9. Named multi-choice argument descriptors

A `named-multi-choice-descriptor`, formed as `name => n0=v0,n1=v1,...,nN=vN [default]`
describes a non-positional argument which is a list where items are took from a
designated list. It works the same way as a `multi-choice-descriptor`, but it is
defined by a value-name pair list. When a command using a
`named-multi-choice-descriptor`, only the names of the wanted value has to be
specified. In the command itself, the argument takes the values corresponding to
the given names.

When `default` is omitted, the default value of the argument is an empty list.
When specified, it must be a list composed of items available in the value list.


### 1.5. Literals

A literal is a token which represents a constant value known at compile time.


#### 1.5.1. Strings

String literals are divided into three flavors :

 - Regular strings, formed by using quotes (`" ... "`). It supports Unicode
   characters and escapes. Results in a value of type `str`.
 - Block strings, formed by using quoted curly braces (`"{ ... }"`). It supports
   Unicode characters, inner block strings and has no escape characters. Results
   in a value of type `str`.
 - Character strings, formed by using single quotes (`' C '`). It supports
   Unicode characters and escapes, but can receive only one character. Results
   in a value of type `u4` unless a type suffix is added (`u1`, `u2`, `u4`, `u8`).


#### 1.5.2. Numbers


##### 1.5.2.1. Integers

An integer can be represented in two ways :

- Baseless : `123456789`. In that case, the number is interpreted to have a base of
  10 (decimal).
- Based : `0 x 0123456789abcd...` In that case, the number is interpreted
  to have a custom base. `x` is a letter (lower-case or upper-case) denoting
  the number's base based on the index of the letter in the alphabet. For
  example, `b`/`B` would be a binary number, `p`/`P` would be a hexadecimal
  number, `g`/`G` would be a octal number.

Both formats accepts underscores `_` to add spacing and make the numbers more
readable.

It can be suffixed by `i1` (signed 1 byte-long number), `i2` (signed 2 byte-long
number), `i4` (signed 4 byte-long number), `i8` (signed 8 byte-long number),
`iL` (signed largest number), `u1` (`i1` but unsigned), `u2`, `u4`, `u8` or with
`uL`.

A "largest number" suffix just denotes a number large enough to contains a
memory address in it : `i4`/`u4` on 32bits systems and `i8`/`u8` on 64bits
systems.


##### 1.5.2.2. Floats

Floats are represented as `123.456`. They can be suffixed by `f4` (4 byte-long
float number) or `f8` (8 byte-long float number). The fractional part can be
omitted : `123.` is still a float. Furthermore, underscores `_` can be added
for readability.


### 1.6. Packages

Commands name can be paths with the form `identifier :: identifier :: ...`.
These path represents packages and sub-packages. By default, no packages
has its scope imported in the current file, meaning that calling anything from
that package requires its package prefix. The Neoshell infrastructure contains
commands to deal with packages and scope importation.


### 1.7. Variables

A variable has the form of `$identifier`. Variables can be accessed from their
whole scope, inside command invocations or expressions. Variables are initially
declared by some specialized CT commands.


### 1.8. Placeholders

The placeholder character `~` is mostly used for MAC commands. One of its
most common uses is with the `chain` macro :

```
chain
    $input-string
    { ~ strip-edges }
    { ~ replace "-" "_" }
    { ~ lower-case }
    { std::writeln ~ };

# this is the same as doing :
std::writeln !{ !{ !{ $input-string strip-edges } replace "-" "_" } lower-case };
```


## 2. Language's constructs

### 2.1. Commands

#### 2.1.1. Execution times

The Neoshell infrastructure has 3 execution modes for the commands. They do not
have different usage syntax (as in, they all follow the regular command
invocation syntax) but are declared using 3 different commands

- Compile time (shorten as CT) commands : These commands are non recursive and
  are used to manipulate the behaviors of the Neoshell infrastructure. These
  commands are declared by using the `def-ct-cmd` CT command.
- Macro (shorten as MAC) commands : These commands are recursive and are used
  to manipulate the surrounding abstract syntax tree (shorten as AST). These
  commands are declared by using the `def-macro` CT command.
- Runtime (shorten as RT) commands : These commands are regular commands and
  compiled to be ran in the resulting executable. These commands are
  declared by using the `def-cmd` CT command.


#### 2.1.2. Invocation algorithm

The commands are invoked based on their pattern and not only on their name :

```
def-cmd
    'puts
        <v str>
        ;
    { ... };

def-cmd
    'puts
        <v i32>
        ;
    { ... };

def-macro
    'puts
        <v generic>
        type = type { type-of $v }
        ;
    {
        switch $type
            str { mac-gen puts !{ convert $v str } }
            i32 { mac-gen puts !{ convert $v i32 } }
    };

puts "test"; # calls the first implementation
puts 1234; # calls the second implementation

puts "test" type = str; # calls the third implementation
# this invocation is a macro, and thus its result is equivalent to :
# puts "test";
```

For each invocation, the Neoshell infrastructure checks for each possible
commands that could fit that invocation. If only one is found, it is used. If
multiple commands are found, the most precise invocation will be used. Finally,
if no commands are found, an error is raised.

The precision of an invocation is defined by how much non-positional arguments
are omitted and by checking the types of all of the arguments. As shown in the
example above, the macro fits all the invocations with only one positional
argument. But in the case of a string, the first declaration of the command
is more precise and thus is being called instead of the macro.

On top of that, some rules are applied based on the execution time of the
command :

- A CT command can only calls other CT commands which aren't itself.
- A MAC command can only calls other MAC commands or RT commands.
- A RT command can only calls other RT commands or MAC commands.

Since RT commands can not have generic types as arguments or return value, this
algorithm allows for generic implementations of a command by using a routing
MAC command, which generates the correct code for the given arguments and
expected return value (if any).


## 3. Neoshell infrastructure commands

## 4. Neoshell standard library commands
