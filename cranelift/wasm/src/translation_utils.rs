//! Helper functions and structures for the translation.
use crate::environ::TargetEnvironment;
use crate::WasmResult;
use core::u32;
use cranelift_codegen::ir;
use cranelift_frontend::FunctionBuilder;
use wasmparser::{FuncValidator, WasmModuleResources};

/// Get the parameter and result types for the given Wasm blocktype.
pub fn blocktype_params_results<T>(
    validator: &FuncValidator<T>,
    ty: wasmparser::BlockType,
) -> (BlockTypeParamsOrReturns, BlockTypeParamsOrReturns)
    where
        T: WasmModuleResources,
{
    use wasmparser::BlockType;

    match ty {
        BlockType::Empty => (
            BlockTypeParamsOrReturns::Empty,
            BlockTypeParamsOrReturns::Empty,
        ),
        BlockType::Type(ty) => (
            BlockTypeParamsOrReturns::Empty,
            BlockTypeParamsOrReturns::One(ty),
        ),
        BlockType::FuncType(ty_index) => {
            let ty = validator
                .resources()
                .sub_type_at(ty_index)
                .expect("should be valid")
                .unwrap_func();

            (
                BlockTypeParamsOrReturns::Many(ty.params(), 0),
                BlockTypeParamsOrReturns::Many(ty.results(), 0),
            )
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum BlockTypeParamsOrReturns<'a> {
    Empty,
    One(wasmparser::ValType),
    Many(&'a [wasmparser::ValType], usize),
}

impl<'a> Iterator for BlockTypeParamsOrReturns<'a> {
    type Item = wasmparser::ValType;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            BlockTypeParamsOrReturns::Empty => None,
            BlockTypeParamsOrReturns::One(ty) => {
                let ty = *ty;
                *self = Self::Empty;
                Some(ty)
            }
            BlockTypeParamsOrReturns::Many(slice, offset) => {
                let val = *slice.get(*offset)?;
                *offset += 1;
                Some(val)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            BlockTypeParamsOrReturns::Empty => (0, Some(0)),
            BlockTypeParamsOrReturns::One(_) => (1, Some(1)),
            BlockTypeParamsOrReturns::Many(slice, _) => (slice.len(), Some(slice.len())),
        }
    }
}

impl<'a> ExactSizeIterator for BlockTypeParamsOrReturns<'a> {}

/// Create a `Block` with the given Wasm parameters.
pub fn block_with_params<PE: TargetEnvironment + ?Sized>(
    builder: &mut FunctionBuilder,
    params: impl IntoIterator<Item = wasmparser::ValType>,
    environ: &PE,
) -> WasmResult<ir::Block> {
    let block = builder.create_block();
    for ty in params {
        match ty {
            wasmparser::ValType::I32 => {
                builder.append_block_param(block, ir::types::I32);
            }
            wasmparser::ValType::I64 => {
                builder.append_block_param(block, ir::types::I64);
            }
            wasmparser::ValType::F32 => {
                builder.append_block_param(block, ir::types::F32);
            }
            wasmparser::ValType::F64 => {
                builder.append_block_param(block, ir::types::F64);
            }
            wasmparser::ValType::Ref(rt) => {
                let hty = environ.convert_heap_type(rt.heap_type());
                builder.append_block_param(block, environ.reference_type(hty));
            }
            wasmparser::ValType::V128 => {
                builder.append_block_param(block, ir::types::I8X16);
            }
        }
    }
    Ok(block)
}

/// Turns a `wasmparser` `f32` into a `Cranelift` one.
pub fn f32_translation(x: wasmparser::Ieee32) -> ir::immediates::Ieee32 {
    ir::immediates::Ieee32::with_bits(x.bits())
}

/// Turns a `wasmparser` `f64` into a `Cranelift` one.
pub fn f64_translation(x: wasmparser::Ieee64) -> ir::immediates::Ieee64 {
    ir::immediates::Ieee64::with_bits(x.bits())
}

/// Special VMContext value label. It is tracked as 0xffff_fffe label.
pub fn get_vmctx_value_label() -> ir::ValueLabel {
    const VMCTX_LABEL: u32 = 0xffff_fffe;
    ir::ValueLabel::from_u32(VMCTX_LABEL)
}
