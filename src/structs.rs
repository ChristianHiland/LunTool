use std::cmp::PartialEq;

pub enum TypeSize {
    NONE,
    U8,
    U16,
    U32,
    U64,
}

pub struct DataProcessData {
    pub data_u8: Vec<u8>,
    pub data_u16: Vec<u16>,
    pub data_u32: Vec<u32>,
    pub data_u64: Vec<u64>,
    pub typeUsed: TypeSize, 
}

impl PartialEq for TypeSize {
    fn eq(&self, other: &Self) -> bool {
        return self.eq(&other);
    }
}

impl DataProcessData {
    pub fn new() -> Self {
        Self {
            data_u8: Vec::new(),
            data_u16: Vec::new(),
            data_u32: Vec::new(),
            data_u64: Vec::new(),
            typeUsed: TypeSize::NONE,
        }
    }
    
    pub fn compact(self) -> Self {
        if self.typeUsed == TypeSize::U8 { 
            return self; 
        } else if self.typeUsed == TypeSize::U16 {
            // TODO: See if any numbers can be made smaller.
        } else if self.typeUsed == TypeSize::U32 { 
            // TODO: See if any numbers can be made smaller.
        } else if self.typeUsed == TypeSize::U64 {
            // TODO: See if any numbers can be made smaller.
        }
        
        return self;
    }
}