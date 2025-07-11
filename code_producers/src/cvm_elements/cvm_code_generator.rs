use super::*;
use num_bigint_dig::BigInt;
//use std::fs::File;
//use std::io::prelude::*;
//use std::path::PathBuf;

pub fn cvm_hexa(nbytes: usize, num: &BigInt) -> String {
    let inbytes = num.to_str_radix(16).to_string();
    let mut temp = "0".repeat(2 * nbytes - inbytes.len());
    temp.push_str(&inbytes);
    let mut res: String = "".to_string();
    for i in 0..nbytes {
        let mut aux = "\\".to_string();
        aux.push_str(&temp[2 * i..2 * i + 2]);
        aux.push_str(&res);
        res = aux;
    }
    res
}

pub fn merge_code(instructions: Vec<String>) -> String {
    let code = format!("{}\n", instructions.join("\n"));
    code
}

pub fn constant_i64(value: &str) -> CVMInstruction {
    format!("i64.const {}", value)
}
pub fn constant_ff(value: &str) -> CVMInstruction {
    format!("ff.const {}", value)
}

pub fn add64() -> CVMInstruction {
    "i64.add".to_string()
}
pub fn addff() -> CVMInstruction {
    "ff.add".to_string()
}
pub fn sub64() -> CVMInstruction {
    "i64.sub".to_string()
}
pub fn subff() -> CVMInstruction {
    "ff.sub".to_string()
}
pub fn mul64() -> CVMInstruction {
    "i64.mul".to_string()
}
pub fn mulff() -> CVMInstruction {
    "ff.mul".to_string()
}
pub fn div64() -> CVMInstruction {
    "i64.div".to_string()
}
pub fn rem64() -> CVMInstruction {
    "i64.rem".to_string()
}
pub fn idivff() -> CVMInstruction {
    "ff.idiv".to_string()
}
pub fn divff() -> CVMInstruction {
    "ff.div".to_string()
}
pub fn remff() -> CVMInstruction {
    "ff.rem".to_string()
}

pub fn powff() -> CVMInstruction {
    "ff.pow".to_string()
}
pub fn pow64() -> CVMInstruction {
    "64.pow".to_string()
}

pub fn extend_i64_ff() -> CVMInstruction {
    "ff.extend_i64".to_string()
}
pub fn wrap_ff_i64() -> CVMInstruction {
    "i64.wrap_ff".to_string()
}

pub fn load64() -> CVMInstruction {
    "i64.load".to_string()
}

pub fn loadff() -> CVMInstruction {
    "ff.load".to_string()
}

pub fn store64() -> CVMInstruction {
    "i64.store".to_string()
}

pub fn storeff() -> CVMInstruction {
    "ff.store".to_string()
}

pub fn get_signal(inx: &str) -> CVMInstruction {
    format!("get_signal {}", inx)
}

pub fn get_cmp_signal(cinx: &str, sinx: &str) -> CVMInstruction {
    format!("get_cmp_signal {} {}", cinx, sinx)
}

pub fn set_signal(inx: &str, value: &str) -> CVMInstruction {
    format!("set_signal {} {}", inx, value)
}
pub fn set_cmp_input_no_dec_no_last(cinx: &str, sinx: &str, value: &str) -> CVMInstruction {
    format!("set_cmp_input {} {} {}", cinx, sinx, value)
}
pub fn set_cmp_input_dec_no_last(cinx: &str, sinx: &str, value: &str) -> CVMInstruction {
    format!("set_cmp_input_cnt {} {} {}", cinx, sinx, value)
}
pub fn set_cmp_input_and_run(cinx: &str, sinx: &str, value: &str) -> CVMInstruction {
    format!("set_cmp_input_run {} {} {}", cinx, sinx, value)
}
pub fn set_cmp_input_dec_and_check_run(cinx: &str, sinx: &str, value: &str) -> CVMInstruction {
    format!("set_cmp_input_cnt_check {} {} {}", cinx, sinx, value)
}
/*
//The 𝗆𝖾𝗆𝗈𝗋𝗒.𝗌𝗂𝗓𝖾 instruction returns the current size of a memory.
pub fn memory_size() -> CVMInstruction {
    "memory.size".to_string()
}
//The 𝗆𝖾𝗆𝗈𝗋𝗒.𝗀𝗋𝗈𝗐 instruction grows memory by a given delta and returns the previous size, or −1 if enough memory cannot be allocated.
pub fn memory_grow() -> CVMInstruction {
    "memory.grow".to_string()
}
 */

pub fn shr64() -> CVMInstruction {
    "i64.shr".to_string()
}
pub fn shl64() -> CVMInstruction {
    "i64.shl".to_string()
}
pub fn shrff() -> CVMInstruction {
    "ff.shr".to_string()
}
pub fn shlff() -> CVMInstruction {
    "ff.shl".to_string()
}
pub fn call(to: &str) -> CVMInstruction {
    format!("call {}", to)
}

pub fn and64() -> CVMInstruction {
    "i64.and".to_string()
}
pub fn or64() -> CVMInstruction {
    "i64.or".to_string()
}
pub fn andff() -> CVMInstruction {
    "ff.and".to_string()
}
pub fn orff() -> CVMInstruction {
    "ff.or".to_string()
}

pub fn band64() -> CVMInstruction {
    "i64.band".to_string()
}
pub fn bor64() -> CVMInstruction {
    "i64.bor".to_string()
}
pub fn bxor64() -> CVMInstruction {
    "i64.bxor".to_string()
}
pub fn bnot64() -> CVMInstruction {
    "i64.bnot".to_string()
}
pub fn bandff() -> CVMInstruction {
    "ff.band".to_string()
}
pub fn borff() -> CVMInstruction {
    "ff.bor".to_string()
}
pub fn bxorff() -> CVMInstruction {
    "ff.bxor".to_string()
}
pub fn bnotff() -> CVMInstruction {
    "ff.bnot".to_string()
}

pub fn gt64() -> CVMInstruction {
    "i64.gt".to_string()
}
pub fn gtff() -> CVMInstruction {
    "ff.gt".to_string()
}
pub fn ge64() -> CVMInstruction {
    "i64.ge".to_string()
}
pub fn geff() -> CVMInstruction {
    "ff.ge".to_string()
}
pub fn lt64() -> CVMInstruction {
    "i64.lt".to_string()
}
pub fn ltff() -> CVMInstruction {
    "ff.lt".to_string()
}
pub fn le64() -> CVMInstruction {
    "i64.le".to_string()
}
pub fn leff() -> CVMInstruction {
    "ff.le".to_string()
}
pub fn eq64() -> CVMInstruction {
    "i64.eq".to_string()
}
pub fn eqff() -> CVMInstruction {
    "ff.eq".to_string()
}
pub fn neq64() -> CVMInstruction {
    "i64.neq".to_string()
}
pub fn neqff() -> CVMInstruction {
    "ff.neq".to_string()
}
pub fn eqz64() -> CVMInstruction {
    "i64.eqz".to_string()
}
pub fn eqzff() -> CVMInstruction {
    "ff.eqz".to_string()
}
pub fn add_loop() -> CVMInstruction {
    "loop".to_string()
}
pub fn add_break() -> CVMInstruction {
    "break".to_string()
}
pub fn add_continue() -> CVMInstruction {
    "continue".to_string()
}
pub fn br_if(value: &str) -> CVMInstruction {
    format!("br_if {}", value)
}
pub fn br(value: &str) -> CVMInstruction {
    format!("br {}", value)
}
pub fn add_if64() -> CVMInstruction {
    "i64.if".to_string()
}
pub fn add_ifff() -> CVMInstruction {
    "ff.if".to_string()
}
pub fn add_else() -> CVMInstruction {
    "else".to_string()
}
pub fn add_end() -> CVMInstruction {
    "end".to_string()
}
pub fn add_return() -> CVMInstruction {
    "return".to_string()
}

pub fn exception(code: &str) -> CVMInstruction {
    format!("error {}",code)
}

pub const RETURN_PARAM_SIZE: usize = 0; // 2 if i64 and ff are in the same memory 
//pub const RETURN_POSITION: &str = "spr"; 
//pub const FUNCTION_DESTINATION: &str = "destination"; 
//pub const FUNCTION_DESTINATION_SIZE: &str = "destination_size"; 

pub fn create_if_selection(
    values: &Vec<(usize, usize)>,
    rcmpid: &str,
    rresult: &str,
    producer: &mut CVMProducer
) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let tid = producer.fresh_var();
    instructions.push(format!("{} = get_template_id {}", tid, rcmpid));
    for i in 0..values.len() {
        let comp = producer.fresh_var();
	instructions.push(format!("{} = {} {} {}", comp, eq64(), tid, values[i].0));
	instructions.push(format!("{} {}", add_if64(), comp));
	instructions.push(format!("{} = i64.{}", rresult, values[i].1)); //Add corresponding size in list
	instructions.push(add_else());
    }
    instructions.push(format!("{} = i64.{}", rresult, 0)); //default o complete the last else
    for _i in 0..values.len() {
	instructions.push(add_end());
    }
    instructions
}


// ----- exception codes and other constants -----------------
/*
pub fn default_memory_for_stack_kib() -> usize {
    10
}

pub fn exception_code_singal_not_found() -> usize {
    1
}

pub fn exception_code_no_remaing_singals_to_set() -> usize {
    2
}

pub fn exception_code_singals_already_set() -> usize {
    3
}

pub fn exception_code_assert_fail() -> usize {
    4
}

pub fn exception_code_not_enough_memory() -> usize {
    5
}

pub fn exception_code_input_array_access_exeeds_size() -> usize {
    6
}
*/
//------------------ compute initial size of memory ---------------
/*
pub fn get_initial_size_of_memory(producer: &CVMProducer) -> usize {
    let n = (producer.get_var_stack_memory_start() + 65535) / 65536;
    n + default_memory_for_stack_kib()
}
*/
//------------------- generate all kinds of Data ------------------

/*
pub fn generate_hash_map(signal_name_list: &Vec<InputInfo>, size: usize) -> Vec<(u64, usize, usize)> {
    assert!(signal_name_list.len() <= size);
    let mut hash_map = vec![(0, 0, 0); size];
    for i in 0..signal_name_list.len() {
        let h = hasher(&signal_name_list[i].name);
        let mut p = h as usize %  size;
        while hash_map[p].1 != 0 {
            p = (p + 1) % size;
        }
        hash_map[p] = (h, signal_name_list[i].start, signal_name_list[i].size);
    }
    hash_map
}

pub fn generate_data_from_hash_map(map: &Vec<(u64, usize, usize)>) -> String {
    let mut hash_map_data = "".to_string();
    for (h, p, s) in map {
        hash_map_data.push_str(&wasm_hexa(8, &BigInt::from(*h))); //64bits 8 slots of 8bits
        hash_map_data.push_str(&wasm_hexa(4, &BigInt::from(*p))); //32bits 4 slots of 8bits
        hash_map_data.push_str(&wasm_hexa(4, &BigInt::from(*s))); //32bits 4 slots of 8bits
    }
    hash_map_data
}

pub fn generate_data_witness_to_signal_list(signal_list: &Vec<usize>) -> String {
    let mut signallist_data = "".to_string();
    for s in signal_list {
        signallist_data.push_str(&wasm_hexa(4, &BigInt::from(*s))); //32bits 4 slots of 8bits
    }
    signallist_data
}

pub fn generate_data_template_instance_to_io(
    producer: &CVMProducer,
    io_map: &TemplateInstanceIOMap,
) -> String {
    let mut io_map_data = "".to_string();
    let mut s = producer.get_io_signals_to_info_start();
    for c in 0..producer.get_number_of_template_instances() {
        match io_map.get(&c) {
            Some(value) => {
                io_map_data.push_str(&&wasm_hexa(4, &BigInt::from(s)));
                s += value.len() * 4;
            }
            None => io_map_data.push_str(&&wasm_hexa(4, &BigInt::from(0))),
        }
    }
    io_map_data
}

pub fn generate_data_io_signals_to_info(
    producer: &CVMProducer,
    io_map: &TemplateInstanceIOMap,
) -> String {
    let mut io_signals = "".to_string();
    let mut pos = producer.get_io_signals_info_start();
    for c in 0..producer.get_number_of_template_instances() {
        match io_map.get(&c) {
            Some(value) => {
                let mut n = 0;
                for s in value {
                    assert_eq!(s.code, n);
                    io_signals.push_str(&&wasm_hexa(4, &BigInt::from(pos)));
                    //do not store code and the first one of lengths (offset + size + length-1(if >0)
                    if s.lengths.len() == 0 { //only offset
                        pos += 4;
                    } else { // offest + length -1 + size
                        pos += s.lengths.len() * 4 + 4;
                    }
		    if let Some(_) = s.bus_id {
			pos += 4;
		    }
                    n += 1;
                }
            }
            None => (),
        }
    }
    io_signals
}

pub fn generate_data_io_signals_info(
    producer: &CVMProducer,
    io_map: &TemplateInstanceIOMap,
) -> String {
    let mut io_signals_info = "".to_string();
    for c in 0..producer.get_number_of_template_instances() {
        match io_map.get(&c) {
            Some(value) => {
 	       //println!("Template Instance: {}", c);
               for s in value {
                    // add the actual offset in memory, taking into account the size of field nums
                    //println!("Offset: {}", s.offset);
                    io_signals_info.push_str(&&wasm_hexa(
                        4,
                        &BigInt::from(s.offset * producer.get_size_32_bits_in_memory() * 4),
                    ));
                    //println!("Length: {}", s.lengths.len());
		    if s.lengths.len() > 0 { // if it is an array
                        // add the dimensions except the first one		    
                        for i in 1..s.lengths.len() {
                            //println!("Index: {}, {}", i, s.lengths[i]);
                            io_signals_info.push_str(&&wasm_hexa(4, &BigInt::from(s.lengths[i])));
                        }
                        // add the actual size of the elements
                        //println!("Size: {}", s.size);
                        io_signals_info.push_str(&&wasm_hexa(
                            4,
                            &BigInt::from(s.size),
                            //&BigInt::from(s.size * producer.get_size_32_bits_in_memory() * 4),
                        ));
		    }
		    // add the busid if it is a  bus
		    if let Some(value) = s.bus_id {
                            //println!("Bus_id: {}", value);
			    io_signals_info.push_str(&&wasm_hexa(4, &BigInt::from(value)));
		    }
                }
            }
            None => (),
        }
    }
    io_signals_info
}


pub fn generate_data_bus_instance_to_field(
    producer: &CVMProducer,
    field_map: &FieldMap,
) -> String {
    let mut field_map_data = "".to_string();
    let mut s = producer.get_field_to_info_start();
    for c in 0..producer.get_number_of_bus_instances() {
        field_map_data.push_str(&&wasm_hexa(4, &BigInt::from(s)));
        s += field_map[c].len() * 4;
    }
    field_map_data
}

pub fn generate_data_field_to_info(
    producer: &CVMProducer,
    field_map: &FieldMap,
) -> String {
    let mut bus_fields = "".to_string();
    let mut pos = producer.get_field_info_start();
    for c in 0..producer.get_number_of_bus_instances() {
        for s in &field_map[c] {
            bus_fields.push_str(&&wasm_hexa(4, &BigInt::from(pos)));
            //do not store the first one of lengths
            if s.dimensions.len() == 0 {
                pos += 4;
            } else {
                pos += s.dimensions.len() * 4 + 4;
            }
            if let Some(_) = s.bus_id {
               pos += 4;
	   }
        }
    }
    bus_fields
}

pub fn generate_data_field_info(
    producer: &CVMProducer,
    field_map: &FieldMap,
) -> String {
    let mut field_info = "".to_string();
    for c in 0..producer.get_number_of_bus_instances() {
 	//println!("Bus Instance: {}", c);
        for s in &field_map[c] {
            // add the actual offset in memory, taking into account the size of field nums
            //println!("Offset: {}", s.offset);
            field_info.push_str(&&wasm_hexa(
                4,
                &BigInt::from(s.offset * producer.get_size_32_bits_in_memory() * 4),
            ));
            //println!("Length: {}", s.dimensions.len());
	    if s.dimensions.len() > 0 { // if it is an array
		// add all dimensions but first one	    
		for i in 1..s.dimensions.len() {
                    //println!("Index: {}, {}", i, s.dimensions[i]);
                    field_info.push_str(&&wasm_hexa(4, &BigInt::from(s.dimensions[i])));
		}
		// add the actual size in memory, if array
                //println!("Size: {}", s.size);
		field_info.push_str(&&wasm_hexa(
                    4,
                    &BigInt::from(s.size),
                    //&BigInt::from(s.size * producer.get_size_32_bits_in_memory() * 4),
		));
	    }
            // add the busid if it contains buses
	    if let Some(value) = s.bus_id {
                //println!("Bus_id: {}", value);
		field_info.push_str(&&wasm_hexa(4, &BigInt::from(value)));
	    }
        }
    }
    field_info
}

pub fn generate_data_constants(producer: &CVMProducer, constant_list: &Vec<String>) -> String {
    let mut constant_list_data = "".to_string();
    //    For short/long form
    //    let szero = wasm_hexa(producer.get_size_32_bit()*4,&BigInt::from(0));
    for s in constant_list {
        /*
                // Only long form
                let n = s.parse::<BigInt>().unwrap();
                constant_list_data.push_str("\\00\\00\\00\\00\\00\\00\\00\\80");
                constant_list_data.push_str(&wasm_hexa(producer.get_size_32_bit()*4,&n));
        */
        //      For sort/long or short/montgomery
        let mut n = s.parse::<BigInt>().unwrap();
        let min_int = BigInt::from(-2147483648);
        let max_int = BigInt::from(2147483647);
        let p = producer.get_prime().parse::<BigInt>().unwrap();
        let b = ((p.bits() + 63) / 64) * 64;
        let mut r = BigInt::from(1);
        r = r << b;
        n = n % BigInt::clone(&p);
        n = n + BigInt::clone(&p);
        n = n % BigInt::clone(&p);
        let hp = BigInt::clone(&p) / 2;
        let mut nn;
        if BigInt::clone(&n) > hp {
            nn = BigInt::clone(&n) - BigInt::clone(&p);
        } else {
            nn = BigInt::clone(&n);
        }
        /*
                // short/long
                if min_int <= nn && nn <= max_int {
                // It is short
                    if nn < BigInt::from(0) {
                        nn = BigInt::parse_bytes(b"100000000", 16).unwrap() + nn;
                    }
                    constant_list_data.push_str(&wasm_hexa(4,&nn));
                    constant_list_data.push_str("\\00\\00\\00\\00");  // 0000
                    constant_list_data.push_str(&szero);
                } else {
                //It is long
                    constant_list_data.push_str("\\00\\00\\00\\00\\00\\00\\00\\80"); // 1000
                    constant_list_data.push_str(&wasm_hexa(producer.get_size_32_bit()*4,&n));
                }
        */
        //short/montgomery
        if min_int <= nn && nn <= max_int {
            // It is short. We have it in short & Montgomery
            if nn < BigInt::from(0) {
                nn = BigInt::parse_bytes(b"100000000", 16).unwrap() + nn;
            }
            constant_list_data.push_str(&wasm_hexa(4, &nn));
            constant_list_data.push_str("\\00\\00\\00\\40"); // 0100
        } else {
            //It is long. Only Montgomery
            constant_list_data.push_str("\\00\\00\\00\\00\\00\\00\\00\\C0"); // 1100
        }
        // Montgomery
        // n*R mod P
        n = (n * BigInt::clone(&r)) % BigInt::clone(&p);
        constant_list_data.push_str(&wasm_hexa(producer.get_size_32_bit() * 4, &n));
    }
    constant_list_data
}
*/

// ------ fix elements --------------------------

/*
pub fn generate_data_list(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut wdata = vec![];
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        0,
        wasm_hexa(4, &BigInt::from(producer.get_var_stack_memory_start()))
    ));
    let p = producer.get_prime().parse::<BigInt>().unwrap();
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_raw_prime_start(),
        wasm_hexa(producer.get_size_32_bit()*4, &p)
    ));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_shared_rw_memory_start() - 8,
        "\\00\\00\\00\\00\\00\\00\\00\\80"
    ));
    let map = generate_hash_map(&producer.get_main_input_list(),producer.get_input_hash_map_entry_size());
    wdata.push(format!(";; hash_map"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_input_signals_hashmap_start(),
        generate_data_from_hash_map(&map)
    ));
    let s = generate_data_witness_to_signal_list(producer.get_witness_to_signal_list());
    wdata.push(format!(";; witness_to_signal_list"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_witness_signal_id_list_start(),
        s
    ));
    wdata.push(format!(";; signal memory"));
    wdata.push(format!("(data (i32.const {}) \"{}{}\")",producer.get_signal_memory_start(),"\\00\\00\\00\\00\\00\\00\\00\\80",wasm_hexa(producer.get_size_32_bit()*4, &BigInt::from(1)))); //setting 'one' as long normal 1
    wdata.push(format!(";; template_instance_to_io_signal"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_template_instance_to_io_signal_start(),
        generate_data_template_instance_to_io(&producer, producer.get_io_map())
    ));
    wdata.push(format!(";; io_signals_to_info"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_io_signals_to_info_start(),
        generate_data_io_signals_to_info(&producer, producer.get_io_map())
    ));
    wdata.push(format!(";; io_signals_info"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_io_signals_info_start(),
        generate_data_io_signals_info(&producer, producer.get_io_map())
    ));
    wdata.push(format!(";; bus_instance_to_field"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_bus_instance_to_field_start(),
        generate_data_bus_instance_to_field(&producer, producer.get_busid_field_info())
    ));
    wdata.push(format!(";; field_to_info"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_field_to_info_start(),
        generate_data_field_to_info(&producer, producer.get_busid_field_info())
    ));
    wdata.push(format!(";; field_info"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_field_info_start(),
        generate_data_field_info(&producer, producer.get_busid_field_info())
    ));
    let ml = producer.get_message_list();
    let m = producer.get_message_list_start();
    wdata.push(format!(";; messages_in_bytes"));
    for i in 0..ml.len() {
        if ml[i].len() < producer.get_size_of_message_in_bytes() {
            wdata.push(format!(
                "(data (i32.const {}) \"{}\\00\")",
                m + i * producer.get_size_of_message_in_bytes(),
                ml[i]
            ));
        } else {
            wdata.push(format!(
                "(data (i32.const {}) \"{}\\00\")",
                m + i * producer.get_size_of_message_in_bytes(),
                &ml[i][..producer.get_size_of_message_in_bytes()-1]
            ));
        }
    }
    let st = producer.get_string_table();
    let s = producer.get_string_list_start();
    for i in 0..st.len() {
        if st[i].len() < producer.get_size_of_message_in_bytes() {
            wdata.push(format!(
                "(data (i32.const {}) \"{}\\00\")",
                s + i * producer.get_size_of_message_in_bytes(),
                st[i]
            ));
        } else {
            wdata.push(format!(
                "(data (i32.const {}) \"{}\\00\")",
                s + i * producer.get_size_of_message_in_bytes(),
                &st[i][..producer.get_size_of_message_in_bytes()-1]
            ));
        }
    }
    wdata.push(format!(";; constants"));
    wdata.push(format!(
        "(data (i32.const {}) \"{}\")",
        producer.get_constant_numbers_start(),
        generate_data_constants(&producer, producer.get_field_constant_list())
    ));
    wdata
}
*/
// ------ stack handling operations
/*
pub fn reserve_stack_fr(producer: &CVMProducer, nbytes: usize) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    instructions.push(set_constant(&nbytes.to_string()));
    instructions.push(call("$reserveStackFr"));
    instructions.push(set_local(producer.get_cstack_tag()));
    instructions
}

pub fn reserve_stack_fr_function_generator() -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $reserveStackFr (type $_t_i32ri32)".to_string();
    instructions.push(header);
    instructions.push(" (param $nbytes i32)".to_string());
    instructions.push("(result i32)".to_string());
    instructions.push(" (local $inistack i32)".to_string());
    instructions.push(" (local $newbsize i32)".to_string());
    instructions.push(" (local $memorybsize i32)".to_string());
    instructions.push(set_constant("0"));
    instructions.push(load32(None));
    instructions.push(set_local("$inistack"));
    instructions.push(get_local("$inistack"));
    instructions.push(get_local("$nbytes"));
    instructions.push(add32());
    instructions.push(set_local("$newbsize"));
    instructions.push(set_constant("0"));
    instructions.push(get_local("$newbsize"));
    instructions.push(store32(None));
    // check if enough memory; otherwise grow
    // bytes per page 64 * 1024 = 2^16
    instructions.push(memory_size());
    instructions.push(set_constant("16"));
    instructions.push(shl32());
    instructions.push(set_local("$memorybsize"));
    instructions.push(get_local("$newbsize"));
    instructions.push(get_local("$memorybsize"));
    instructions.push(gt32_u());
    instructions.push(add_if());
    instructions.push(get_local("$newbsize"));
    instructions.push(get_local("$memorybsize"));
    instructions.push(sub32());
    instructions.push(set_constant("65535")); //64KiB-1
    instructions.push(add32());
    instructions.push(set_constant("16"));
    instructions.push(shr32_u()); //needed pages
    instructions.push(memory_grow());
    instructions.push(set_constant("-1"));
    instructions.push(eq32());
    instructions.push(add_if());
    instructions.push(set_constant(&exception_code_not_enough_memory().to_string()));
    instructions.push(call("$exceptionHandler"));
    instructions.push(add_end());
    instructions.push(add_end());
    instructions.push(get_local("$inistack"));
    instructions.push(")".to_string());
    instructions
}

pub fn free_stack(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    instructions.push(set_constant("0"));
    instructions.push(get_local(producer.get_cstack_tag()));
    instructions.push(store32(Option::None));
    instructions
}
*/
// ---------------------- functions ------------------------
/*
pub fn desp_io_subcomponent_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getOffsetIOSubComponet (type $_t_i32i32ri32)".to_string();
    instructions.push(header);
    instructions.push(" (param $comp i32)".to_string());
    instructions.push(" (param $ios i32)".to_string());
    instructions.push("(result i32)".to_string());
    instructions
        .push(set_constant(&producer.get_template_instance_to_io_signal_start().to_string()));
    instructions.push(get_local("$comp"));
    instructions.push(add32());
    instructions.push(load32(None));
    instructions.push(get_local("$ios"));
    instructions.push(set_constant("4"));
    instructions.push(mul32());
    instructions.push(add32());
    instructions.push(load32(None));
    instructions.push(")".to_string());
    instructions
}

pub fn get_shared_rw_memory_start_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getSharedRWMemoryStart (type $_t_ri32)".to_string();
    instructions.push(header);
    instructions.push("(result i32)".to_string());
    instructions.push(set_constant(&producer.get_shared_rw_memory_start().to_string()));
    instructions.push(")".to_string());
    instructions
}

pub fn read_shared_rw_memory_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $readSharedRWMemory (type $_t_i32ri32)".to_string();
    instructions.push(header);
    instructions.push(" (param $p i32)".to_string());
    instructions.push("(result i32)".to_string());
    instructions.push(set_constant(&producer.get_shared_rw_memory_start().to_string()));
    instructions.push(get_local("$p"));
    instructions.push(set_constant("4"));
    instructions.push(mul32());
    instructions.push(add32());
    instructions.push(load32(None));
    instructions.push(")".to_string());
    instructions
}

pub fn write_shared_rw_memory_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $writeSharedRWMemory (type $_t_i32i32)".to_string();
    instructions.push(header);
    instructions.push(" (param $p i32)".to_string());
    instructions.push(" (param $v i32)".to_string());
    instructions.push(set_constant(&producer.get_shared_rw_memory_start().to_string()));
    instructions.push(get_local("$p"));
    instructions.push(set_constant("4"));
    instructions.push(mul32());
    instructions.push(add32());
    instructions.push(get_local("$v"));
    instructions.push(store32(None));
    instructions.push(")".to_string());
    instructions
}

pub fn get_version_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getVersion (type $_t_ri32)".to_string();
    instructions.push(header);
    instructions.push(set_constant(&producer.get_version().to_string()));
    instructions.push(")".to_string());
    let header = "(func $getMinorVersion (type $_t_ri32)".to_string();
    instructions.push(header);
    instructions.push(set_constant(&producer.get_minor_version().to_string()));
    instructions.push(")".to_string());
    let header = "(func $getPatchVersion (type $_t_ri32)".to_string();
    instructions.push(header);
    instructions.push(set_constant(&producer.get_patch_version().to_string()));
    instructions.push(")".to_string());
    instructions
}

pub fn init_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $init (type $_t_i32)".to_string();
    instructions.push(header);
    instructions.push(" (param $t i32)".to_string());
    instructions.push(" (local $i i32)".to_string());
    instructions.push(format!(" (local {} i32)", producer.get_merror_tag()));
    // initialize set counter
    instructions.push(set_constant(&producer.get_remaining_input_signal_counter().to_string()));
    instructions.push(";; Number of Main inputs".to_string());
    instructions.push(set_constant(&producer.get_number_of_main_inputs().to_string()));
    instructions.push(store32(None));
    // initialize set positions
    instructions.push(set_constant(&producer.get_input_signal_set_map_start().to_string()));
    instructions.push(set_local("$i"));
    instructions.push(add_block()); //block 1
    instructions.push(add_loop()); //loop 2
    instructions.push(get_local("$i"));
    let end_pos =
        producer.get_input_signal_set_map_start() + 4 * producer.get_number_of_main_inputs();
    instructions.push(set_constant(&end_pos.to_string()));
    instructions.push(eq32());
    instructions.push(br_if("1"));
    instructions.push(get_local("$i"));
    instructions.push(set_constant("0"));
    instructions.push(store32(None));
    instructions.push(get_local("$i"));
    instructions.push(set_constant("4"));
    instructions.push(add32());
    instructions.push(set_local("$i"));
    instructions.push(br("0"));
    instructions.push(add_end()); //end loop 2
    instructions.push(add_end()); //end block 1
                                  // initialize component_free_pos
    instructions.push(set_constant(&producer.get_component_free_pos().to_string()));
    instructions.push(set_constant(&producer.get_component_tree_start().to_string()));
    instructions.push(store32(None));
    //signal offset of the main component
    let next_to_one = producer.get_signal_memory_start()
        + producer.get_main_signal_offset() * producer.get_size_32_bits_in_memory() * 4;
    //    // initialize signal_free_pos
    //    instructions.push(set_constant(&producer.get_signal_free_pos().to_string()));
    //    instructions.push(set_constant(&next_to_one.to_string()));
    //    instructions.push(store32(None));
    instructions.push(set_constant(&next_to_one.to_string()));
    let funcname = format!("${}_create", producer.get_main_header());
    instructions.push(call(&funcname));    
    instructions.push(drop());
    if producer.get_number_of_main_inputs() == 0 {
    instructions.push(set_constant(&producer.get_component_tree_start().to_string()));
    let funcname = format!("${}_run", producer.get_main_header());
    instructions.push(call(&funcname));
    instructions.push(tee_local(producer.get_merror_tag()));
    instructions.push(add_if()); 
    instructions.push(get_local("$merror"));    
    instructions.push(call("$exceptionHandler"));
    instructions.push(add_end());
    }
    instructions.push(")".to_string());
    instructions
}

pub fn get_input_signal_map_position_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getInputSignalMapPosition (type $_t_i64ri32)".to_string();
    let sizeones = producer.get_input_hash_map_entry_size()-1;
    instructions.push(header);
    instructions.push(" (param $hn i64)".to_string());
    instructions.push("(result i32)".to_string());
    instructions.push(" (local $ini i32)".to_string());
    instructions.push(" (local $i i32)".to_string());
    instructions.push(" (local $aux i32)".to_string());
    instructions.push(get_local("$hn"));
    instructions.push(wrap_i6432());
    instructions.push(set_constant(&sizeones.to_string()));
    instructions.push(and32());
    instructions.push(set_local("$ini"));
    instructions.push(get_local("$ini"));
    instructions.push(set_local("$i"));
    instructions.push(add_block()); // block 1
    instructions.push(add_loop()); // loop 2
    instructions.push(set_constant(&producer.get_input_signals_hashmap_start().to_string()));
    instructions.push(get_local("$i"));
    instructions.push(set_constant("16")); // 8(h)+4(p)+4(s)
    instructions.push(mul32());
    instructions.push(add32());
    instructions.push(set_local("$aux"));
    instructions.push(get_local("$aux"));
    instructions.push(load64(None));
    instructions.push(get_local("$hn"));
    instructions.push(eq64());
    instructions.push(add_if()); // if 3
    instructions.push(get_local("$aux"));
    instructions.push(add_return());
    instructions.push(add_end()); // end if 3
    instructions.push(get_local("$aux"));
    instructions.push(load64(None));
    instructions.push(eqz64());
    instructions.push(add_if()); // if 4
    instructions.push(set_constant("0")); // error
    instructions.push(add_return());
    instructions.push(add_end()); // end if 4
    instructions.push(get_local("$i"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_constant(&sizeones.to_string()));
    instructions.push(and32());
    instructions.push(set_local("$i"));
    instructions.push(get_local("$i"));
    instructions.push(get_local("$ini"));
    instructions.push(eq32());
    instructions.push(add_if()); //if 5
    instructions.push(set_constant("0")); // error
    instructions.push(add_return());
    instructions.push(add_end()); // end if 5
    instructions.push(br("0"));
    instructions.push(add_end()); // end loop 2
    instructions.push(add_end()); // end block 1
    instructions.push(set_constant("0"));
    instructions.push(")".to_string());
    instructions
}

pub fn check_if_input_signal_set_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $checkIfInputSignalSet (type $_t_i32ri32)".to_string();
    instructions.push(header);
    instructions.push(" (param $sip i32)".to_string());
    instructions.push("(result i32)".to_string());
    instructions.push(set_constant(&producer.get_input_signal_set_map_start().to_string()));
    instructions.push(get_local("$sip"));
    instructions.push(add32());
    instructions.push(load32(None));
    instructions.push(")".to_string());
    instructions
}

pub fn set_input_signal_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let mut code_aux = get_input_signal_map_position_generator(&producer);
    instructions.append(&mut code_aux);
    code_aux = check_if_input_signal_set_generator(&producer);
    instructions.append(&mut code_aux);
    let header = "(func $setInputSignal (type $_t_i32i32i32)".to_string();
    instructions.push(header);
    instructions.push(" (param $hmsb i32)".to_string());
    instructions.push(" (param $hlsb i32)".to_string());
    instructions.push(" (param $pos i32)".to_string());
    instructions.push(" (local $ns i32) ;; number of signals to set".to_string());
    instructions.push(" (local $mp i32) ;; map position".to_string());
    instructions.push(" (local $sip i32) ;; signal+position number".to_string());
    instructions.push(" (local $sipm i32) ;; position in the signal memory".to_string());
    instructions.push(" (local $vint i32)".to_string());
    instructions.push(format!(" (local {} i32)", producer.get_merror_tag()));
    instructions.push(set_constant(&producer.get_remaining_input_signal_counter().to_string()));
    instructions.push(load32(None));
    instructions.push(set_local("$ns"));
    instructions.push(get_local("$ns"));
    instructions.push(eqz32());
    instructions.push(add_if()); // if 1
    instructions.push(set_constant(&exception_code_no_remaing_singals_to_set().to_string()));
    instructions.push(call("$exceptionHandler"));
    instructions.push(add_else()); // else if 1
    instructions.push(get_local("$hmsb"));
    instructions.push(extend_i32_u64());
    instructions.push(set_constant_64("32"));
    instructions.push(shl64());
    instructions.push(get_local("$hlsb"));
    instructions.push(extend_i32_u64());
    instructions.push(or64());
    instructions.push(call("$getInputSignalMapPosition"));
    instructions.push(tee_local("$mp"));
    instructions.push(eqz32());
    instructions.push(add_if()); // if 2
    instructions.push(set_constant(&exception_code_singal_not_found().to_string()));
    instructions.push(call("$exceptionHandler"));
    instructions.push(add_else()); // else if 2
    instructions.push(get_local("$pos"));
    instructions.push(get_local("$mp"));
    instructions.push(load32(Some("12"))); // load the second component (signal size)
    instructions.push(ge32_u());
    instructions.push(add_if()); // if 3
    instructions.push(set_constant(&exception_code_input_array_access_exeeds_size().to_string()));
    instructions.push(call("$exceptionHandler"));
    instructions.push(add_else()); // else if 3    
    instructions.push(get_local("$mp"));
    instructions.push(load32(Some("8"))); // load the first component (signal position)
    instructions.push(get_local("$pos"));
    instructions.push(add32());
    instructions.push(tee_local("$sip"));
    let o = producer.get_number_of_main_outputs() + 1;
    instructions.push(set_constant(&o.to_string()));
    instructions.push(sub32());
    instructions.push(call("$checkIfInputSignalSet"));
    instructions.push(add_if()); // if 4
    instructions.push(set_constant(&exception_code_singals_already_set().to_string()));
    instructions.push(call("$exceptionHandler"));
    instructions.push(add_else()); // else if 4
    instructions.push(get_local("$sip"));
    let s = producer.get_size_32_bits_in_memory() * 4;
    instructions.push(set_constant(&s.to_string()));
    instructions.push(mul32());
    instructions.push(set_constant(&producer.get_signal_memory_start().to_string()));
    instructions.push(add32()); // address of the signal in memory
    instructions.push(set_local("$sipm"));
    instructions.push(get_local("$sipm"));
    let p_fr_rw_memory = producer.get_shared_rw_memory_start() - 8; // address of the shared memory as Fr
    instructions.push(set_constant(&p_fr_rw_memory.to_string()));
    instructions.push(call("$Fr_toInt")); // value as Int (if Int)
    instructions.push(set_local("$vint"));
    instructions.push(get_local("$vint"));
    instructions.push(store32(None));
    instructions.push(get_local("$sipm"));
    instructions.push(set_constant("0"));
    instructions.push(store32(Some("4")));
    instructions.push(get_local("$sipm"));
    instructions.push(set_constant("8"));
    instructions.push(add32());
    instructions.push(call("$Fr_int_zero")); // sets zeros in the long positions
    instructions.push(get_local("$sipm"));
    instructions.push(set_constant(&p_fr_rw_memory.to_string())); // address of the shared memory as Fr
    instructions.push(call("$Fr_eqR"));
    instructions.push(add_if()); // if 5
    instructions.push(get_local("$sipm"));
    instructions.push(get_local("$vint"));
    instructions.push(store32(None));
    instructions.push(get_local("$sipm"));
    instructions.push(set_constant("0"));
    instructions.push(store32(Some("4")));
    instructions.push(get_local("$sipm"));
    instructions.push(set_constant("8"));
    instructions.push(add32());
    instructions.push(call("$Fr_int_zero")); // sets zeros in the long positions
    instructions.push(add_else()); // else if 5
    instructions.push(get_local("$sipm"));
    instructions.push(set_constant(&p_fr_rw_memory.to_string())); // address of the shared memory as Fr
    instructions.push(call("$Fr_copy"));
    instructions.push(add_end()); // end else if 5
    instructions.push(get_local("$ns"));
    instructions.push(set_constant("-1"));
    instructions.push(add32());
    instructions.push(set_local("$ns"));
    instructions.push(set_constant(&producer.get_remaining_input_signal_counter().to_string()));
    instructions.push(get_local("$ns"));
    instructions.push(store32(None));
    instructions.push(get_local("$ns"));
    instructions.push(eqz32());
    instructions.push(add_if()); // if 6
    instructions.push(set_constant(&producer.get_component_tree_start().to_string()));
    let funcname = format!("${}_run", producer.get_main_header());
    instructions.push(call(&funcname));
    instructions.push(tee_local(producer.get_merror_tag()));
    instructions.push(add_if()); // if 7
    instructions.push(get_local("$merror"));    
    instructions.push(call("$exceptionHandler"));
    instructions.push(add_end()); // end if 7
    instructions.push(add_end()); // end if 6
    instructions.push(add_end()); // end else if 4
    instructions.push(add_end()); // end else if 3
    instructions.push(add_end()); // end else if 2
    instructions.push(add_end()); // end else if 1
    instructions.push(")".to_string());
    instructions
}

pub fn get_input_signal_size_generator(_producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getInputSignalSize (type $_t_i32i32ri32)".to_string();
    instructions.push(header);
    instructions.push(" (param $hmsb i32)".to_string());
    instructions.push(" (param $hlsb i32)".to_string());
    instructions.push("(result i32)".to_string());
    instructions.push(get_local("$hmsb"));
    instructions.push(extend_i32_u64());
    instructions.push(set_constant_64("32"));
    instructions.push(shl64());
    instructions.push(get_local("$hlsb"));
    instructions.push(extend_i32_u64());
    instructions.push(or64());
    instructions.push(call("$getInputSignalMapPosition"));
    instructions.push(load32(Some("12")));
    instructions.push(")".to_string());
    instructions
}

pub fn get_raw_prime_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getRawPrime (type $_t_void)".to_string();
    instructions.push(header);
    instructions.push(set_constant(&producer.get_raw_prime_start().to_string())); // address of the raw prime number
    instructions.push(set_constant(&producer.get_shared_rw_memory_start().to_string())); // address of the shared memory
    instructions.push(call("$Fr_int_copy"));
    instructions.push(")".to_string());
    instructions
}

pub fn get_field_num_len32_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getFieldNumLen32 (type $_t_ri32)".to_string();
    instructions.push(header);
    instructions.push("(result i32)".to_string());
    instructions.push(set_constant(&producer.get_size_32_bit().to_string()));
    instructions.push(")".to_string());
    instructions
}

pub fn get_input_size_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getInputSize (type $_t_ri32)".to_string();
    instructions.push(header);
    instructions.push("(result i32)".to_string());
    instructions.push(set_constant(&producer.get_number_of_main_inputs().to_string()));
    instructions.push(")".to_string());
    instructions
}

pub fn get_witness_size_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getWitnessSize (type $_t_ri32)".to_string();
    instructions.push(header);
    instructions.push("(result i32)".to_string());
    instructions.push(set_constant(&producer.get_number_of_witness().to_string()));
    instructions.push(")".to_string());
    instructions
}

pub fn copy_32_in_shared_rw_memory_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $copy32inSharedRWMemory (type $_t_i32)".to_string(); //receives i32 to be put in 0 of SharedRWMemory
    instructions.push(header);
    instructions.push(" (param $p i32)".to_string());
    instructions.push(set_constant(&producer.get_shared_rw_memory_start().to_string()));
    instructions.push(get_local("$p"));
    instructions.push(store32(None));
    instructions.push(set_constant(&producer.get_shared_rw_memory_start().to_string()));
    instructions.push(set_constant("0"));
    instructions.push(store32(Some("4")));
    for i in 1..producer.get_size_32_bit()/2 {
	let pos = 8*i;
	instructions.push(set_constant(&producer.get_shared_rw_memory_start().to_string()));
	instructions.push(set_constant_64("0"));
	instructions.push(store64(Some(&pos.to_string())));
    }
    instructions.push(")".to_string());
    instructions
}

pub fn copy_fr_in_shared_rw_memory_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $copyFr2SharedRWMemory (type $_t_i32)".to_string(); //receives address to be copied
    instructions.push(header);
    instructions.push(" (param $p i32)".to_string());
    let pos = producer.get_shared_rw_memory_start() - 8;
    instructions.push(set_constant(&pos.to_string()));
    instructions.push(get_local("$p"));
    instructions.push(call("$Fr_copy"));
    instructions.push(set_constant(&pos.to_string()));
    instructions.push(call("$Fr_toLongNormal"));
    instructions.push(")".to_string());
    instructions
}

pub fn get_witness_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getWitness (type $_t_i32)".to_string();
    instructions.push(header);
    instructions.push(" (param $p i32)".to_string());
    instructions.push(" (local $c i32)".to_string());
    instructions.push(set_constant(&producer.get_witness_signal_id_list_start().to_string()));
    instructions.push(get_local("$p"));
    instructions.push(set_constant("2")); // 32 bytes per witness
    instructions.push(shl32());
    instructions.push(add32()); // address of the witness in the witness list
    instructions.push(load32(None)); // number of the signal in the signal Memory
    instructions.push(set_constant(&format!("{}",producer.get_size_32_bit()*4+8)));//40
    instructions.push(mul32());
    instructions.push(set_constant(&producer.get_signal_memory_start().to_string()));
    instructions.push(add32()); // address of the signal in the signal Memory
    instructions.push(set_local("$c"));
    let pos = producer.get_shared_rw_memory_start() - 8;
    instructions.push(set_constant(&pos.to_string()));
    instructions.push(get_local("$c"));
    instructions.push(call("$Fr_copy"));
    instructions.push(set_constant(&pos.to_string()));
    instructions.push(call("$Fr_toLongNormal"));
    instructions.push(")".to_string());
    instructions
}

pub fn get_message_char_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $getMessageChar (type $_t_ri32)".to_string();
    instructions.push(header);
    instructions.push(" (local $c i32)".to_string());
    instructions.push(set_constant(&producer.get_message_buffer_counter_position().to_string()));
    instructions.push(load32(None)); // current position in buffer
    instructions.push(set_local("$c"));
    instructions.push(get_local("$c"));
    instructions.push(set_constant(&producer.get_size_of_message_buffer_in_bytes().to_string()));
    instructions.push(ge32_u());
    instructions.push(add_if());
    instructions.push(set_constant("0"));
    instructions.push(add_return());
    instructions.push(add_else());
    instructions.push(set_constant(&producer.get_message_buffer_start().to_string()));
    instructions.push(get_local("$c"));
    instructions.push(add32());
    instructions.push(load32_8u(None));
    instructions.push(set_constant(&producer.get_message_buffer_counter_position().to_string()));
    instructions.push(get_local("$c"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(store32(None)); // new current position in buffer
    instructions.push(add_return());
    instructions.push(add_end());
    instructions.push(set_constant("0"));
    instructions.push(")".to_string());
    instructions
}

pub fn build_log_message_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $buildLogMessage (type $_t_i32)".to_string();
    instructions.push(header);
    instructions.push(" (param $m i32)".to_string()); //string position
    instructions.push(" (local $em i32)".to_string()); //position in error message
    instructions.push(" (local $bm i32)".to_string()); //position in buffer
    instructions.push(" (local $mc i32)".to_string()); //message char
    instructions.push(get_local("$m"));
    instructions.push(set_local("$em"));
    instructions.push(set_constant(&producer.get_message_buffer_start().to_string()));
    instructions.push(set_local("$bm"));
    instructions.push(add_block());
    instructions.push(add_loop()); //move bytes until end of message or zero found
                                   // check if end of message
    let final_pos = producer.get_size_of_message_in_bytes() + producer.get_message_buffer_start();
    instructions.push(set_constant(&final_pos.to_string()));
    instructions.push(get_local("$em"));
    instructions.push(eq32());
    instructions.push(br_if("1")); // jump to end of block 1
    instructions.push(get_local("$em"));
    instructions.push(load32_8u(None));
    instructions.push(set_local("$mc"));
    instructions.push(get_local("$mc"));
    instructions.push(eqz32());
    instructions.push(br_if("1")); // jump to end of block 1
    instructions.push(get_local("$bm"));
    instructions.push(get_local("$mc"));
    instructions.push(store32_8(None));
    instructions.push(get_local("$em"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$em"));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(br("0"));
    instructions.push(add_end());
    instructions.push(add_end());
    //fill rest of buffer with 0's
    instructions.push(add_block());
    instructions.push(add_loop());
    instructions.push(get_local("$bm"));
    let buff_final_pos =
        producer.get_message_buffer_start() + producer.get_size_of_message_buffer_in_bytes();
    instructions.push(set_constant(&buff_final_pos.to_string()));
    instructions.push(eq32());
    instructions.push(br_if("1")); //jump to the end of block
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0"));
    instructions.push(store32_8(None)); // stores the digit in the buffer
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(br("0")); // jump to the loop
    instructions.push(add_end());
    instructions.push(add_end());
    // initialize message buffer position to 0
    instructions.push(set_constant(&producer.get_message_buffer_counter_position().to_string()));
    instructions.push(set_constant("0"));
    instructions.push(store32(None));
    instructions.push(")".to_string());
    instructions
}

pub fn build_buffer_message_generator(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    let header = "(func $buildBufferMessage (type $_t_i32i32)".to_string();
    instructions.push(header);
    instructions.push(" (param $m i32)".to_string()); //message id
    instructions.push(" (param $l i32)".to_string()); //line
    instructions.push(" (local $em i32)".to_string()); //position in error message
    instructions.push(" (local $bm i32)".to_string()); //position in buffer
    instructions.push(" (local $mc i32)".to_string()); //message char
    instructions.push(" (local $p10 i32)".to_string()); //power of 10
    instructions.push(set_constant(&producer.get_message_list_start().to_string()));
    instructions.push(get_local("$m"));
    instructions.push(set_constant(&producer.get_size_of_message_in_bytes().to_string()));
    instructions.push(mul32());
    instructions.push(add32());
    instructions.push(set_local("$em"));
    instructions.push(set_constant(&producer.get_message_buffer_start().to_string()));
    instructions.push(set_local("$bm"));
    instructions.push(add_block());
    instructions.push(add_loop()); //move bytes until end of message or zero found
                                   // check if end of message
    let final_pos = producer.get_size_of_message_in_bytes() + producer.get_message_buffer_start();
    instructions.push(set_constant(&final_pos.to_string()));
    instructions.push(get_local("$em"));
    instructions.push(eq32());
    instructions.push(br_if("1")); // jump to end of block 1
    instructions.push(get_local("$em"));
    instructions.push(load32_8u(None));
    instructions.push(set_local("$mc"));
    instructions.push(get_local("$mc"));
    instructions.push(eqz32());
    instructions.push(br_if("1")); // jump to end of block 1
    instructions.push(get_local("$bm"));
    instructions.push(get_local("$mc"));
    instructions.push(store32_8(None));
    instructions.push(get_local("$em"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$em"));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(br("0"));
    instructions.push(add_end());
    instructions.push(add_end());
    //adding the line " line: "
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0x20")); //space
    instructions.push(store32_8(None));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0x6C")); //l
    instructions.push(store32_8(None));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0x69")); //i
    instructions.push(store32_8(None));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0x6E")); //n
    instructions.push(store32_8(None));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0x65")); //e
    instructions.push(store32_8(None));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0x3A")); //:
    instructions.push(store32_8(None));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0x20")); //space
    instructions.push(store32_8(None));
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    //adding the line number
    //compute the power of 10 with the number of digits
    instructions.push(set_constant("1"));
    instructions.push(set_local("$p10"));
    instructions.push(add_block());
    instructions.push(add_loop());
    //check if $p10 * 10 > $l
    instructions.push(get_local("$p10"));
    instructions.push(set_constant("10"));
    instructions.push(mul32());
    instructions.push(get_local("$l"));
    instructions.push(gt32_u());
    instructions.push(br_if("1")); // jump to end of block 1
    instructions.push(get_local("$p10"));
    instructions.push(set_constant("10"));
    instructions.push(mul32());
    instructions.push(set_local("$p10"));
    instructions.push(br("0")); // jump to the loop
    instructions.push(add_end());
    instructions.push(add_end());

    //now we extract the digits and add them to buffer. We assume line > 0
    instructions.push(add_block());
    instructions.push(add_loop());
    //check if $p10 != 0
    instructions.push(get_local("$p10"));
    instructions.push(eqz32());
    instructions.push(br_if("1")); // jump to end of block 1
    instructions.push(get_local("$bm")); //next position in the buffer
                                         //get the next digit left-to-right
    instructions.push(get_local("$l"));
    instructions.push(get_local("$p10"));
    instructions.push(div32_u()); // highest digit
    instructions.push(set_constant("0x30"));
    instructions.push(add32()); // hex of the digit
    instructions.push(store32_8(None)); // stores the digit in the buffer
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(get_local("$l"));
    instructions.push(get_local("$p10"));
    instructions.push(rem32_u()); // remove the highest digit
    instructions.push(set_local("$l"));
    instructions.push(get_local("$p10"));
    instructions.push(set_constant("10"));
    instructions.push(div32_u()); // decrease power of 10
    instructions.push(set_local("$p10"));
    instructions.push(br("0")); // jump to the loop
    instructions.push(add_end());
    instructions.push(add_end());
    //fill rest of buffer with 0's
    instructions.push(add_block());
    instructions.push(add_loop());
    instructions.push(get_local("$bm"));
    let buff_final_pos =
        producer.get_message_buffer_start() + producer.get_size_of_message_buffer_in_bytes();
    instructions.push(set_constant(&buff_final_pos.to_string()));
    instructions.push(eq32());
    instructions.push(br_if("1")); //jump to the end of block
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("0"));
    instructions.push(store32_8(None)); // stores the digit in the buffer
    instructions.push(get_local("$bm"));
    instructions.push(set_constant("1"));
    instructions.push(add32());
    instructions.push(set_local("$bm"));
    instructions.push(br("0")); // jump to the loop
    instructions.push(add_end());
    instructions.push(add_end());
    // initialize message buffer position to 0
    instructions.push(set_constant(&producer.get_message_buffer_counter_position().to_string()));
    instructions.push(set_constant("0"));
    instructions.push(store32(None));
    instructions.push(")".to_string());
    instructions
}

pub fn generate_table_of_template_runs(producer: &CVMProducer) -> Vec<CVMInstruction> {
    let mut instructions = vec![];
    //    if !producer.get_io_map().is_empty() {
    let tlen = producer.get_template_instance_list().len();
    instructions.push(format!("(table $runsmap {} {} funcref)", tlen, tlen));
    instructions.push("(elem $runsmap (i32.const 0)".to_string());
    for i in 0..tlen {
        instructions.push(format!(" ${}_run", producer.get_template_instance_list()[i]));
    }
    instructions.push(")".to_string());
    //    }
    instructions
}

//  need list io (id,template_name)
//  (table $map _num funcref)
//  (elem $map (i32.const 0) $mmmm_run $mmmm_run    )
//  data...


fn get_file_instructions(name: &str) -> Vec<CVMInstruction> {
    use std::io::BufReader;
    use std::path::Path;
    let mut instructions = vec![];
    let path = format!("./{}.wat", name);
    if Path::new(&path).exists() {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for rline in reader.lines() {
            if let Result::Ok(line) = rline {
                instructions.push(line);
            }
        }
    } else {
        panic!("FILE NOT FOUND {}", name);
    }
    instructions
}

pub fn fr_types(prime: &String) -> Vec<CVMInstruction> {

    // TODO
    Vec::new()
}

pub fn fr_data(prime: &String) -> Vec<CVMInstruction> {
    // TODO
    Vec::new()
}
pub fn fr_code(prime: &String) -> Vec<CVMInstruction> {
    // TODO
    Vec::new()
}
*/
/*
pub fn generate_utils_js_file(js_folder: &PathBuf) -> std::io::Result<()> {
    use std::io::BufWriter;
    let mut file_path  = js_folder.clone();
    file_path.push("utils");
    file_path.set_extension("js");
    let file_name = file_path.to_str().unwrap();
    let mut js_file = BufWriter::new(File::create(file_name).unwrap());
    let mut code = "".to_string();
    let file = include_str!("utils.js");
    for line in file.lines() {
        code = format!("{}{}\n", code, line);
    }
    js_file.write_all(code.as_bytes())?;
    js_file.flush()?;
    Ok(())
}


pub fn generate_generate_witness_js_file(js_folder: &PathBuf) -> std::io::Result<()> {
    use std::io::BufWriter;
    let mut file_path  = js_folder.clone();
    file_path.push("generate_witness");
    file_path.set_extension("js");
    let file_name = file_path.to_str().unwrap();
    let mut js_file = BufWriter::new(File::create(file_name).unwrap());
    let mut code = "".to_string();
    let file = include_str!("common/generate_witness.js");
    for line in file.lines() {
        code = format!("{}{}\n", code, line);
    }
    js_file.write_all(code.as_bytes())?;
    js_file.flush()?;
    Ok(())
}

pub fn generate_witness_calculator_js_file(js_folder: &PathBuf) -> std::io::Result<()> {
    use std::io::BufWriter;
    let mut file_path  = js_folder.clone();
    file_path.push("witness_calculator");
    file_path.set_extension("js");
    let file_name = file_path.to_str().unwrap();
    let mut js_file = BufWriter::new(File::create(file_name).unwrap());
    let mut code = "".to_string();
    let file = include_str!("common/witness_calculator.js");
    for line in file.lines() {
        code = format!("{}{}\n", code, line);
    }
    js_file.write_all(code.as_bytes())?;
    js_file.flush()?;
    Ok(())
}
 */


/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, BufWriter, Write};
    use std::path::Path;
    const LOCATION: &'static str = "../target/code_generator_test";

    fn create_producer() -> CVMProducer {
        CVMProducer::default()
    }

    fn create_writer() -> BufWriter<File> {
        if !Path::new(LOCATION).is_dir() {
            std::fs::create_dir(LOCATION).unwrap();
        }
        let path = format!("{}/code.cvm", LOCATION);
        let file = File::create(path).unwrap();
        BufWriter::new(file)
    }

    /*
        let bytes = empty.read_line(&mut buffer)?;
        if bytes == 0 {
            eprintln!("EOF reached");
        }
    */
    fn write_block(writer: &mut BufWriter<File>, code: Vec<CVMInstruction>) {
        let data = merge_code(code);
        writer.write_all(data.as_bytes()).unwrap();
        writer.flush().unwrap();
    }

    #[test]
    fn produce_code() {
        let producer = create_producer();
        let mut writer = create_writer();
        // For every block of code that you want to write in code.wat the following two lines.
        // In the first line the code you want tow write is produced. Then, to write that code the
        // test function "write_block" is called.
        let mut code = vec![];
        code.push("(module".to_string());
        let mut code_aux = generate_imports_list();
        code.append(&mut code_aux);
        code_aux = generate_memory_def_list(&producer);
        code.append(&mut code_aux);

        code_aux = get_instructions_from_file("fr-types");
        code.append(&mut code_aux);

        code_aux = generate_types_list();
        code.append(&mut code_aux);
        code_aux = generate_exports_list();
        code.append(&mut code_aux);

        code_aux = get_instructions_from_file("fr-code");
        code.append(&mut code_aux);

        code_aux = desp_io_subcomponent_generator(&producer);
        code.append(&mut code_aux);

        code_aux = get_version_generator(&producer);
        code.append(&mut code_aux);

        code_aux = get_shared_rw_memory_start_generator(&producer);
        code.append(&mut code_aux);

        code_aux = read_shared_rw_memory_generator(&producer);
        code.append(&mut code_aux);

        code_aux = write_shared_rw_memory_generator(&producer);
        code.append(&mut code_aux);

        //code_aux = reserve_stack_fr_function_generator(&producer);
        code_aux = reserve_stack_fr_function_generator();
        code.append(&mut code_aux);

        code_aux = init_generator(&producer);
        code.append(&mut code_aux);

        code_aux = set_input_signal_generator(&producer);
        code.append(&mut code_aux);

        code_aux = get_input_signal_size_generator(&producer);
        code.append(&mut code_aux);

        code_aux = get_raw_prime_generator(&producer);
        code.append(&mut code_aux);

        code_aux = get_field_num_len32_generator(&producer);
        code.append(&mut code_aux);

        code_aux = get_witness_size_generator(&producer);
        code.append(&mut code_aux);

        code_aux = get_witness_generator(&producer);
        code.append(&mut code_aux);

        code_aux = copy_32_in_shared_rw_memory_generator(&producer);
        code.append(&mut code_aux);

        code_aux = copy_fr_in_shared_rw_memory_generator(&producer);
        code.append(&mut code_aux);

        code_aux = get_message_char_generator(&producer);
        code.append(&mut code_aux);

        code_aux = build_buffer_message_generator(&producer);
        code.append(&mut code_aux);

        code_aux = build_log_message_generator(&producer);
        code.append(&mut code_aux);
	
        //code_aux = main_sample_generator(&producer);
        //code.append(&mut code_aux);

        code_aux = get_instructions_from_file("fr-data");
        code.append(&mut code_aux);

        code_aux = generate_data_list(&producer);
        code.append(&mut code_aux);

        code.push(")".to_string());

        write_block(&mut writer, code);

        //let num = BigInt::parse_bytes(b"2240", 10).unwrap();
        // println!("Hexa: {}",wasm_hexa(4,&num));
        // println!("Bytes in1: {:?}",b"in1");
        // println!("Hash in1: {:X}",hasher("in1"));

        // After this test is executed the output can be find in:
        // circom_compiler/target/code_generator_test/code.wat
        assert!(true);
    }
}
*/

// FUNCTIONS FOR GENERATING CVM

pub fn declare_variable(vtype: Option<usize>, dimensions: &Vec<usize>) -> String{
    let stype = match vtype{
        Option::None => "ff".to_string(),
        Option::Some(node_id) => format!("bus_{}", node_id)
    };
    let mut s_dimensions = "".to_string();
    for d in dimensions{
        s_dimensions = format!("{}{} ", s_dimensions, d);
    }
    if dimensions.len() > 0{
        s_dimensions.pop();
    }
    format!("{} {} {}", stype, dimensions.len(), s_dimensions)
}

pub fn generate_prime(producer: &CVMProducer)-> Vec<CVMInstruction>{
    let mut instr = Vec::new();
    instr.push(";; Prime value".to_string());
    instr.push(format!("%%prime {}", producer.get_prime()));
    instr.push("\n".to_string());
    instr
}


pub fn generate_signals_memory(producer: &CVMProducer) -> Vec<CVMInstruction>{
    let mut instr = Vec::new();
    instr.push(";; Memory of signals".to_string());
    instr.push(format!("%%signals {}", producer.get_total_number_of_signals()));
    instr.push("\n".to_string());

    instr
}



pub fn generate_components_heap(producer: &CVMProducer)-> Vec<CVMInstruction>{
    let mut instr = Vec::new();
    instr.push(";; Heap of components".to_string());
    instr.push(format!("%%components_heap {}", producer.get_size_of_component_tree()));// ???
    instr.push("\n".to_string());

    instr
}


pub fn generate_types(producer: &CVMProducer) -> Vec<CVMInstruction>{
    let mut instr = Vec::new();
    instr.push(";; Types (for each field we store name type offset size nDims dims)".to_string());
    let mut node_id = 0;
    for bus in producer.get_busid_field_info(){
        instr.push(format!("%%type $bus_{}", node_id));
        for field in bus{
            // We store the following info: name type offset size number_dims dims
            let type_field = if field.bus_id.is_some(){
                format!("$bus_{}", field.bus_id.unwrap())
            } else{
                "ff".to_string()
            };
            let mut dims = "".to_string();
            for dim in &field.dimensions{
                dims = format!("{} {}", dims, dim);
            }

            instr.push(format!("       ${} ${} {} {} {} {}",
                field.name,
                type_field,
                field.offset,
                field.size,
                field.dimensions.len(),
                dims
            ));

        }
        node_id += 1;

    }
    instr.push("\n".to_string());

    instr
}



pub fn generate_main_template(producer: &CVMProducer) -> Vec<CVMInstruction>{
    let mut instr = Vec::new();
    instr.push(";; Main template".to_string());
    instr.push(format!("%%start {}", producer.get_main_header()));// ???
    instr.push("\n".to_string());

    instr
}


pub fn generate_components(producer: &CVMProducer) -> Vec<CVMInstruction>{
    let mut instr = Vec::new();
    instr.push(";; Component creation mode (implicit/explicit)".to_string());
    let creation_mode = match producer.get_implicit_component_creation(){
        true => "implicit",
        false => "explicit",
    };
    instr.push(format!("%%components {}", creation_mode));
    instr.push("\n".to_string());

    instr
}


pub fn generate_witness(producer: &CVMProducer) -> Vec<CVMInstruction>{
    let mut instr = Vec::new();
    instr.push(";; Witness (signal list)".to_string());
    let mut witness = "".to_string();
    for s in producer.get_witness_to_signal_list(){
        witness = format!("{} {}", witness, s)
    }
    instr.push(format!("%%witness{}", witness));
    instr.push("\n".to_string());

    instr
}
