---
description: >-
  This is a detailed description of the circom vitrual machine code 
---
# Circom Virtual Machine format

## Sections

## Type definitions

## Predefined registers

#### Function destination address

```
i64 destination
```
This is a local register availabe in the code of any function. This register contains the address given in the call to the fuction, when the call is made giving the address to place the result and the size of the result.

#### Function destination size

```
i64 destination_size
```
This is a local register availabe in the code of any function. This register contains the size given in the call to the fuction, when the call is made giving the address to place the result and the size of the result.

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

i64.extend_ff

ff.wrap_i64

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
set_cmp_input cidx sidx value
set_cmp_input_cnt cidx sidx value
set_cmp_input_run cidx sidx value
set_cmp_input_cnt_check cidx sidx value
```
cidx is i64
sidx is i64
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

```text

call <function-name> <parameters list>
ff.call <function-name> <parameters list>
i64.call <function-name> <parameters list>
```
The parameter list contains element of the form:
<value> | signal(indx,size) | subcmpsignal(cmp,indx,size) | i64.memory(indx,size) | ff.memory(indx,size) 

When the call has no result a return address and size are expected to be provided in the list of parameters. The address will be used by a return statement to place the result in the local memory of the callee.  Examples:

```text
x = ff.call $foo y signal(s,3) ff.memory(0,1)
```
which calls the function $foo with the value in the register y, the tree signals starting from the signal index in the register s and with one field value at position 0 of the local memory and leaves the result in the register x.

```text
call $mfoo r s y signal(s,3) ff.memory(0,1)
```
which calls the function $mfoo with the same parameter as above but the result will be stored at the local memory from address given by register r on.

#### Return operations

```text
ff.return <value>
i64.return <value>
```

It copies value to the address given in the call (it can be in the local memory of the callee or in signals)

`return <address-of-the-result> <size of return>`

It compares the given size with the size provided in the function call and copies as many element given in size from the provided address to the address given in the call (it can be in the local memory of the callee or in signals).




### Other operations

```text
get_template_id <i64 value>
```
Given a sub component number returns the template_id (i64) of the sub component as i64.
it can only be used in the code of a template.

```text
get_template_signal_position <i64 value> <i64 value>
```
Given a template id and a signal id returns the offset (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_template_signal_size <i64 value> <i64 value>
```
Given a template id and a signal id returns the size (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_template_signal_dimension <i64 value> <i64 value>
```
Given a template id and a signal id returns the number of dimensions of (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_template_signal_type <i64 value> <i64 value>
```
Given a template id and a signal id returns the type id (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_bus_signal_position <i64 value> <i64 value>
```
Given a bus id and a signal id returns the offset (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_bus_signal_size <i64 value> <i64 value>
```
Given a bus id and a signal id returns the size (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_bus_signal_dimension <i64 value> <i64 value>
```
Given a bus id and a signal id returns the number of dimensions of (i64) of the signal in such template.
It can only be used in the code of a template.

```text
get_bus_signal_type <i64 value> <i64 value>
```
Given a bus id and a signal id returns the type id (i64) of the signal in such template.
It can only be used in the code of a template.

outs ??

error code

