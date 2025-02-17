---
description: >-
  This is a detailed description of the circom vitrual machine code 
---
# Circom Virtual Machine format

## Sections

## Type definitions

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

return

### Other operations

outs ??

error code

