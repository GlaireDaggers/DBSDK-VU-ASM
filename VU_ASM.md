# Vertex Unit
The "Vertex Unit" (VU) is responsible for processing vertex data in Dreambox. VU programs may be up to 64 instructions long, and feature a compact instruction set with no branching.

All inputs, registers, and outputs are 128-bits wide, storing a 4-component floating point vector at 32 bits per component ("vec4").

## VU inputs, registers, & outputs
The user may supply up to 8 "vertex input" slots, each of which can be configured with an offset & format which defines how to load & convert the source data into the given vec4 input slot.

There are 4 fixed output slots. These are:
- pos (clip-space position)
- tex (texture coords - xy are sent to texture unit 0, and zw are sent to texture unit 1)
- col (vertex color)
- ocol (vertex offset color)

There are 16 vec4 registers available, numbered r0..r15

There are 16 vec4 "constant" slots available, which can be written to by the user in order to provide data to VU programs.

## VU opcodes
- ld (dst reg) (input slot)
  - Load a value from the given input slot number into the given register
- ldc (dst reg) (input slot)
  - Load a value from the given constant slot number into the given register
- st (dst output) (src register)
  - Store the source register into the given output slot
- add (dst reg) (src reg)
  - `dst = dst + src`
- sub (dst reg) (src reg)
  - `dst = dst - src`
- mul (dst reg) (src reg)
  - `dst = dst * src`
- div (dst reg) (src reg)
  - `dst = dst / src`
- dot (dst reg) (src reg)
  - `dst = [dot(dst, src), 0, 0, 0]`
- abs (dst reg) (src reg)
  - `dst = abs(src)`
- sign (dst reg) (src reg)
  - `dst = sign(src)`
- sqrt (dst reg) (src reg)
  - `dst = sqrt(src)`
- pow (dst reg) (src reg)
  - `dst = pow(dst, src)`
- exp (dst reg) (src reg)
  - `dst = exp(src)`
- log (dst reg) (src reg)
  - `dst = log(src)`
- min (dst reg) (src reg)
  - `dst = min(dst, src)`
- max (dst reg) (src reg)
  - `dst = max(dst, src)`
- sin (dst reg) (src reg)
  - `dst = sin(src)`
- cos (dst reg) (src reg)
  - `dst = cos(src)`
- tan (dst reg) (src reg)
  - `dst = tan(src)`
- asin (dst reg) (src reg)
  - `dst = asin(src)`
- acos (dst reg) (src reg)
  - `dst = acos(src)`
- atan (dst reg) (src reg)
  - `dst = atan(src)`
- atan2 (dst reg) (src reg)
  - `dst = atan2(dst, src)`
- shf (dst reg) (src reg) (swizzle) (mask)
  - Shuffles the elements of the source register into the destination register, with a 4-bit mask value to indicate which destination registers to change. Example: `shf r0 r1 xyzw 0b0011` would copy the x and y elements of r1 into r0, while leaving z and w unchanged.
- mulm (dst reg) (src reg)
  - Transforms dst register with a 4x4 column-major matrix encoded in four sequential registers starting at src (for example, given "r1" as the source operand, the matrix columns are expected to be stored in r1, r2, r3, and r4)
- end
  - Terminates execution of the VU program

## Example VU program
```
// Simple VU program which multiplies input position with a transform matrix & stores into the output slots

ld r0 0     // slot 0 = position
ld r1 1     // slot 2 = texcoord
ld r2 2     // slot 3 = color
ld r3 3     // slot 4 = ocolor

ldc r4 0    // constant 0 = transform column 0
ldc r5 1    // constant 1 = transform column 1
ldc r6 2    // constant 2 = transform column 2
ldc r7 3    // constant 3 = transform column 3

mulm r0 r4  // transform position with matrix

st pos r0
st tex r1
st col r2
st ocol r3
```
