
# Neoshell

## Syntax

- Command : `command-name [arguments...];`
- Piped command : `generator |> consumer-generator |> consumer...;`
- Block : `{ commands... }`
  - Block as forced evaluation : `!{ ... }`
  - Block as forced argument : `&{ ... }`
- Expression : `( expr... )`
- Descriptor : `'descriptions...;`
- Literal :
  - Integer :
    - Decimal : `012_34[integer-suffix]` (type suffix is `i4` if not specified)
	- Hexadecimal : `0x0123_4[integer-suffix]` (type suffix is `u4` if not specified)
	- Binary : `0b0101_0[integer-suffix]` (type suffix is `u4` if not specified)
  - Float   : `01_234.012_34[float-suffix]` (type suffix is `f4` if not specified)
  - String  :
    - Regular : `"abcdef ..."`
	- Block   : `"{ anything... }"`
	- Char    : `'a'`


## Types

### Primitives

- Signed integers : `i1, i2, i4, i8`
- Unsigned integers : `u1, u2, u4, u8`
- Floats : `f4, f8`
- Logic : `bool`
- Empty : `void`, `generic`
- Block : `block`
- Descriptor : `descriptor` (only usable by compile-time/macro commands)


### Composed

- Array : `[T]`
- Stream : `<T>`
- String : `str`
- Enumeration : WIP
- Instantiated Command : WIP


## Command

Description : `name [argument-descriptors...]`


### Command names

- Scoped identifier : `scope::name`
  - Executed at run-time : `scope::name`
  - Executed as a macro : `scope::name!`
  - Executed at compile-time : `!scope::name`
- Variable : `$name`
- Placeholder (used by macros) : `~`


### Arguments

- Mandatory argument : `<description>`
  - `default`s might be omitted for mandatory arguments.
- Optional argument  : `[description]`
  - Multiple description can be put inside of the same optional block :
    `[desc0; desc1; ...; descN]`. If `desc0` is specified, all of the other
	arguments must be defined too.
- Static argument : `'word`
  - Defines the current positional argument as static. Useful for routing
    patterns in the command dispatch system.


- Input :
  - Description : `|>type`
  - Definition  : `some-cmd |> name`
- Output :
  - Description : `type|>`
  - Definition  : `name |> some-cmd`

- Positional :
  - Description : `name type [default]`
    - If `default` is not specified, the default value is the type's default
	  value.
  - Definition  : `literal | block | identifier | type`
- Positional list :
  - Description : `name... type [default]`
    - If `default` is not specified, the default value is an empty list.
	- A command can have at most 1 positional list argument.
  - Definition  : `(literal | block | identifier | type)*`
- Flags :
  - Description : `/name [default; enabled]`
    - If `default` is not specified, the default value is `false` and the
	  enabled value is `true`.
	- If `default` is specified, `enabled` must be specified.
	- The types of `default` and `enabled` must match.
  - Definition  : `/name`
- Options :
  - Description : `name = type [default]`
    - If `default` is not specified, the default value is the type's default
	  value.
  - Definition  : `name = value`
- Lists :
  - Description : `name[] = type [default]`
	- If `default` is not specified, the default value is an empty list.
  - Definition  : `name[] = v0,v1,...,vN`
- Choices :
  - Description : `name -> v0,v1,...,vN default`
    - `default` must be a value declared with a value included in
	  `v0,v1,...,vN`.
  - Definition  : `name -> vX`
- Named choices :
  - Description : `name -> n0=v0,n1=v1,...,nN=vN [default]`
    - `default` must be a value declared with a value included in
    `n0,n1,...,nN`.
  - Definition  : `name -> nX`
- Multi-choices :
  - Description : `name => v0,v1,...,vN [default]`
    - If `default` is not specified, the default value is an empty list.
  - Definition  : `name => vX,vY,...`
- Named multi-choices :
  - Description : `name => n0=v0,n1=v1,...,nN=vN [default]`
    - If `default` is not specified, the default value is an empty list.
  - Definition  : `name => nX,nY,...`


Choices, Named choices, Multi-choices and named multi-choices can be described
and defined in multiple chunks :

```
!defcmd
	'test-chunked-choices
		<choice -> A,B,C>
		<choice -> D,E,F>
		# equivalent to <choices -> A,B,C,D,E,F>.

		# spaces are allowed within descriptors :
		<types ->
			ubyte=u1,
			ushort=u2,
			uint=u4,
			ulong=u8>
		;
	{ not-yet-implemented!; };

```



## Builtin commands


### Globals

```
!defcmd
	<description descriptor>
	<code block>
	;
```

Defines a new command.
- `descriptor` : String describing the command's syntax.
- `code` : Code associated with that command.

----

### Arrays

### Streams
