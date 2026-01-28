use crate::hxposed::{ProcessObject, RmdObject};
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::memory::*;
use bit_field::BitField;
use core::ops::{BitAnd, Shl};

#[derive(Debug)]
pub struct AllocateMemoryRequest {
    pub size: u32,
    pub memory_type: MemoryType,
}

#[derive(Debug)]
pub struct FreeMemoryRequest {
    pub obj: RmdObject
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemoryType {
    NonPagedPool,
    ContiguousPhysical,
}

impl Into<u64> for MemoryType {
    fn into(self) -> u64 {
        match self {
            MemoryType::NonPagedPool => 0,
            MemoryType::ContiguousPhysical => 1,
        }
    }
}

impl From<u64> for MemoryType {
    fn from(value: u64) -> Self {
        match value {
            0 => MemoryType::NonPagedPool,
            1 => MemoryType::ContiguousPhysical,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct MapVaToPaRequest {
    pub addr_space: ProcessObject,
    pub object: RmdObject,
    pub map_addr: u64,
    pub operation: MapOperation
}

#[derive(Debug)]
pub struct PageAttributeRequest {
    pub addr_space: ProcessObject,
    pub paging_type: PagingType,
    pub type_bits: u64,
    pub operation: PageAttributeOperation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MapOperation {
    Map,
    Unmap
}

impl MapOperation {
    pub const fn into_bits(self) -> u64 {
        match self {
            MapOperation::Map => 0,
            MapOperation::Unmap => 1
        }
    }

    pub const fn from_bits(value: u64) -> Self {
        match value {
            0 => MapOperation::Map,
            1 => MapOperation::Unmap,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PageAttributeOperation {
    Set,
    Get,
}

impl PageAttributeOperation {
    pub const fn into_bits(self) -> u64 {
        match self {
            PageAttributeOperation::Set => 0,
            PageAttributeOperation::Get => 1
        }
    }

    pub const fn from_bits(bits: u64) -> Self {
        match bits {
            0 => PageAttributeOperation::Set,
            1 => PageAttributeOperation::Get,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PagingType {
    Pml5(Va),
    Pml4(Va),
    Pdp(Va),
    Pd(Va),
    Pt(Va),
}

impl PagingType {
    pub fn from_raw_enum(object: u64, value: u64) -> Self {
        match object {
            0 => Self::Pml5(Va::from(value)),
            1 => Self::Pml4(Va::from(value)),
            2 => Self::Pdp(Va::from(value)),
            3 => Self::Pd(Va::from(value)),
            4 => Self::Pt(Va::from(value)),
            _ => unreachable!()
        }
    }

    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            PagingType::Pml5(x) => (0, x.into()),
            PagingType::Pml4(x) => (1, x.into()),
            PagingType::Pdp(x) => (2, x.into()),
            PagingType::Pd(x) => (3, x.into()),
            PagingType::Pt(x) => (4, x.into())
        }
    }
}

impl VmcallRequest for FreeMemoryRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::free_mem(),
            arg1: self.obj,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            obj: request.arg1
        }
    }
}

impl VmcallRequest for AllocateMemoryRequest {
    type Response = AllocateMemoryResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::mem_alloc(),
            arg1: self.size as _,
            arg2: self.memory_type.into(),
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            size: request.arg1 as _,
            memory_type: request.arg2.into(),
        }
    }
}

impl VmcallRequest for MapVaToPaRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::mem_map(),
            arg1: self.object,
            arg2: self.addr_space,
            arg3: self.map_addr,
            extended_arg1: self.operation.clone().into_bits() as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            object: request.arg1,
            addr_space: request.arg2,
            map_addr: request.arg3,
            operation: MapOperation::from_bits(request.extended_arg1 as _),
        }
    }
}

impl VmcallRequest for PageAttributeRequest {
    type Response = PageAttributeResponse;

    fn into_raw(self) -> HypervisorRequest {
        let args = self.paging_type.clone().into_raw_enum();
        HypervisorRequest {
            call: HypervisorCall::set_page_attr(),
            arg1: self.addr_space,
            arg2: self.operation.into_bits(),
            arg3: self.type_bits,
            // now you know why I don't like to use Into and From traits
            extended_arg1: args.0 as _,
            extended_arg2: args.1 as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr_space: request.arg1,
            operation: PageAttributeOperation::from_bits(request.arg2),
            type_bits: request.arg3,
            paging_type: PagingType::from_raw_enum(request.extended_arg1 as _, request.extended_arg2 as _),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pa(u64);

#[derive(Copy, Clone, Debug)]
pub struct Pfn(u64);

impl Pfn {
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub const fn into_bits(self) -> u64 {
        self.0
    }

    pub const fn into_phys(self) -> Pa {
        Pa(self.0 << 12)
    }
}

impl Into<Pa> for Pfn {
    fn into(self) -> Pa {
        Pa::from_pfn(self)
    }
}

impl Into<u64> for Pa {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<u64> for Pfn {
    fn into(self) -> u64 {
        self.0
    }
}

impl Pa {
    pub const fn into_pfn(self) -> Pfn {
        Pfn(self.0 >> 12)
    }

    pub const fn from_pfn(pfn: Pfn) -> Self {
        Self(pfn.0 << 12)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Va(u64);

impl Va {
    pub fn get_phys_offset(&self) -> u16 {
        self.0.get_bits(0..12) as u16
    }

    pub fn get_pt_index(&self) -> u16 {
        self.0.get_bits(12..21) as u16
    }

    pub fn get_pd_index(&self) -> u16 {
        self.0.get_bits(21..30) as u16
    }

    pub fn get_pdp_index(&self) -> u16 {
        self.0.get_bits(30..39) as u16
    }

    pub fn get_pml4_index(&self) -> u16 {
        self.0.get_bits(39..48) as u16
    }

    pub fn get_pml5_index(&self) -> u16 {
        self.0.get_bits(48..57) as u16
    }


    pub const fn get_page_addr(self) -> u64 {
        self.0 >> 12
    }

    pub const fn get_addr(self) -> u64 {
        self.0
    }
}

impl Into<u64> for Va {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for Va {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<u64> for Pa {
    fn from(v: u64) -> Self {
        Self(v)
    }
}
