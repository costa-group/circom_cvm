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

ff.store

### Signal memory operations 

get_signal  inx

get_cmp_signal cinx sinx

set_signal  inx value

set_cmp_input cinx sinx value

set_cmp_input_cnt cinx sinx value

set_cmp_input_run cinx sinx value

set_cmp_input_cnt_check cinx sinx value

### Control flow operations

loop

break

continue

if

else

end

#### Function call  operations

```text
ff.call <function-name> [<local-memory-address>, <size>] <parameters list>
i64.call <function-name> [<local-memory-address>, <size>] <parameters list>
```
The parameter list contains element of the form:
<value> | signal(indx,size) | subcmpsignal(cmp,indx,size) | i64.memory(indx,size) | ff.memory(indx,size) 

The return address (and size) is optional. If not incuded the result of the call should be assigned to  a register. Otherwise, the call is not assigned and the result is saved in the provided local memory address with (at most) the provided size. Note that if the call is not asigned then the first two parameters are the result address and the size. Examples:

```text
x = ff.call $foo y signal(s,3) ff.memory(0,1)
```
which calls the function $foo with the value in the register y, the tree signals starting from the signal index in the register s and with one field value at position 0 of the local memory and leaves the result in the register x.

```text
ff.call $mfoo r s y signal(s,3) ff.memory(0,1)
```
which calls the function $mfoo with the same parameter as above but the result will be stored at the local memory from address given by register r on.
Note taht the fact that the call is assigned or not determines whether the first data after the function name is the return address or not.

#### Return operations

```text
ff.return <value>
i32.return <value>
```

It copies value to the address given in the call (it can be in the local memory of the callee or in signals)

`return <address-of-the-result> <size of return>`

It compares the given size with the size provided in the function call and copies as many element given in size from the provided address to the address given in the call (it can be in the local memory of the callee or in signals).




### Other operations

outs ??

error code

