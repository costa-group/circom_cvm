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
        use cvm_code_generator::*;
        match &self {
            Indexed { location, .. } => {
                let (mut instructions, vloc) = location.produce_cvm(producer);
                match &address_type {
                    AddressType::Variable => {
                        (instructions, ComputedAddress::Variable(vloc))
                    }
                    AddressType::Signal => {
                        (instructions, ComputedAddress::Signal(vloc))
                    }
                    AddressType::SubcmpSignal {cmp_address, .. } => {
                        let (mut instructions_cmp, vcmp) = cmp_address.produce_cvm(producer);
                        instructions.append(&mut instructions_cmp);
                        (instructions, ComputedAddress::SubcmpSignal(vcmp,vloc))
                    }
                }
            }
            Mapped { signal_code, indexes} => {
                let mut instructions = vec![];
                match address_type {
                    AddressType::SubcmpSignal { cmp_address, .. } => {
			if producer.needs_comments() {
                            instructions.push(";; is subcomponent mapped".to_string());
			}
                        let (mut instructions_cmp, vcmp) = cmp_address.produce_cvm(producer);
                        instructions.append(&mut instructions_cmp);
                        let tid = producer.fresh_var();
                        instructions.push(format!("{} = get_template_id {}", tid, vcmp));
                        let sp = producer.fresh_var();
                        instructions.push(format!("{} = get_template_signal_position {} i64.{}", sp, tid, signal_code));
			if indexes.len() == 0 {
                            (instructions, ComputedAddress::SubcmpSignal(vcmp,sp))
			} else {
                            let mut accsize = sp;
                            let mut tbid = tid.clone();
                            let mut get_dimention_function = " get_template_signal_dimension".to_string();
                            let mut get_size_function = " get_template_signal_size".to_string();
                            let mut get_type_function = " get_template_signal_type".to_string();
                            let mut idxpos = 0;
			    while idxpos < indexes.len() {
                                if let AccessType::Indexed(index_info) = &indexes[idxpos] {
                                    let index_list = &index_info.indexes;
                                    let dimensions = index_info.symbol_dim;
                                    assert!(index_list.len() > 0);
                                    let (mut instructions_idx0, vidx0) = index_list[0].produce_cvm(producer);
                                    instructions.append(&mut instructions_idx0);
                                    let psize = producer.fresh_var();
                                    let mut prevsize = psize;
                                    instructions.push(format!("{} = {}", prevsize, vidx0));
				    for i in 1..index_list.len() {
                                        let dimi = producer.fresh_var();
                                        instructions.push(format!("{} = {} {} i64.{} i64.{}", dimi, get_dimention_function, tbid, signal_code, i));
                                        let (mut instructions_idxi, vidxi) = index_list[i].produce_cvm(producer);
                                        instructions.append(&mut instructions_idxi);
                                        let curmul = producer.fresh_var();
                                        instructions.push(format!("{} = {} {} {}", curmul, mul64(), prevsize, dimi));
                                        let cursize = producer.fresh_var();
                                        instructions.push(format!("{} = {} {} {}", cursize, add64(), curmul, vidxi));
                                        prevsize = cursize;
                                    }
                                    assert!(index_list.len() <= dimensions);
				    let diff = dimensions - index_list.len();
				    if diff > 0 {
				        //println!("There is difference: {}",diff);
				        // must be last access
				        assert!(idxpos+1 == indexes.len());
				        for i in 0..diff-1 {
                                            let dimi = producer.fresh_var();
                                            instructions.push(format!("{} = {} {} i64.{} i64.{}", dimi, get_dimention_function, tid, signal_code, indexes.len() + i));                                        
                                            let cursize = producer.fresh_var();
                                            instructions.push(format!("{} = {} {} {}", cursize, mul64(), prevsize, dimi));
                                            prevsize = cursize;
				        }
				    } // after this we have the product of the remaining dimensions
                                    let vsize = producer.fresh_var();
                                    instructions.push(format!("{} = {} {} i64.{}", vsize,  get_size_function, tid, signal_code));
                                    let finalsize = producer.fresh_var();
                                    instructions.push(format!("{} = {} {} {}", finalsize, mul64(), prevsize, vsize));
                                    let access = producer.fresh_var();
                                    instructions.push(format!("{} = {} {} {}", access, add64(), accsize, prevsize));
                                    accsize = access;
                                } else if let AccessType::Qualified(field_no) = &indexes[idxpos] {
                                    let bid = producer.fresh_var();
                                    instructions.push(format!("{} = {} {} i64.{}", bid, get_type_function, tbid, signal_code));
                                    tbid = bid.clone();
                                    let sfield = producer.fresh_var();
                                    instructions.push(format!("{} = get_bus_signal_position {} i64.{}", sfield, bid, field_no));
                                    let access = producer.fresh_var();
                                    instructions.push(format!("{} = {} {} {}", access, add64(), accsize, sfield));
                                    accsize = access;
				} else {
				    assert!(false);
				}
                                if idxpos == 0 {
                                    get_dimention_function = " get_bus_signal_dimension".to_string();
                                    get_size_function = " get_bus_signal_size".to_string();
                                    get_type_function = " get_bus_signal_type".to_string();
                                }
                                idxpos += 1;
			    }
			    if producer.needs_comments() {
                                instructions.push(";; end of load bucket".to_string());
			    }
                            (instructions, ComputedAddress::SubcmpSignal(vcmp,accsize))
			}
                        //after this we have  the offset on top of the stack and the subcomponent start_of_signals just below
                    }
                    _ => {
                        assert!(false);
                        (vec![], ComputedAddress::Variable("".to_string()))
                    }
                }
            }
        }
    }
}
