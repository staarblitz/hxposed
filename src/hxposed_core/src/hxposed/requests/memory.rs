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
}

#[derive(Debug)]
pub struct PageAttributeRequest {
    pub addr_space: ProcessObject,
    pub paging_type: PagingType,
    pub attributes: PageAttributes,
    pub operation: PageAttributeOperation,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PageAttributes {
    Present(bool),
    Writable(bool),
    UserAccessible(bool),
    WriteThrough(bool),
    NoCache(bool),
    Accessed(bool),
    Dirty(bool),
    Large(bool),
    CopyOnWrite(bool),
    SoftwareWrite(bool),
    Global(bool),
    Pfn(u64),
    ExecuteDisable(bool),
}

impl From<u64> for PageAttributeOperation {
    fn from(attr: u64) -> Self {
        match attr {
            0 => Self::Get,
            1 => Self::Set,
            _ => unreachable!(),
        }
    }
}

impl PageAttributes {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            PageAttributes::Present(x) => (1, x as _),
            PageAttributes::Writable(x) => (2, x as _),
            PageAttributes::UserAccessible(x) => (3, x as _),
            PageAttributes::WriteThrough(x) => (4, x as _),
            PageAttributes::NoCache(x) => (5, x as _),
            PageAttributes::Accessed(x) => (6, x as _),
            PageAttributes::Large(x) => (7, x as _),
            PageAttributes::Global(x) => (8, x as _),
            PageAttributes::Pfn(x) => (9, x as _),
            PageAttributes::ExecuteDisable(x) => (10, x as _),
            PageAttributes::Dirty(x) => (11, x as _),
            PageAttributes::CopyOnWrite(x) => (12, x as _),
            PageAttributes::SoftwareWrite(x) => (13, x as _),
        }
    }

    pub fn from_raw_enum(object: u64, value: u64) -> Self {
        match object {
            1 => PageAttributes::Present(value == 1),
            2 => PageAttributes::Writable(value == 1),
            3 => PageAttributes::UserAccessible(value == 1),
            4 => PageAttributes::WriteThrough(value == 1),
            5 => PageAttributes::NoCache(value == 1),
            6 => PageAttributes::Accessed(value == 1),
            7 => PageAttributes::Large(value == 1),
            8 => PageAttributes::Global(value == 1),
            9 => PageAttributes::Pfn(value),
            10 => PageAttributes::ExecuteDisable(value == 1),
            11 => PageAttributes::Dirty(value == 1),
            12 => PageAttributes::CopyOnWrite(value == 1),
            13 => PageAttributes::SoftwareWrite(value == 1),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u64)]
pub enum PageAttributeOperation {
    Set,
    Get,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PagingType {
    Pml5(u16),
    Pml4(u16, u16),
    Pdp(u16, u16, u16),
    Pd(u16, u16, u16, u16),
    Pt(u16, u16, u16, u16, u16),
}

impl PagingType {
    pub fn from_va(va: Va) -> Self {
        Self::Pt(
            va.get_pml5_index(),
            va.get_pml4_index(),
            va.get_pdp_index(),
            va.get_pd_index(),
            va.get_pt_index(),
        )
    }

    pub fn into_raw_enum(self) -> (u64, u128) {
        match self {
            PagingType::Pml5(index) => (1, index as _),
            PagingType::Pml4(index, index2) => (2, index as u128 | (index2 as u128) << 15),
            PagingType::Pdp(index, index2, index3) => (
                3,
                index as u128 | (index2 as u128) << 15 | (index3 as u128) << 31,
            ),
            PagingType::Pd(index, index2, index3, index4) => (
                4,
                index as u128
                    | (index2 as u128) << 15
                    | (index3 as u128) << 31
                    | (index4 as u128) << 47,
            ),
            PagingType::Pt(index, index2, index3, index4, index5) => (
                5,
                index as u128
                    | (index2 as u128) << 15
                    | (index3 as u128) << 31
                    | (index4 as u128) << 47
                    | (index5 as u128) << 63,
            ),
        }
    }

    pub fn from_raw_enum(object: u64, value: u128) -> Self {
        match object {
            1 => PagingType::Pml5(value as _),
            2 => PagingType::Pml4(value.bitand(0xFFFF) as _, (value >> 15).bitand(0xFFFF) as _),
            3 => PagingType::Pdp(
                value.bitand(0xFFFF) as _,
                (value >> 15).bitand(0xFFFF) as _,
                (value >> 31).bitand(0xFFFF) as _,
            ),
            4 => PagingType::Pd(
                value.bitand(0xFFFF) as _,
                (value >> 15).bitand(0xFFFF) as _,
                (value >> 31).bitand(0xFFFF) as _,
                (value >> 47).bitand(0xFFFF) as _,
            ),
            5 => PagingType::Pt(
                value.bitand(0xFFFF) as _,
                (value >> 15).bitand(0xFFFF) as _,
                (value >> 31).bitand(0xFFFF) as _,
                (value >> 47).bitand(0xFFFF) as _,
                (value >> 63).bitand(0xFFFF) as _,
            ),
            _ => unreachable!(),
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
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            object: request.arg1,
            addr_space: request.arg2,
            map_addr: request.arg3
        }
    }
}

impl VmcallRequest for PageAttributeRequest {
    type Response = PageAttributeResponse;

    fn into_raw(self) -> HypervisorRequest {
        let args = self.paging_type.into_raw_enum();
        let args2 = self.attributes.into_raw_enum();
        HypervisorRequest {
            call: HypervisorCall::set_page_attr(),
            arg1: self.addr_space,
            arg2: self.operation as _,
            extended_arg1: args.0 as _,
            extended_arg2: args.1 as _,
            extended_arg3: args2.0 as _,
            extended_arg4: args2.1 as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        let paging_type =
            PagingType::from_raw_enum(request.extended_arg1 as _, request.extended_arg2 as _);

        let attributes =
            PageAttributes::from_raw_enum(request.extended_arg3 as _, request.extended_arg4 as _);

        Self {
            addr_space: request.arg1,
            operation: PageAttributeOperation::from(request.arg2),
            paging_type,
            attributes,
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

#[derive(Copy, Clone, Debug)]
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
