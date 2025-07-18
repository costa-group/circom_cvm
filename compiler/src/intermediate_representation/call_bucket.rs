use super::ir_interface::*;
use crate::translating_traits::*;
use code_producers::c_elements::*;
use code_producers::wasm_elements::*;
use code_producers::cvm_elements::*;


#[derive(Clone)]
pub struct FinalData {
    // greater than one only with signals.
    pub context: InstrContext,
    pub dest_is_output: bool,
    pub dest_address_type: AddressType,
    pub dest: LocationRule,
}

#[derive(Clone)]
pub enum ReturnType {
    Intermediate { op_aux_no: usize },
    Final(FinalData),
}

#[derive(Clone)]
pub struct CallBucket {
    pub line: usize,
    pub message_id: usize,
    pub symbol: String,
    pub argument_types: Vec<InstrContext>,
    pub arguments: InstructionList,
    pub arena_size: usize,
    pub return_info: ReturnType,
    pub is_called_function_returning_array: bool,
}

impl IntoInstruction for CallBucket {
    fn into_instruction(self) -> Instruction {
        Instruction::Call(self)
    }
}

impl Allocate for CallBucket {
    fn allocate(self) -> InstructionPointer {
        InstructionPointer::new(self.into_instruction())
    }
}

impl ObtainMeta for CallBucket {
    fn get_line(&self) -> usize {
        self.line
    }
    fn get_message_id(&self) -> usize {
        self.message_id
    }
}

impl ToString for CallBucket {
    fn to_string(&self) -> String {
        let line = self.line.to_string();
        let template_id = self.message_id.to_string();
	let ret = match &self.return_info {
            ReturnType::Intermediate { op_aux_no } => {format!("Intermediate({})",op_aux_no.to_string())}
	    _ => {format!("Final")}
	};
        let mut args = "".to_string();
        for i in &self.arguments {
            args = format!("{}{},", args, i.to_string());
        }
        format!("CALL(line:{},template_id:{},id:{},return_type:{},args:{})", line, template_id, self.symbol, ret, args)
    }
}

impl WriteWasm for CallBucket {
    fn produce_wasm(&self, producer: &WASMProducer) -> Vec<String> {
        use code_producers::wasm_elements::wasm_code_generator::*;
        let mut instructions = vec![];
        if producer.needs_comments() {
            instructions.push(";; call bucket".to_string());
	}
        if self.arguments.len() > 0 {
            let local_info_size_u32 = producer.get_local_info_size_u32();
            instructions.push(set_constant("0"));
            instructions.push(load32(None)); // current stack size
            let var_start = local_info_size_u32 * 4; // starts after local info
            if local_info_size_u32 != 0 {
                instructions.push(set_constant(&var_start.to_string()));
                instructions.push(add32());
            }
            instructions.push(set_local(producer.get_call_lvar_tag()));
            let mut count = 0;
            let mut i = 0;
            for p in &self.arguments {
		if producer.needs_comments() {
                    instructions.push(format!(";; copying argument {}", i));
		}
                instructions.push(get_local(producer.get_call_lvar_tag()));
                instructions.push(set_constant(&count.to_string()));
                instructions.push(add32());
                let arg_size = match &self.argument_types[i].size{
                    SizeOption::Single(value) => *value,
                    SizeOption::Multiple(_value) => {
			assert!(false); //should never be the case!
			0
                    }
                };
                if arg_size > 1 {
                    instructions.push(set_local(producer.get_store_aux_1_tag()));
                }
                let mut instructions_arg = p.produce_wasm(producer);
                instructions.append(&mut instructions_arg);
                if arg_size == 1 {
                    instructions.push(call("$Fr_copy"));
                } else {
                    instructions.push(set_local(producer.get_store_aux_2_tag()));
                    instructions.push(set_constant(&arg_size.to_string()));
                    instructions.push(set_local(producer.get_copy_counter_tag()));
                    instructions.push(add_block());
                    instructions.push(add_loop());
                    instructions.push(get_local(producer.get_copy_counter_tag()));
                    instructions.push(eqz32());
                    instructions.push(br_if("1"));
                    instructions.push(get_local(producer.get_store_aux_1_tag()));
                    instructions.push(get_local(producer.get_store_aux_2_tag()));
                    instructions.push(call("$Fr_copy"));
                    instructions.push(get_local(producer.get_copy_counter_tag()));
                    instructions.push(set_constant("1"));
                    instructions.push(sub32());
                    instructions.push(set_local(producer.get_copy_counter_tag()));
                    instructions.push(get_local(producer.get_store_aux_1_tag()));
                    let s = producer.get_size_32_bits_in_memory() * 4;
                    instructions.push(set_constant(&s.to_string()));
                    instructions.push(add32());
                    instructions.push(set_local(producer.get_store_aux_1_tag()));
                    instructions.push(get_local(producer.get_store_aux_2_tag()));
                    instructions.push(set_constant(&s.to_string()));
                    instructions.push(add32());
                    instructions.push(set_local(producer.get_store_aux_2_tag()));
                    instructions.push(br("0"));
                    instructions.push(add_end());
                    instructions.push(add_end());
                }
		if producer.needs_comments() {
                    instructions.push(format!(";; end copying argument {}", i));
		}
                count += arg_size * 4 * producer.get_size_32_bits_in_memory();
                i += 1;
            }
        }
        match &self.return_info {
            ReturnType::Intermediate { op_aux_no } => {
                instructions.push(get_local(producer.get_expaux_tag()));
                instructions.push(set_constant(&op_aux_no.to_string()));
                instructions.push(add32());
                instructions.push(set_constant("1"));
                instructions.push(call(&format!("${}", self.symbol)));
		instructions.push(tee_local(producer.get_merror_tag()));
		instructions.push(add_if());
                instructions.push(call("$printErrorMessage"));
		instructions.push(get_local(producer.get_merror_tag()));    
                instructions.push(add_return());
                instructions.push(add_end());
                instructions.push(get_local(producer.get_expaux_tag()));
                instructions.push(set_constant(&op_aux_no.to_string()));
                instructions.push(add32());
            }
            ReturnType::Final(data) => {
                let mut my_template_header = Option::<String>::None;
                match &data.dest {
                    LocationRule::Indexed { location, template_header } => {
			if producer.needs_comments() {
                            instructions.push(";; getting result address".to_string());
			}
                        let mut instructions_dest = location.produce_wasm(producer);
                        instructions.append(&mut instructions_dest);
                        let size = producer.get_size_32_bits_in_memory() * 4;
                        instructions.push(set_constant(&size.to_string()));
                        instructions.push(mul32());
                        match &data.dest_address_type {
                            AddressType::Variable => {
                                instructions.push(get_local(producer.get_lvar_tag()));
                            }
                            AddressType::Signal => {
                                instructions.push(get_local(producer.get_signal_start_tag()));
                            }
                            AddressType::SubcmpSignal { cmp_address, .. } => {
                                my_template_header = template_header.clone();
                                instructions.push(get_local(producer.get_offset_tag()));
                                instructions.push(set_constant(
                                    &producer.get_sub_component_start_in_component().to_string(),
                                ));
                                instructions.push(add32());
                                let mut instructions_sci = cmp_address.produce_wasm(producer);
                                instructions.append(&mut instructions_sci);
                                instructions.push(set_constant("4")); //size in byte of i32
                                instructions.push(mul32());
                                instructions.push(add32());
                                instructions.push(load32(None)); //subcomponent block
                                instructions.push(tee_local(producer.get_sub_cmp_tag()));
                                //instructions.push(set_local(producer.get_sub_cmp_tag()));
                                //instructions.push(get_local(producer.get_sub_cmp_tag()));
                                instructions.push(set_constant(
                                    &producer.get_signal_start_address_in_component().to_string(),
                                ));
                                instructions.push(add32());
                                instructions.push(load32(None)); //subcomponent start_of_signals
                            }
                        }
                        instructions.push(add32());
                    }
                    LocationRule::Mapped { signal_code, indexes } => {
                        match &data.dest_address_type {
                            AddressType::SubcmpSignal { cmp_address, .. } => {
				if producer.needs_comments() {
                                    instructions.push(";; is subcomponent".to_string());
				}
                                instructions.push(get_local(producer.get_offset_tag()));
                                instructions.push(set_constant(
                                    &producer.get_sub_component_start_in_component().to_string(),
                                ));
                                instructions.push(add32());
                                let mut instructions_sci = cmp_address.produce_wasm(producer);
                                instructions.append(&mut instructions_sci);
                                instructions.push(set_constant("4")); //size in byte of i32
                                instructions.push(mul32());
                                instructions.push(add32());
                                instructions.push(load32(None)); //subcomponent block
                                instructions.push(tee_local(producer.get_sub_cmp_tag()));
                                //instructions.push(set_local(producer.get_sub_cmp_tag()));
                                //instructions.push(get_local(producer.get_sub_cmp_tag()));
                                instructions.push(load32(None)); // get template id                     A
                                instructions.push(set_constant("4")); //size in byte of i32
                                instructions.push(mul32());
                                instructions.push(load32(Some(
                                    &producer
                                        .get_template_instance_to_io_signal_start()
                                        .to_string(),
                                ))); // get position in component io signal to info list
                                let signal_code_in_bytes = signal_code * 4; //position in the list of the signal code
                                instructions.push(load32(Some(&signal_code_in_bytes.to_string()))); // get where the info of this signal is
				//now we have first the offset, and then the all size dimensions but the last one
				if indexes.len() == 0 {
				    instructions.push(load32(None)); // get signal offset (it is already the actual one in memory);
				} else {
				    instructions.push(tee_local(producer.get_io_info_tag()));
				    instructions.push(load32(None)); // get offset; first slot in io_info (to start adding offsets)
				    // if the first access is qualified we place the address of the bus_id
				    if let AccessType::Qualified(_) = &indexes[0] {
					instructions.push(get_local(producer.get_io_info_tag()));
					instructions.push(load32(Some("4"))); // it is a bus, so the bus_id is in the second position
				    }
				    let mut idxpos = 0;			    
				    while idxpos < indexes.len() {
					if let AccessType::Indexed(index_info) = &indexes[idxpos] {					    
					    let index_list = &index_info.indexes;
					    let dim = index_info.symbol_dim;
					    let mut infopos = 0;
					    assert!(index_list.len() > 0);
					    //We first compute the number of elements as
					    //((index_list[0] * length_of_dim[1]) + index_list[1]) * length_of_dim[2] + ... )* length_of_dim[n-1] + index_list[n-1]
					    //first position in the array access
					    let mut instructions_idx0 = index_list[0].produce_wasm(producer);				    
					    instructions.append(&mut instructions_idx0);				    
					    for i in 1..index_list.len() {
						instructions.push(get_local(producer.get_io_info_tag()));
						infopos += 4;	//position in io or bus info of dimension of [1] (recall that first dimension is not added)
						instructions.push(load32(Some(&infopos.to_string()))); // second dimension
						instructions.push(mul32());
						let mut instructions_idxi = index_list[i].produce_wasm(producer);				    
						instructions.append(&mut instructions_idxi);				    
						instructions.push(add32());
					    }
					    assert!(index_list.len() <= dim);
					    let diff = dim - index_list.len();
					    if diff > 0 {
						//println!("There is difference: {}",diff);
						//instructions.push(format!(";; There is a difference {}", diff));
						// must be last access
						assert!(idxpos+1 == indexes.len());
						instructions.push(get_local(producer.get_io_info_tag()));
						infopos += 4; //position in io or bus info of the next dimension 
						instructions.push(load32(Some(&infopos.to_string()))); // length of next dimension					
						for _i in 1..diff {
						    //instructions.push(format!(";; Next dim {}", i));
						    instructions.push(get_local(producer.get_io_info_tag()));
						    infopos += 4; //position in io or bus info of the next dimension 
						    instructions.push(load32(Some(&infopos.to_string()))); // length of next dimension					
						    instructions.push(mul32()); // multiply with previous dimensions
						}
					    } // after this we have the product of the remaining dimensions
					    let field_size = producer.get_size_32_bits_in_memory() * 4;
					    instructions.push(set_constant(&field_size.to_string()));
					    instructions.push(get_local(producer.get_io_info_tag()));
					    infopos += 4; //position in io or bus info of size 
					    instructions.push(load32(Some(&infopos.to_string()))); // size
					    instructions.push(mul32()); // size mult by size of field in bytes
					    if diff > 0 {
						//instructions.push(format!(";; Multiply dimensions"));
						instructions.push(mul32()); // total size of the content according to the missing dimensions
					    }
					    instructions.push(mul32()); // total offset in the array
					    instructions.push(add32()); // to the current offset
					    idxpos += 1;
					    if idxpos < indexes.len() {
						//next must be Qualified
						if let AccessType::Indexed(_) = &indexes[idxpos] {
						    assert!(false);
						}
						// we add the type of bus it is
						instructions.push(get_local(producer.get_io_info_tag()));
						infopos += 4;
						instructions.push(load32(Some(&infopos.to_string()))); // bus_id
					    }
					} else if let AccessType::Qualified(field_no) = &indexes[idxpos] {
					    //we have on the stack the bus_id
					    instructions.push(set_constant("4")); //size in byte of i32
					    instructions.push(mul32()); //maybe better in the memory like this
					    instructions.push(load32(Some(
						&producer.get_bus_instance_to_field_start().to_string()
					    ))); // get position in the bus to field in memory
					    let field_no_bytes = field_no * 4;
					    instructions.push(load32(Some(&field_no_bytes.to_string()))); // get position in the field info in memory
					    if idxpos +1 < indexes.len() {				    
						instructions.push(tee_local(producer.get_io_info_tag()));
					    }
					    //let field_size = producer.get_size_32_bits_in_memory() * 4;
					    //instructions.push(set_constant(&field_size.to_string()));
					    instructions.push(load32(None)); // get the offset
					    //instructions.push(mul32()); // mult by size of field in bytes
					    instructions.push(add32()); // add to the current offset
					    idxpos += 1;
					    if idxpos < indexes.len() {
						if let AccessType::Qualified(_) = &indexes[idxpos] {
						    instructions.push(get_local(producer.get_io_info_tag()));
						    instructions.push(load32(Some("4"))); // bus_id
						}
					    }
					} else {
					    assert!(false);
					}
				    }
				}
                                instructions.push(get_local(producer.get_sub_cmp_tag()));
                                instructions.push(set_constant(
                                    &producer.get_signal_start_address_in_component().to_string(),
                                ));
                                instructions.push(add32());
                                instructions.push(load32(None)); //subcomponent start_of_signals
                                instructions.push(add32()); // we get the position of the signal (with indexes) in memory
                            }
                            _ => {
                                assert!(false);
                            }
                        }
                    }
                }
		// We check if we have to compute the possible sizes, case multiple size
                match &data.context.size{
		    SizeOption::Single(value) => {
			instructions.push(set_constant(&value.to_string()));
		    }
		    SizeOption::Multiple(values) => { //create a nested if-else with all cases
			instructions.push(get_local(producer.get_sub_cmp_tag()));
			instructions.push(load32(None)); // get template id
			instructions.push(set_local(producer.get_aux_0_tag()));
			let mut instr_if = create_if_selection(&values,producer.get_aux_0_tag());
			instructions.append(&mut instr_if);
			instructions.push(tee_local(producer.get_result_size_tag()));
		    }
		};
                instructions.push(call(&format!("${}", self.symbol)));
                instructions.push(tee_local(producer.get_merror_tag()));
		instructions.push(add_if());
                instructions.push(set_constant(&self.message_id.to_string()));
                instructions.push(set_constant(&self.line.to_string()));
                instructions.push(call("$buildBufferMessage"));
                instructions.push(call("$printErrorMessage"));
		instructions.push(get_local(producer.get_merror_tag()));    
                instructions.push(add_return());
                instructions.push(add_end());
                match &data.dest_address_type {
                    AddressType::SubcmpSignal { .. } => {
                        // if subcomponent input check if run needed
			if producer.needs_comments() {
                            instructions.push(";; decrease counter".to_string()); // by self.context.size
			}
                        instructions.push(get_local(producer.get_sub_cmp_tag()));
                        instructions.push(get_local(producer.get_sub_cmp_tag()));
                        instructions.push(load32(Some(
                            &producer.get_input_counter_address_in_component().to_string(),
                        ))); //remaining inputs to be set
			match &data.context.size{
			    SizeOption::Single(value) => {
				instructions.push(set_constant(&value.to_string()));
			    }
			    SizeOption::Multiple(_) => { 
				instructions.push(get_local(producer.get_result_size_tag()));
			    }
			};
                        instructions.push(sub32());
                        instructions.push(store32(Some(
                            &producer.get_input_counter_address_in_component().to_string(),
                        ))); // update remaining inputs to be set
			if producer.needs_comments() {
                            instructions.push(";; check if run is needed".to_string());
			}
                        instructions.push(get_local(producer.get_sub_cmp_tag()));
                        instructions.push(load32(Some(
                            &producer.get_input_counter_address_in_component().to_string(),
                        )));
                        instructions.push(eqz32());
                        instructions.push(add_if());
			if producer.needs_comments() {
                            instructions.push(";; run sub component".to_string());
			}
                        instructions.push(get_local(producer.get_sub_cmp_tag()));
                        match &data.dest {
                            LocationRule::Indexed { .. } => {
                                if let Some(name) = &my_template_header {
                                    instructions.push(call(&format!("${}_run", name)));
                                    instructions.push(tee_local(producer.get_merror_tag()));
                                    instructions.push(add_if());
                                    instructions.push(set_constant(&self.message_id.to_string()));
                                    instructions.push(set_constant(&self.line.to_string()));
                                    instructions.push(call("$buildBufferMessage"));
                                    instructions.push(call("$printErrorMessage"));
                                    instructions.push(get_local(producer.get_merror_tag()));    
                                    instructions.push(add_return());
                                    instructions.push(add_end());
                                } else {
                                    assert!(false);
                                }
                            }
                            LocationRule::Mapped { .. } => {
                                instructions.push(get_local(producer.get_sub_cmp_tag()));
                                instructions.push(load32(None)); // get template id
                                instructions.push(call_indirect(
                                    &"$runsmap".to_string(),
                                    &"(type $_t_i32ri32)".to_string(),
                                ));
                                instructions.push(tee_local(producer.get_merror_tag()));
                                instructions.push(add_if());
                                instructions.push(set_constant(&self.message_id.to_string()));
                                instructions.push(set_constant(&self.line.to_string()));
                                instructions.push(call("$buildBufferMessage"));
                                instructions.push(call("$printErrorMessage"));
                                instructions.push(get_local(producer.get_merror_tag()));    
                                instructions.push(add_return());
                                instructions.push(add_end());
                            }
                        }
			if producer.needs_comments() {
                            instructions.push(";; end run sub component".to_string());
			}
                        instructions.push(add_end());
                    }
                    _ => (),
                }
            }
        }
        if producer.needs_comments() {
            instructions.push(";; end call bucket".to_string());
	}
        instructions
    }
}

impl WriteC for CallBucket {
    fn produce_c(&self, producer: &CProducer, parallel: Option<bool>) -> (Vec<String>, String) {
        use c_code_generator::*;
        let mut prologue = vec![];
        //create block
        prologue.push("{\n".to_string());
        prologue.push("// start of call bucket".to_string());
        // create lvar parameter
        if producer.prime_str != "goldilocks" {
            prologue.push(format!("{};", declare_lvar_func_call(self.arena_size)));
        } else {
            prologue.push(format!("{};", declare_64bit_lvar_func_call(self.arena_size)));
        }            
        // copying parameters
        let mut count = 0;
        let mut i = 0;
        for p in &self.arguments {
            prologue.push(format!("// copying argument {}", i));
            let (mut prologue_value, src) = p.produce_c(producer, parallel);
            prologue.append(&mut prologue_value);
            let arena_position =
                if producer.prime_str != "goldilocks" {
                    format!("&{}[{}]", L_VAR_FUNC_CALL_STORAGE, count)
                } else {
                    format!("{}[{}]", L_VAR_FUNC_CALL_STORAGE, count)
                };
            // TODO, CASE CALL ARGUMENTS
            let size = match &self.argument_types[i].size{
                SizeOption::Single(value) => *value,
                SizeOption::Multiple(_values) => {
		    assert!(false); //should never be the case!
		    0
                }
            };
            if size > 1 {
                let copy_arguments = if producer.prime_str != "goldilocks" {
                    vec![arena_position, src, size.to_string()]
                }else {
                    vec![format!("&{}",arena_position), format!("&{}",src), size.to_string()]
                };
                prologue
                    .push(format!("{};", build_call("Fr_copyn".to_string(), copy_arguments)));
            } else {
                let copy_arguments = vec![arena_position, src];
                prologue
                    .push(format!("{};", build_call("Fr_copy".to_string(), copy_arguments)));
            }
            prologue.push(format!("// end copying argument {}", i));
            count += size;
            i += 1;
        }
        let result;
        let mut call_arguments = vec![];
        call_arguments.push(CIRCOM_CALC_WIT.to_string());
        call_arguments.push(L_VAR_FUNC_CALL_STORAGE.to_string());
        call_arguments.push(my_id());
        match &self.return_info {
            ReturnType::Intermediate { op_aux_no } => {
                let exp_aux_index = op_aux_no.to_string();
                let result_ref =
                    if producer.prime_str != "goldilocks" {
                        format!("&{}", expaux(exp_aux_index.clone()))
                    } else {
                        format!("{}", expaux(exp_aux_index.clone()))
                    };
                call_arguments.push(result_ref.clone());
                call_arguments.push("1".to_string());
                prologue.push(format!("{};", build_call(self.symbol.clone(), call_arguments)));
                result = result_ref;
            }
            ReturnType::Final(data) => {
                let cmp_index_ref = "cmp_index_ref".to_string();
		if let AddressType::SubcmpSignal { cmp_address, .. } = &data.dest_address_type {
		    let (mut cmp_prologue, cmp_index) = cmp_address.produce_c(producer, parallel);
		    prologue.append(&mut cmp_prologue);
		    prologue.push(format!("{{"));
	        prologue.push(format!("uint {} = {};",  cmp_index_ref, cmp_index));
		}
        let size = match &data.context.size{
            SizeOption::Single(value) => value.to_string(),
            SizeOption::Multiple(values) => {
                prologue.push(format!("std::map<int,int> size_store {};",
                    set_list_tuple(values.to_vec())
                ));
                let sub_component_pos_in_memory = format!("{}[{}]",MY_SUBCOMPONENTS,cmp_index_ref);
                let temp_id = template_id_in_component(sub_component_pos_in_memory);
                format!("size_store[{}]", temp_id)
            }
        };

                let ((mut dest_prologue, dest_index), my_template_header) =
                    if let LocationRule::Indexed { location, template_header } = &data.dest {
                        (location.produce_c(producer, parallel), template_header.clone())
		    } else if let LocationRule::Mapped { signal_code, indexes} = &data.dest {
            let mut map_prologue = vec![];
			let sub_component_pos_in_memory = format!("{}[{}]",MY_SUBCOMPONENTS,cmp_index_ref.clone());
			let mut map_access = format!("{}->{}[{}].defs[{}].offset",
						     circom_calc_wit(), template_ins_2_io_info(),
						     template_id_in_component(sub_component_pos_in_memory.clone()),
						     signal_code.to_string());
			if indexes.len() > 0 {
			    map_prologue.push(format!("{{"));
			    map_prologue.push(format!("uint map_accesses_aux[{}];",indexes.len().to_string()));	
			    map_prologue.push(format!("{{"));
			    //cur_def contains a pointer to the definion of the next acces.
			    //The first time it is taken from template_ins_2_io_info
			    map_prologue.push(format!("IOFieldDef *cur_def = &({}->{}[{}].defs[{}]);",
						      circom_calc_wit(), template_ins_2_io_info(),
						      template_id_in_component(sub_component_pos_in_memory.clone()),
						      signal_code.to_string()));
			    let mut idxpos = 0;
			    while idxpos < indexes.len() {
				if let AccessType::Indexed(index_info) = &indexes[idxpos] {
				    let index_list = &index_info.indexes;
				    let dim = index_info.symbol_dim;
				    map_prologue.push(format!("{{"));
				    map_prologue.push(format!("uint map_index_aux[{}];",index_list.len().to_string()));
				    //We first compute the number of elements as
				    //((map_index_aux[0] * length_of_dim[1]) + map_index_aux[1]) * length_of_dim[2] + ... )* length_of_dim[n-1] + map_index_aux[n-1] with
				    // map_index_aux[i] = computation of index_list[i]
				    let (mut index_code_0, mut map_index) = index_list[0].produce_c(producer, parallel);
				    map_prologue.append(&mut index_code_0);
				    map_prologue.push(format!("map_index_aux[0]={};",map_index));
				    map_index = format!("map_index_aux[0]");
				    for i in 1..index_list.len() {
					let (mut index_code, index_exp) = index_list[i].produce_c(producer, parallel);
					map_prologue.append(&mut index_code);
					map_prologue.push(format!("map_index_aux[{}]={};",i.to_string(),index_exp));
					// multiply the offset inthe array by the size of the elements
					map_index = format!("({})*cur_def->lengths[{}]+map_index_aux[{}]",
							    map_index,(i-1).to_string(),i.to_string());
				    }
				    assert!(index_list.len() <= dim);
				    if dim - index_list.len() > 0 {
					map_prologue.push(format!("//There is a difference {};",dim - index_list.len()));
					// must be last access
					assert!(idxpos+1 == indexes.len());
					for i in index_list.len()..dim {
					    map_index = format!("{}*cur_def->lengths[{}]",
								map_index, (i-1).to_string());
					} // after this we have multiplied by the remaining dimensions
				    }
				    // multiply the offset in the array (after multiplying by the missing dimensions) by the size of the elements
				    map_prologue.push(format!("map_accesses_aux[{}] = {}*cur_def->size;", idxpos.to_string(), map_index));
				    map_prologue.push(format!("}}"));
				} else if let AccessType::Qualified(field_no) = &indexes[idxpos] {
				    map_prologue.push(format!("cur_def = &({}->{}[cur_def->busId].defs[{}]);",
							      circom_calc_wit(), bus_ins_2_field_info(),
							      field_no.to_string()));
				    map_prologue.push(format!("map_accesses_aux[{}] = cur_def->offset;", idxpos.to_string()));
				} else {
				    assert!(false);
				}
				// add to the access expression the computed offset
				map_access = format!("{}+map_accesses_aux[{}]",
						     map_access, idxpos.to_string());
				idxpos += 1;
			    }
			    map_prologue.push(format!("}}"));
			}
			((map_prologue, map_access),Some(template_id_in_component(sub_component_pos_in_memory.clone())))
                    } else {
			assert!(false);
			((vec![], "".to_string()),Option::<String>::None)
                    };
		prologue.append(&mut dest_prologue);
                let result_ref = match &data.dest_address_type {
                    AddressType::Variable => {
                        if producer.prime_str != "goldilocks" {
                            format!("&{}", lvar(dest_index.clone()))
                        } else {
                            format!("{}", lvar(dest_index.clone()))
                        }
                    }
                    AddressType::Signal => {
                        if producer.prime_str != "goldilocks" {
                            format!("&{}", signal_values(dest_index.clone()))
                        } else {
                            format!("{}", signal_values(dest_index.clone()))
                        }
                    }
                    AddressType::SubcmpSignal { .. } => {
                        let sub_cmp_start = format!(
                            "{}->componentMemory[{}[{}]].signalStart",
                            CIRCOM_CALC_WIT, MY_SUBCOMPONENTS, cmp_index_ref
                        );
                        if producer.prime_str != "goldilocks" {
                            format!(
                                "&{}->signalValues[{} + {}]",
                                CIRCOM_CALC_WIT, sub_cmp_start, dest_index.clone()
                            )
                        } else {
                            format!(
                                "{}->signalValues[{} + {}]",
                                CIRCOM_CALC_WIT, sub_cmp_start, dest_index.clone()
                            )
                        }
                    }
                };
                call_arguments.push(result_ref);
                call_arguments.push(size.clone());
                prologue.push(format!("{};", build_call(self.symbol.clone(), call_arguments)));
		if let LocationRule::Mapped { indexes, .. } = &data.dest {
		    if indexes.len() > 0 {
    			prologue.push(format!("}}"));
		    }
		}
		// if output and parallel send notify
		if let AddressType::Signal = &data.dest_address_type {
		    if parallel.unwrap()  && data.dest_is_output {
			if size != "0" {
			    prologue.push(format!("{{"));
			    prologue.push(format!("for (int i = 0; i < {}; i++) {{", size));
			    prologue.push(format!("{}->componentMemory[{}].mutexes[{}+i].lock();",CIRCOM_CALC_WIT,CTX_INDEX,dest_index.clone()));
			    prologue.push(format!("{}->componentMemory[{}].outputIsSet[{}+i]=true;",CIRCOM_CALC_WIT,CTX_INDEX,dest_index.clone()));
			    prologue.push(format!("{}->componentMemory[{}].mutexes[{}+i].unlock();",CIRCOM_CALC_WIT,CTX_INDEX,dest_index.clone()));
			    prologue.push(format!("{}->componentMemory[{}].cvs[{}+i].notify_all();",CIRCOM_CALC_WIT,CTX_INDEX,dest_index.clone()));
			    prologue.push(format!("}}"));
			    prologue.push(format!("}}"));
			} else {
			    prologue.push(format!("{}->componentMemory[{}].mutexes[{}].lock();",CIRCOM_CALC_WIT,CTX_INDEX,dest_index.clone()));
			    prologue.push(format!("{}->componentMemory[{}].outputIsSet[{}]=true;",CIRCOM_CALC_WIT,CTX_INDEX,dest_index.clone()));
			    prologue.push(format!("{}->componentMemory[{}].mutexes[{}].unlock();",CIRCOM_CALC_WIT,CTX_INDEX,dest_index.clone()));
			    prologue.push(format!("{}->componentMemory[{}].cvs[{}].notify_all();",CIRCOM_CALC_WIT,CTX_INDEX,dest_index.clone()));
			}
		    }
		}
                // like store update counters and check if Subcomponent needs to be run
                match &data.dest_address_type {
                    AddressType::SubcmpSignal { uniform_parallel_value, input_information, .. } => {
                        // if subcomponent input check if run needed
                        let sub_cmp_counter = format!(
                            "{}->componentMemory[{}[{}]].inputCounter",
                            CIRCOM_CALC_WIT, MY_SUBCOMPONENTS, cmp_index_ref
                        );
                        let sub_cmp_counter_decrease = format!(
                            "{} -= {}",
                            sub_cmp_counter, size
                        );
			if let InputInformation::Input{status, needs_decrement} = input_information {
			    if let StatusInput::NoLast = status {
				// no need to run subcomponent
                if *needs_decrement{
				    prologue.push("// no need to run sub component".to_string());
				    prologue.push(format!("{};", sub_cmp_counter_decrease));
				    prologue.push(format!("assert({} > 0);", sub_cmp_counter));
                }
			    } else {
				let sub_cmp_pos = format!("{}[{}]", MY_SUBCOMPONENTS, cmp_index_ref);
				let sub_cmp_call_arguments =
				    vec![sub_cmp_pos, CIRCOM_CALC_WIT.to_string()];

                // to create the call instruction we need to consider the cases of parallel/not parallel/ known only at execution
                if uniform_parallel_value.is_some(){
                    // Case parallel
                    let mut call_instructions = if uniform_parallel_value.unwrap(){
                        let sub_cmp_call_name = if let LocationRule::Indexed { .. } = &data.dest {
                            format!("{}_run_parallel", my_template_header.unwrap())
                        } else {
                            format!("(*{}[{}])", function_table_parallel(), my_template_header.unwrap())
                        };
                        let mut thread_call_instr = vec![];
                            
                            // parallelism
                        thread_call_instr.push(format!("{}->componentMemory[{}].sbct[{}] = std::thread({},{});",CIRCOM_CALC_WIT,CTX_INDEX,cmp_index_ref, sub_cmp_call_name, argument_list(sub_cmp_call_arguments)));
                        thread_call_instr.push(format!("std::unique_lock<std::mutex> lkt({}->numThreadMutex);",CIRCOM_CALC_WIT));
                        thread_call_instr.push(format!("{}->ntcvs.wait(lkt, [{}]() {{return  {}->numThread <  {}->maxThread; }});",CIRCOM_CALC_WIT,CIRCOM_CALC_WIT,CIRCOM_CALC_WIT,CIRCOM_CALC_WIT));
                        thread_call_instr.push(format!("ctx->numThread++;"));
                        //thread_call_instr.push(format!("printf(\"%i \\n\", ctx->numThread);"));
                        thread_call_instr

                    }
                    // Case not parallel
                    else{
                        let sub_cmp_call_name = if let LocationRule::Indexed { .. } = &data.dest {
                            format!("{}_run", my_template_header.unwrap())
                        } else {
                            format!("(*{}[{}])", function_table(), my_template_header.unwrap())
                        };
                        vec![format!(
                            "{};",
                            build_call(sub_cmp_call_name, sub_cmp_call_arguments)
                        )]
                    };
                    if let StatusInput::Unknown = status {
                        assert!(*needs_decrement);

                        let sub_cmp_counter_decrease_andcheck = format!("!({})",sub_cmp_counter_decrease);
                        let if_condition = vec![sub_cmp_counter_decrease_andcheck];
                        prologue.push("// run sub component if needed".to_string());
                        let else_instructions = vec![];
                        prologue.push(build_conditional(if_condition,call_instructions,else_instructions));
                    } else {
                        if *needs_decrement{
                            // TODO: Remove these instructions
                            prologue.push("// need to run sub component".to_string());
                            prologue.push(format!("{};", sub_cmp_counter_decrease));
                            prologue.push(format!("assert(!({}));", sub_cmp_counter));
                        }
                        prologue.append(&mut call_instructions);
                    }
                }
                // Case we only know if it is parallel at execution
                else{
                    prologue.push(format!(
                        "if ({}[{}]){{",
                        MY_SUBCOMPONENTS_PARALLEL, 
                        cmp_index_ref
                    ));

                    // case parallel
                    let sub_cmp_call_name = if let LocationRule::Indexed { .. } = &data.dest {
                        format!("{}_run_parallel", my_template_header.clone().unwrap())
                    } else {
                        format!("(*{}[{}])", function_table_parallel(), my_template_header.clone().unwrap())
                    };
                    let mut call_instructions = vec![];  
                        // parallelism
                    call_instructions.push(format!("{}->componentMemory[{}].sbct[{}] = std::thread({},{});",CIRCOM_CALC_WIT,CTX_INDEX,cmp_index_ref, sub_cmp_call_name, argument_list(sub_cmp_call_arguments.clone())));
                    call_instructions.push(format!("std::unique_lock<std::mutex> lkt({}->numThreadMutex);",CIRCOM_CALC_WIT));
                    call_instructions.push(format!("{}->ntcvs.wait(lkt, [{}]() {{return {}->numThread <  {}->maxThread; }});",CIRCOM_CALC_WIT,CIRCOM_CALC_WIT,CIRCOM_CALC_WIT,CIRCOM_CALC_WIT));
                    call_instructions.push(format!("ctx->numThread++;"));
                    //call_instructions.push(format!("printf(\"%i \\n\", ctx->numThread);"));
                    if let StatusInput::Unknown = status {
                        let sub_cmp_counter_decrease_andcheck = format!("!({})",sub_cmp_counter_decrease);
                        let if_condition = vec![sub_cmp_counter_decrease_andcheck];
                        prologue.push("// run sub component if needed".to_string());
                        let else_instructions = vec![];
                        prologue.push(build_conditional(if_condition,call_instructions,else_instructions));
                    } else {
                        prologue.push("// need to run sub component".to_string());
                        prologue.push(format!("{};", sub_cmp_counter_decrease));
                        prologue.push(format!("assert(!({}));", sub_cmp_counter));
                        prologue.append(&mut call_instructions);
                    }
                    // end of case parallel

                    prologue.push(format!("}} else {{"));
                    
                    // case not parallel
                    let sub_cmp_call_name = if let LocationRule::Indexed { .. } = &data.dest {
                        format!("{}_run", my_template_header.unwrap())
                    } else {
                        format!("(*{}[{}])", function_table(), my_template_header.unwrap())
                    };
                    let mut call_instructions = vec![format!(
                        "{};",
                        build_call(sub_cmp_call_name, sub_cmp_call_arguments)
                    )];                   
                    if let StatusInput::Unknown = status {
                        let sub_cmp_counter_decrease_andcheck = format!("!({})",sub_cmp_counter_decrease);
                        let if_condition = vec![sub_cmp_counter_decrease_andcheck];
                        prologue.push("// run sub component if needed".to_string());
                        let else_instructions = vec![];
                        prologue.push(build_conditional(if_condition,call_instructions,else_instructions));
                    } else {
                        prologue.push("// need to run sub component".to_string());
                        prologue.push(format!("{};", sub_cmp_counter_decrease));
                        prologue.push(format!("assert(!({}));", sub_cmp_counter));
                        prologue.append(&mut call_instructions);
                    }

                    prologue.push(format!("}}"));
                }
			}
            } else {
			    assert!(false);
			}
		    }
                    _ => (),
                }
                if let AddressType::SubcmpSignal { .. } = &data.dest_address_type {
	             prologue.push(format!("}}"));
	        }
                result = "".to_string();
            }
        }
        //make the call with lvar dest, size)
        prologue.push("// end call bucket".to_string());
        prologue.push("}\n".to_string());
        (prologue, result)
    }
}

impl WriteCVM for CallBucket{
    fn produce_cvm(&self, producer: &mut CVMProducer) -> (Vec<String>, String) {
        use cvm_code_generator::*;
        use super::location_rule::*;
        let mut instructions = vec![];
        instructions.push(";; start of call bucket".to_string());
        // create lvar parameter
        let mut params = "".to_string();
        let mut i = 0;
        for p in &self.arguments {
            //instructions.push(format!(";; evaluating argument {}", i));
            let size = match &self.argument_types[i].size{
                SizeOption::Single(value) => *value,
                SizeOption::Multiple(_values) => {
		    assert!(false); //should never be the case!
		    0
                }
            };
            if size > 1 {
                if let Instruction::Load(load) = &**p {
                    let (mut instructions_p, lp) = load.src.produce_cvm(&load.address_type, &load.context,producer);
                    instructions.append(&mut instructions_p);
                    match lp {
                        ComputedAddress::Variable(rvar) => {
                            params = format!("{} ff.memory({},i64.{})", params, rvar, size);
                        }
                        ComputedAddress::Signal(rsig) => {
                            params = format!("{} signal({},i64.{})", params, rsig, size);
                        }
                        ComputedAddress::SubcmpSignal(rcmp,rsig) => {
                            params = format!("{} subcmpsignal({},{},i64.{})", params, rcmp, rsig, size);
                        }
                    }
                } else {
                    assert!(false); //should never be the case!
                }
            } else {
                let (mut instructions_value, src) = p.produce_cvm(producer);
                instructions.append(&mut instructions_value);
                params = format!("{} {}", params, src);
            }
            instructions.push(format!(";; end copying argument {}", i));
            i += 1;
        }
        let result = "".to_string();
        match &self.return_info {
            ReturnType::Intermediate { .. } => {
                assert!(!self.is_called_function_returning_array);
                assert!(false);
            }
            ReturnType::Final(data) => {
                let (mut instructions_dest, ldest) = data.dest.produce_cvm(&data.dest_address_type, &data.context,producer);
                let size = match &data.context.size {
                    SizeOption::Single(value) => format!("i64.{}",value),
                    SizeOption::Multiple(values) => {
                        let rsize = producer.fresh_var();
                        if let  ComputedAddress::SubcmpSignal(rcmp,_rsig) = &ldest {
                            let mut instructions_if_size = create_if_selection(&values, &rcmp, &rsize, producer);
                            instructions.append(&mut instructions_if_size);
                        } else {
                            assert!(false);
                        }
                        rsize
                    }
                };
                if producer.get_current_line() != self.line {
                    instructions.push(format!(";;line {}", self.line));
                    producer.set_current_line(self.line);
                }
                if !self.is_called_function_returning_array {
                    let call_res = producer.fresh_var();
                    instructions.push(format!("{} = ff.call ${} {}", call_res, self.symbol.clone(), params));
                    match ldest {
                        ComputedAddress::Variable(rvar) => {
                            instructions.push(format!("{} {} {}", storeff(), &rvar, &call_res));
                        }
                        ComputedAddress::Signal(rsig) => {
                            instructions.push(set_signal(&rsig, &call_res));
                        }
                        ComputedAddress::SubcmpSignal(rcmp,rsig) => {
                            if let AddressType::SubcmpSignal {input_information, .. } = &data.dest_address_type {
                                if let InputInformation::Input{status, needs_decrement} = input_information {
		                    if let StatusInput::NoLast = status {
			                // no need to run subcomponent
                                        if *needs_decrement{
                                            instructions.push(set_cmp_input_dec_no_last(&rcmp,&rsig, &call_res));
                                        } else {
                                            instructions.push(set_cmp_input_no_dec_no_last(&rcmp,&rsig, &call_res));
                                        }
                                    } else if let StatusInput::Last = status {
                                        instructions.push(set_cmp_input_and_run(&rcmp,&rsig, &call_res));
                                    } else {
                                        instructions.push(set_cmp_input_dec_and_check_run(&rcmp,&rsig, &call_res));
                                    }
                                } else {
                                    assert!(false);
                                }
                            } else {
                                assert!(false);
                            }
                        }
                    }
                } else {
                    let result_position;
                    if let  ComputedAddress::Variable(rvar) = &ldest {
                        result_position = rvar.clone();
                    } else {
                        result_position = producer.get_current_var_to_return_from_call();
                    }
                    instructions.push(format!("ff.mcall ${} {} {} {}", self.symbol.clone(), result_position, size.clone(), params));
                    instructions.append(&mut instructions_dest);
                    let src_value = producer.fresh_var();
                    let dest_location;
                    let dest_position = producer.fresh_var();
                    let instruction_get_src = format!("{} = {} {}", src_value, loadff(), result_position);
                    let mut instruction_set_dest = "".to_string();
                    let mut last_out = false;
                    let mut last_instructions = vec![];
                    match &ldest {
                        ComputedAddress::Variable(_rvar) => {
                            dest_location = "";
                        }
                        ComputedAddress::Signal(rsig) => {
                            dest_location = rsig;
                            instruction_set_dest = set_signal(&dest_position,&src_value);
                        }
                        ComputedAddress::SubcmpSignal(rcmp,rsig) => {
                            dest_location = rsig;
                            if let AddressType::SubcmpSignal {input_information, .. } = &data.dest_address_type {
                                if let InputInformation::Input{status, needs_decrement} = input_information {
		                    if let StatusInput::NoLast = status {
			                // no need to run subcomponent
                                        if *needs_decrement{
                                            instruction_set_dest = set_cmp_input_dec_no_last(&rcmp,&dest_position, &src_value);
                                        } else {
                                            instruction_set_dest = set_cmp_input_no_dec_no_last(&rcmp,&dest_position, &src_value);
                                        }
                                    } else if let StatusInput::Last = status {
                                        last_out = true;
                                        instruction_set_dest = set_cmp_input_no_dec_no_last(&rcmp,&dest_position, &src_value);
                                        last_instructions.push(instruction_get_src.clone());
                                        last_instructions.push(set_cmp_input_and_run(&rcmp,&dest_position, &src_value));
                                    } else {
                                        last_out = true;
                                        instruction_set_dest = set_cmp_input_dec_no_last(&rcmp,&dest_position, &src_value);
                                        last_instructions.push(instruction_get_src.clone());
                                        last_instructions.push(set_cmp_input_dec_and_check_run(&rcmp,&dest_position, &src_value));
                                    }
                                } else {
                                    assert!(false);
                                }
                            } else {
                                assert!(false);
                            }
                        }
                    }
                    if let  ComputedAddress::Variable(_rvar) = &ldest {
                    } else {
                        if producer.get_current_line() != self.line {
                            instructions.push(format!(";;line {}", self.line));
                            producer.set_current_line(self.line);
                        }
                        instructions.push(format!("{} = {}", &dest_position, &dest_location));
                        let mut has_zero = false;
                        let counter;
                        match &data.context.size{
	                    SizeOption::Single(value) => {
                                counter = producer.fresh_var();
		                instructions.push(format!("{} = i64.{}", counter, value.to_string()));
	                    }
	                    SizeOption::Multiple(values) => {
                                has_zero = values.iter().any(|e| e.1 == 0);
                                counter = size;
	                    }
	                };
                        if last_out {
                            instructions.push(format!("{} = {} {} i64.1", counter, sub64(), counter)); 
                        }
                        if has_zero && last_out {
                            instructions.push(format!("{} {} ", add_if64(), &counter));
                        }
                        instructions.push(add_loop());
                        instructions.push(format!("{} {} ", add_if64(), &counter));
                        instructions.push(instruction_get_src);
                        instructions.push(instruction_set_dest);
                        instructions.push(format!("{} = {} {} i64.1", &counter, sub64(), &counter));
                        instructions.push(format!("{} = {} {} i64.1", &result_position, add64(), &result_position));
                        instructions.push(format!("{} = {} {} i64.1", &dest_position ,add64(), &dest_position));
                        instructions.push(add_continue());
                        instructions.push(add_end());
                        instructions.push(add_break());
                        instructions.push(add_end());
                        instructions.append(&mut last_instructions);
                        if producer.get_current_line() != self.line {
                            instructions.push(format!(";;line {}", self.line));
                            producer.set_current_line(self.line);
                        }
                        //result = "".to_string();
                        if has_zero && last_out {
                            instructions.push(add_end());
                        }
                    }
                }
            }
        }
        //make the call with lvar dest, size)
        instructions.push(";; end call bucket".to_string());
        (instructions,result)
    }
}


