use std::cmp::PartialEq;

pub struct TypeSize {
    NONE:   i8,
    U8:     i8,
    U16:    i8,
    U32:    i8,
    U64:    i8,
    SEL:    i8,
}

pub struct DataProcessData {
    pub data_u8: Vec<u8>,
    pub data_u16: Vec<u16>,
    pub data_u32: Vec<u32>,
    pub data_u64: Vec<u64>,
    pub typeUsed: TypeSize, 
}

impl TypeSize {
    pub fn new() -> Self {
           Self {
               NONE: 0,
               U8: 1,
               U16: 2,
               U32: 3,
               U64: 4,
               SEL: 0,
           }
    }
    
    pub fn set(&mut self, index: i8) {
        self.SEL = index;
    }
}

impl DataProcessData {
    pub fn new() -> Self {
        let typesize = TypeSize::new();
        
        Self {
            data_u8: Vec::new(),
            data_u16: Vec::new(),
            data_u32: Vec::new(),
            data_u64: Vec::new(),
            typeUsed: typesize,
        }
    }
    
    pub fn compact(self) -> Self {
        let typesize = TypeSize::new();
        if self.typeUsed.SEL == typesize.NONE { 
            return self; 
        } else if self.typeUsed.SEL == typesize.U16 {
            // TODO: See if any numbers can be made smaller.
            let mut isHigherAny: bool = false;
            for data_16 in  self.data_u16.clone() {
                if data_16 <= 0b11111111 {
                    continue;
                } else if data_16 > 0b11111111 {
                    isHigherAny = true;
                }
            }
            
            if !isHigherAny {
                for data_16_to_8 in &self.data_u16 {
                    if data_16_to_8 << 0b00000001 <= 0b11111111u16  {
                        
                    }
                }
            }
        } else if self.typeUsed.SEL == typesize.U32 { 
            // TODO: See if any numbers can be made smaller.
        } else if self.typeUsed.SEL == typesize.U64 {
            // TODO: See if any numbers can be made smaller.
        }
        return self;
    }
}