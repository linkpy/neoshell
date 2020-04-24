
# Neoshell Language Specification

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
(1 + { sin (2 * $PI * $x); })
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
single `positional-list-descriptor` if specified.


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

When `default` is omitted, the default argument of the argument is an empty list.
When specified, it must be a list composed of items available in the value list.


##### 1.4.1.9. Named multi-choice argument descriptors




### 1.5. Literals

#### 1.5.1. Strings

#### 1.5.2. Numbers

### 1.6. Packages


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
