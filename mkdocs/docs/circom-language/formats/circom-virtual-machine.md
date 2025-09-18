---
description: >-
  This is a detailed description of the circom vitrual machine code 
---
# Circom Virtual Machine format


## Prime definition

%%prime 21888242871839275222246405745257275088548364400416034343698204186575808495617

## Global memory of signals size

It provides the total number of signals in the circuit.

```text
%%signals 5
```

## Size of the heap of components.

It provides the total number of components in the circuit.

```text
%%components_heap <number>
```

##Definition of the component creation mode

It can be `implicit` or `explicit`.

When implicit, the offsets of the components are assigned following a depth-first traversal of the tree structure of the circuit given by the initial template and the template definitions. 

When explicit a description of the offsets is explicitly given for each component assigning a number to each component:

component #n #template #offset '['subcmp number list']'

should be consistent with all other definitions

This is still under construction.

In both cases, the global number of each signal is obtained by adding the offset to the local number of the signal in the template. 

```text
%%components implicit
```

## Type definitions

```text
%%type $name [dimensions_list] type_list
```
If dimension_list (sequence of numbers) is non-empty then typelist (sequence of numbers) has a single element.
All types are numbered from 1 on (ff is $0) 

```text
%%type $name [dimensions_list] type_list
```

## Definition of the Initial template

```text
%%start TempName
```

## Witness list
It provides the map from witness number (from 0 on) to signal number

```text
%%witness 0 1 2 4 5 9 11
```

## Templates 
Every template has a local memory for i64 values and a local memory for ff. Each instance of a template can only access to its local memories.
Templates have their signals numbered from 1 on, and when instantiated as a component and executed, the corresponding offset is added.
A template can only access its own signals (with getters and setters) and the signals of its immediate subcomponents, with setters for the inputs and getters for the outputs).

Templates are not explicitly executed, there are instructions to set inputs and run the template maybe after checking whether all inputs are set.

## Functions

The header of a function is of the form:

```text
%%function name output inputs
code

```
where
output is either the empty list [] if the output is void or a list [ff] or [i64] if it returns an ff element or an i64 element respectively.
inputs is a list of simple (ff or i64) or array type parameters. The format to provide each inputs is: simple type (ff or i64) followed by the number of dimensions n and a sequence of n element providing the size of each dimension. For instance:

```text
%%function f1_0 [ff] [ ff 2 2 2 ff 0]

or,

```text
%%function f1_0 [] [ i64 0 i64 0 ff 2 2 2 ff 0 i64 0]
```

Every function has a local memory for i64 values and a local memory for ff. There are no side-effects when calling a function (parameters are passed by value).
When a function has no output it is because it is going to copy n elements (e.g. an array) from the local memory of the called function to the local memory of the callee (using one the available return operations), which can be another function or a component (template instantiation).

Parameters are passed through the local memories of the called function and are placed in the upper position of these memories. Parameters that contain i64 (resp. ff) values are placed in the i64 (resp. ff) memory in the same relative order they appear. For instance in the second example above we have three parameters in the i64 memory and two in the ff memory. The first parameter is in position 0 of the i64 memory, the second one is in position 1 of the i64 memory, the third one starts in position 0 of the ff memory, the forth one is in position 4 of the ff memory (as the size of the previous one is 4) and the fifth parameter is in position 3 of the i64 memory.

Functions returning an ff or i64 are called by assigning the result to a register and when there is no output the call is not assigned.

Altogether, all memories are local and we have that the callee writes in the local memory of the called when passing the parameter and the called function may write in the callee memory when returning an array of elements.

## Comment lines

;; comment

## Instruction Set (opcodes)

### Arithmetic

i64.add

ff.add

i64.sub

ff.sub

i64.mul

ff.mul

i64.div

i64.rem

ff.idiv

ff.div

ff.rem

ff.pow

64.pow

### Relational operations

i64.gt

ff.gt

i64.ge

ff.ge

i64.lt

ff.lt

i64.le

ff.le

i64.eq

ff.eq

i64.neq

ff.neq

i64.eqz

ff.eqz

### Boolean operations

i64.and

i64.or

ff.and

ff.or

### Bit operations:

64.shr

i64.shl

ff.shr

ff.shl

i64.band  

i64.bor

i64.bxor

i64.bnot

ff.band

ff.bor

ff.bxor

ff.bnot


### Conversions

ff.extend_i64

i64.wrap_ff

### Variable Memory operations 

i64.load

ff.load

i64.store

```text
ff.store addr value
```
addr is i64
value is ff

```text
ff.mstore addr1 addr2 size
```
addr1 is i64: destination address in variable memory
addr2 is i64: source address in variable memory
size is i64: : number of elements to be copied

```text
ff.mstore_from_signal addr idx size
```
addr is i64: destination address in variable memory
sidx is i64: source signal (local)
size is i64: : number of elements to be copied

```text
ff.mstore_from_cmp_signal addr idx size
```
addr is i64: destination address in variable memory
cidx is i64: source component
sidx is i64: source signal in component
size is i64: : number of elements to be copied

### Signal memory operations 

```text
get_signal  idx
```
idx is i64
returns an ff

```text
get_cmp_signal cidx sidx
```
cidx is i64
sidx is i64
returns an ff

```text
set_signal  idx value
```
idx is i64
value is ff

```text
mset_signal  idx1 idx2 size
```
idx1 is i64: destination
idx2 is i64: source (signal)
size: number of elements to be set

```text
mset_signal_from_memory  idx addr size
```
idx is i64: destination signal
addr is i64: address in variable memory
size: number of elements to be set

```text
mset_signal_from_cmp idx cidx sidx value
```
idx is i64: destination signal
cidx is i64: source component
sidx is i64: source signal
size: number of elements to be set

```text
set_cmp_input cidx sidx value
set_cmp_input_cnt cidx sidx value
set_cmp_input_run cidx sidx value
set_cmp_input_cnt_check cidx sidx value
```
cidx is i64: destination component
sidx is i64: destination signal in component
value is ff

```text
mset_cmp_input cidx sidx1 sidx2 size
mset_cmp_input_cnt cidx sidx1 sidx2 size
mset_cmp_input_run cidx sidx1 sidx2 size
mset_cmp_input_cnt_check cidx sidx1 sidx2 size
```
cidx is i64: destination component
sidx1 is i64: destination signal
sidx2 is i64: source signal (local)
size: number of elements to be set

```text
mset_cmp_input_from_cmp cidx1 sidx1 cidx2 sidx2 size
mset_cmp_input_from_cmp_cnt cidx1 sidx1 cidx2 sidx2 size
mset_cmp_input_from_cmp_run cidx1 sidx1 cidx2 sidx2 size
mset_cmp_input_from_cmp_cnt_check cidx1 sidx1 cidx2 sidx2 size
```
cidx1 is i64: destination component
sidx1 is i64: destination signal
cidx2 is i64: source component
sidx2 is i64: source signal
size: number of elements to be set

```text
mset_cmp_input_from_memory cidx sidx addr size
mset_cmp_input_from_memory_cnt cidx sidx addr size
mset_cmp_input_from_memory_run cidx sidx addr size
mset_cmp_input_from_memory_cnt_check cidx sidx addr size
```
cidx is i64: destination component
sidx is i64: destination signal
addr is i64: address in variable memory
size: number of elements to be set

### Control flow operations

loop

break

continue

if

else

end

#### Function call  operations

Function calls with result passed using registers:

```text
ff.call <function-name> <parameters list>
i64.call <function-name> <parameters list>
```
The parameter list contains element of the form:
<value> | signal(indx,size) | subcmpsignal(cmp,indx,size) | i64.memory(indx,size) | ff.memory(indx,size) 

The result must be assigned to a register. Example:

```text
x = ff.call $foo y signal(s,3) ff.memory(0,1)
```
which calls the function $foo with the value in the register y, the tree signals starting from the signal index in the register s and with one field value at position 0 of the local memory and leaves the result in the register x.


Function calls with results passed using the local memories:

```text
ff.mcall <function-name> <parameters list>
i64.mcall <function-name> <parameters list>
```


```text
ff.mcall $mfoo r s y signal(s,3) ff.memory(0,1)
```
which calls the function $mfoo with parameters similar to the avobe one but the result will be given using the corresponding return operation that stored the results in some address of the local memory (with this address normally given by a parameter together with the number of elements to be copied).

#### Return operations

Return operations for functions with a result returned in a register:

```text
ff.return <value>
i64.return <value>
```
It returns value to the caller (<values> is i64 or ff, depending of the return operation used).


Return operations for functions with a result returned in a register:

ff.mreturn <address-of-caller-mamory> <address-of-the-callee-memory> <size-of-return>
i64.mreturn <address-of-caller-mamory> <address-of-the-callee-memory> <size-of-return>
```
The three values are i64, and the first and the second are memory adresses in the ff memories in the first case and in the i64 memories in the second one.

It copies as many elements as given in size-of-return from the provided address-of-the-callee-memory to the given address-of-caller-mamory.



### Other operations

The template ids are assigned following the Template definition order, starting from $0$.

The template output or input signal ids are given one by one from $0$ starting first with the outputs and then the inputs following the template definition given in the CVM code. There is an id per input/output signal (no matter if it is a single signal or an array of signals), namely if a template has 2 output and 3 inputs, the first output has id $0$ and the second one id $1$, the first input has id $2$, the second one $3$ and the third one $4$.

The bus ids are assigned following the Bus definition order, starting from $0$.

The bus field signal ids are given one by one starting from $0$ to the fields of the bus following the bus definition given in the CVM code. There is an id per field signal (no matter if it is a single signal or an array of signals), namely if a bus has 4 fields, the first one has id $0$, the second one $1$ and so one.

```text
get_template_id <i64 value>
```
Given a sub component number returns the template_id (i64) of the sub component as i64.
it can only be used in the code of a template.

```text
get_template_signal_position <i64 value> <i64 value>
```
Given a template id and an output or input signal id returns the offset (i64) of the signal in such template.
It can only be used in the code of a template.


For instance

```text
get_template_signal_size <i64 value> <i64 value>
```
Given a template id and a signal id returns the size (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_template_signal_dimension <i64 value> <i64 value> <i64 value>
```
Given a template id and an output or input signal id and a dimension returns the size (i64) of this dimension of the signal in such template .
It can only be used in the code of a template.

```text
get_template_signal_type <i64 value> <i64 value>
```
Given a template id and an output or input signal id returns the type id (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_bus_signal_position <i64 value> <i64 value>
```
Given a bus id and a bus field signal id returns the offset (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_bus_signal_size <i64 value> <i64 value>
```
Given a bus id and a bus field signal id returns the size (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_bus_signal_dimension <i64 value> <i64 value> <i64 value>
```
Given a bus id, a bus field signal id and a dimension returns the size (i64) of this dimension of the signal in such template.
It can only be used in the code of a template.

```text
get_bus_signal_type <i64 value> <i64 value>
```
Given a bus id and a bus field signal id returns the type id (i64) of the signal in such template.
It can only be used in the code of a template.

outs ??

error code

