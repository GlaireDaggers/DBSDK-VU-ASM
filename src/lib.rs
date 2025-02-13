use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, Lit};

const MAX_INPUT_SLOTS: u32 = 8;
const MAX_CONSTANT_SLOTS: u32 = 16;

struct VUProgram {
    instructions: Vec<u32>
}

fn parse_reg(id: Ident) -> syn::Result<u32> {
    let str = id.to_string();
    match str.as_str() {
        "r0" => Ok(0),
        "r1" => Ok(1),
        "r2" => Ok(2),
        "r3" => Ok(3),
        "r4" => Ok(4),
        "r5" => Ok(5),
        "r6" => Ok(6),
        "r7" => Ok(7),
        "r8" => Ok(8),
        "r9" => Ok(9),
        "r10" => Ok(10),
        "r11" => Ok(11),
        "r12" => Ok(12),
        "r13" => Ok(13),
        "r14" => Ok(14),
        "r15" => Ok(15),
        _ => Err(syn::Error::new(id.span().into(), "Invalid register identifier"))
    }
}

fn parse_output(id: Ident) -> syn::Result<u32> {
    let str = id.to_string();
    match str.as_str() {
        "pos" => Ok(0),
        "tex" => Ok(1),
        "col" => Ok(2),
        "ocol" => Ok(3),
        _ => Err(syn::Error::new(id.span().into(), "Invalid vertex output identifier"))
    }
}

fn parse_sub(ch: char, id: Ident) -> syn::Result<u32> {
    match ch {
        'x' => Ok(0),
        'y' => Ok(1),
        'z' => Ok(2),
        'w' => Ok(3),
        'r' => Ok(0),
        'g' => Ok(1),
        'b' => Ok(2),
        'a' => Ok(3),
        _ => Err(syn::Error::new(id.span().into(), "Invalid shuffle subscript"))
    }
}

fn parse_swizzle(id: Ident) -> syn::Result<(u32, u32, u32, u32)> {
    let str = id.to_string();
    if str.len() != 4 {
        return Err(syn::Error::new(id.span().into(), "Invalid shuffle subscript"));
    }

    let x = parse_sub(str.chars().nth(0).unwrap(), id.clone())?;
    let y = parse_sub(str.chars().nth(1).unwrap(), id.clone())?;
    let z = parse_sub(str.chars().nth(2).unwrap(), id.clone())?;
    let w = parse_sub(str.chars().nth(3).unwrap(), id.clone())?;

    return Ok((x, y, z, w))
}

fn encode_instr(op: u32, d: u32, s: u32, sx: u32, sy: u32, sz: u32, sw: u32, m: u32) -> u32 {
    (op & 0x3F)         |
    ((d & 0xF) << 6)    |
    ((s & 0xF) << 10)   |
    ((sx & 3) << 14)    |
    ((sy & 3) << 16)    |
    ((sz & 3) << 18)    |
    ((sw & 3) << 20)    |
    ((m & 0xF) << 22)
}

impl Parse for VUProgram {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut instr = Vec::new();

        while !input.is_empty() {
            let op = input.parse::<Ident>()?;
            let op_str = op.to_string();

            match op_str.as_str() {
                "ld" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src_token = input.parse::<Lit>()?;
                    let src = match src_token.clone() {
                        Lit::Int(v) => v,
                        _ => {
                            return Err(syn::Error::new(src_token.span().into(), "Invalid argument"));
                        }
                    }.base10_parse::<u32>()?;

                    if src >= MAX_INPUT_SLOTS {
                        return Err(syn::Error::new(src_token.span().into(), "Input vertex slot index out of range"));
                    }

                    instr.push(encode_instr(0, dst, src, 0, 0, 0, 0, 0));
                }
                "st" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_output(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(1, dst, src, 0, 0, 0, 0, 0));
                }
                "ldc" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src_token = input.parse::<Lit>()?;
                    let src = match src_token.clone() {
                        Lit::Int(v) => v,
                        _ => {
                            return Err(syn::Error::new(src_token.span().into(), "Invalid argument"));
                        }
                    }.base10_parse::<u32>()?;

                    if src >= MAX_CONSTANT_SLOTS {
                        return Err(syn::Error::new(src_token.span().into(), "Input constant slot index out of range"));
                    }

                    instr.push(encode_instr(2, dst, src, 0, 0, 0, 0, 0));
                }
                "add" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(3, dst, src, 0, 0, 0, 0, 0));
                }
                "sub" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(4, dst, src, 0, 0, 0, 0, 0));
                }
                "mul" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(5, dst, src, 0, 0, 0, 0, 0));
                }
                "div" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(6, dst, src, 0, 0, 0, 0, 0));
                }
                "dot" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(7, dst, src, 0, 0, 0, 0, 0));
                }
                "abs" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(8, dst, src, 0, 0, 0, 0, 0));
                }
                "sign" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(9, dst, src, 0, 0, 0, 0, 0));
                }
                "sqrt" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(10, dst, src, 0, 0, 0, 0, 0));
                }
                "pow" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(11, dst, src, 0, 0, 0, 0, 0));
                }
                "exp" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(12, dst, src, 0, 0, 0, 0, 0));
                }
                "log" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(13, dst, src, 0, 0, 0, 0, 0));
                }
                "min" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(14, dst, src, 0, 0, 0, 0, 0));
                }
                "max" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(15, dst, src, 0, 0, 0, 0, 0));
                }
                "sin" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(16, dst, src, 0, 0, 0, 0, 0));
                }
                "cos" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(17, dst, src, 0, 0, 0, 0, 0));
                }
                "tan" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(18, dst, src, 0, 0, 0, 0, 0));
                }
                "asin" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(19, dst, src, 0, 0, 0, 0, 0));
                }
                "acos" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(20, dst, src, 0, 0, 0, 0, 0));
                }
                "atan" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(21, dst, src, 0, 0, 0, 0, 0));
                }
                "atan2" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(22, dst, src, 0, 0, 0, 0, 0));
                }
                "shf" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    let sw = input.parse::<Ident>()?;
                    let sw = parse_swizzle(sw)?;

                    let mask_token = input.parse::<Lit>()?;
                    let mask = match mask_token.clone() {
                        Lit::Int(v) => v,
                        _ => {
                            return Err(syn::Error::new(mask_token.span().into(), "Invalid argument"));
                        }
                    }.base10_parse::<u32>()?;

                    if mask & 0b1111 != mask {
                        return Err(syn::Error::new(mask_token.span().into(), "Mask value out of range (expected 4-bit value)"));
                    }

                    instr.push(encode_instr(23, dst, src, sw.0, sw.1, sw.2, sw.3, mask));
                }
                "mulm" => {
                    let dst = input.parse::<Ident>()?;
                    let dst = parse_reg(dst)?;

                    let src = input.parse::<Ident>()?;
                    let src = parse_reg(src)?;

                    instr.push(encode_instr(24, dst, src, 0, 0, 0, 0, 0));
                }
                "end" => {
                    instr.push(0x3F);
                }
                _ => {
                    return Err(syn::Error::new(op.span().into(), "Invalid opcode"));
                }
            };
        }

        // terminate with "end" instruction
        if instr.len() < 64 {
            instr.push(0x3F);
        }
        else if instr.len() > 64 {
            return Err(syn::Error::new(input.span().into(), "Program too large (must be no more than 64 instructions)"));
        }
        
        Ok(VUProgram {
            instructions: instr
        })
    }
}

#[proc_macro]
/// Takes Dreambox VU assembly as input and produces an inline array of encoded VU instructions
pub fn vu_asm(input: TokenStream) -> TokenStream {
    let input_stream = parse_macro_input!(input as VUProgram);
    let instr = input_stream.instructions;

    quote! {
        [
            #(#instr),*
        ]
    }.into()
}