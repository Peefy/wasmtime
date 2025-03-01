//! Representation of Cranelift IR functions.

mod atomic_rmw_op;
mod builder;
pub mod condcodes;
pub mod constant;
pub mod dfg;
pub mod dynamic_type;
pub mod entities;
mod extfunc;
mod extname;
pub mod function;
mod globalvalue;
mod heap;
pub mod immediates;
pub mod instructions;
pub mod jumptable;
pub(crate) mod known_symbol;
pub mod layout;
pub(crate) mod libcall;
mod memflags;
mod progpoint;
mod sourceloc;
pub mod stackslot;
mod table;
mod trapcode;
pub mod types;

#[cfg(feature = "enable-serde")]
use serde::{Deserialize, Serialize};

pub use crate::ir::atomic_rmw_op::AtomicRmwOp;
pub use crate::ir::builder::{
    InsertBuilder, InstBuilder, InstBuilderBase, InstInserterBase, ReplaceBuilder,
};
pub use crate::ir::constant::{ConstantData, ConstantPool};
pub use crate::ir::dfg::{DataFlowGraph, ValueDef};
pub use crate::ir::dynamic_type::{dynamic_to_fixed, DynamicTypeData, DynamicTypes};
pub use crate::ir::entities::{
    Block, Constant, DynamicStackSlot, DynamicType, FuncRef, GlobalValue, Heap, Immediate, Inst,
    JumpTable, SigRef, StackSlot, Table, UserExternalNameRef, Value,
};
pub use crate::ir::extfunc::{
    AbiParam, ArgumentExtension, ArgumentPurpose, ExtFuncData, Signature,
};
pub use crate::ir::extname::{ExternalName, UserExternalName, UserFuncName};
pub use crate::ir::function::{DisplayFunctionAnnotations, Function};
pub use crate::ir::globalvalue::GlobalValueData;
pub use crate::ir::heap::{HeapData, HeapStyle};
pub use crate::ir::instructions::{
    InstructionData, InstructionImms, Opcode, ValueList, ValueListPool, VariableArgs,
};
pub use crate::ir::jumptable::JumpTableData;
pub use crate::ir::known_symbol::KnownSymbol;
pub use crate::ir::layout::Layout;
pub use crate::ir::libcall::{get_probestack_funcref, LibCall};
pub use crate::ir::memflags::{Endianness, MemFlags};
pub use crate::ir::progpoint::{ExpandedProgramPoint, ProgramOrder, ProgramPoint};
pub use crate::ir::sourceloc::RelSourceLoc;
pub use crate::ir::sourceloc::SourceLoc;
pub use crate::ir::stackslot::{
    DynamicStackSlotData, DynamicStackSlots, StackSlotData, StackSlotKind, StackSlots,
};
pub use crate::ir::table::TableData;
pub use crate::ir::trapcode::TrapCode;
pub use crate::ir::types::Type;
pub use crate::value_label::LabelValueLoc;

use crate::entity::{entity_impl, PrimaryMap, SecondaryMap};

/// Map of jump tables.
pub type JumpTables = PrimaryMap<JumpTable, JumpTableData>;

/// Source locations for instructions.
pub(crate) type SourceLocs = SecondaryMap<Inst, RelSourceLoc>;

/// Marked with a label value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ValueLabel(u32);
entity_impl!(ValueLabel, "val");

/// A label of a Value.
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ValueLabelStart {
    /// Source location when it is in effect
    pub from: RelSourceLoc,

    /// The label index.
    pub label: ValueLabel,
}

/// Value label assignements: label starts or value aliases.
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum ValueLabelAssignments {
    /// Original value labels assigned at transform.
    Starts(alloc::vec::Vec<ValueLabelStart>),

    /// A value alias to original value.
    Alias {
        /// Source location when it is in effect
        from: RelSourceLoc,

        /// The label index.
        value: Value,
    },
}
