
# Neoshell

## Types

### Primitives

- Signed integers : `i1, i2, i4, i8`
- Unsigned integers : `u1, u2, u4, u8`
- Floats : `f4, f8`
- Logic : `bool`
- Block : `block(T)` (T can be omitted to have a block returning nothing)

- Array : `[T]`
- Stream : `<T>`


### Composed

- Enumeration
- Struct
- Instantiated Command


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
    `[desc0, desc1, ..., descN]`. If `desc0` is specified, all of the other
	arguments must be defined too.

- Positional :
  - Description : `name type [default]`
    - If `default` is not specified, the default value is the type's default
	  value.
  - Definition  : `literal | block | identifier`
- Positional list :
  - Description : `name... type [default]`
    - If `default` is not specified, the default value is an empty list.
	- A command can have at most 1 positional list argument.
  - Definition  : `(literal | block | identifier)*`
- Flags :
  - Description : `/name [default, enabled]`
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
defcmd
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
defcmd
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
