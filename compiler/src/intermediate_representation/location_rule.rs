use super::ir_interface::*;
use code_producers::cvm_elements::*;
use crate::translating_traits::*;

#[derive(Clone)]
pub struct IndexedInfo{
    pub indexes: Vec<InstructionPointer>,
    pub symbol_dim: usize
}

#[derive(Clone)]
pub enum AccessType{
    Indexed(IndexedInfo), // Case accessing an array
    Qualified(usize), // Case accessing a field -> id field
}

pub enum ComputedAddress{
    Variable(String),
    Signal(String),
    SubcmpSignal(String,String)
}


impl ToString for AccessType {
    fn to_string(&self) -> String {
        match &self{
            AccessType::Indexed(index) =>{
		
                format!("Indexed({},{})", index.symbol_dim, index.indexes.iter().map(|i| i.to_string()).collect::<String>())
            }
            AccessType::Qualified(value) =>{
                format!("field({})", value)
            }
        }
    }
}

// Example: accessing a[2][3].b[2].c
// [Indexed([2, 3]), Qualified(id_b), Indexed([2]), Qualified(id_c)]

#[derive(Clone)]
pub enum LocationRule {
    Indexed { location: InstructionPointer, template_header: Option<String> },
    Mapped { signal_code: usize, indexes: Vec<AccessType> },
}

impl ToString for LocationRule {
    fn to_string(&self) -> String {
        use LocationRule::*;
        match self {
            Indexed { location, template_header } => {
                let location_msg = location.to_string();
                let header_msg = template_header.as_ref().map_or("NONE".to_string(), |v| v.clone());
                format!("INDEXED: ({}, {})", location_msg, header_msg)
            }
            Mapped { signal_code, indexes } => {
                let code_msg = signal_code.to_string();
                let index_mgs: Vec<String> = indexes.iter().map(|i| i.to_string()).collect();
                format!("MAPPED: ({}, {:?})", code_msg, index_mgs)
            }
        }
    }
}

impl  LocationRule {
    pub fn produce_cvm(&self, address_type: & AddressType, _context: & InstrContext, producer: &mut CVMProducer) -> (Vec<String>, ComputedAddress) {
        use LocationRule::*;
        match &self {
                Indexed { location, .. } => {
                    let (mut instructions, vloc) = location.produce_cvm(producer);
                    match &address_type {
                        AddressType::Variable => {
                            (instructions, ComputedAddress::Variable(vloc));
                        }
                        AddressType::Signal => {
                            (instructions, ComputedAddress::Signal(vloc));
                        }
                        AddressType::SubcmpSignal {cmp_address, .. } => {
                            let (mut instructions_cmp, vcmp) = cmp_address.produce_cvm(producer);
                            instructions.append(&mut instructions_cmp);
                            (instructions, ComputedAddress::SubcmpSignal(vcmp,vloc));
                        }
                    }
                }
                Mapped { signal_code: _, .. } => {
                    assert!(false);
                }
            }
        (vec![], ComputedAddress::Variable("".to_string()))
    }
}
