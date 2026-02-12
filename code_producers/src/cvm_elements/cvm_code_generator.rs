use circom_algebra::num_bigint::BigInt;
use crate::cvm_elements::CVMProducer;
use std::collections::HashMap;
use crate::c_elements::FieldMap;
use crate::components::FieldData;
use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};

const SECTIONS: u8 = 10;
const MAGIC: &[u8] = b"cvm";
const PRIME: &[u8] = &[0, 0, 0, 0];
const VERSION: &[u8] = &[1, 0, 0, 0];
const MEM_SIGNALS: &[u8] = &[1, 0, 0, 0];
const HEAP_COMP: &[u8] = &[2, 0, 0, 0];
const TYPES: &[u8] = &[3, 0, 0, 0];
const MAIN: &[u8] = &[4, 0, 0, 0];
const COMPONENTS: &[u8] = &[5, 0, 0, 0];
const WITNESS: &[u8] = &[6, 0, 0, 0];
const INPUTS: &[u8] = &[7, 0, 0, 0];
pub const TEMPLATES: &[u8] = &[8, 0, 0, 0];
const FUNCTIONS: &[u8] = &[9, 0, 0, 0];
const PLACE_HOLDER: &[u8] = &[3, 3, 3, 3, 3, 3, 3, 3];

fn into_format(number: &[u8], with_bytes: usize) -> (Vec<u8>, usize) {
    let mut value = number.to_vec();
    while value.len() < with_bytes {
        value.push(0);
    }
    let size = value.len();
    (value, size)
}

pub fn bigint_as_bytes(number: &BigInt, with_bytes: usize) -> (Vec<u8>, usize) {
    let (_, value) = number.to_bytes_le();
    into_format(&value, with_bytes)
}

pub fn usize_as_bytes(number: usize, with_bytes: usize) -> Vec<u8>{
    bigint_as_bytes(&BigInt::from(number), with_bytes).0
}

pub fn initialize_section(writer: &mut BufWriter<File>, header: &[u8]) -> Result<u64, ()> {
    writer.write_all(header).map_err(|_err| {})?;
    //writer.flush().map_err(|_err| {})?;
    let go_back = writer.seek(SeekFrom::Current(0)).map_err(|_err| {})?;
    //writer.write_all(PLACE_HOLDER).map_err(|_| {})?;
    //writer.flush().map_err(|_err| {})?;
    Result::Ok(go_back)
}

pub fn end_section(writer: &mut BufWriter<File>, go_back: u64, size: usize) -> Result<(), ()> {
    let go_back_1 = writer.seek(SeekFrom::Current(0)).map_err(|_err| {})?;
    writer.seek(SeekFrom::Start(go_back)).map_err(|_err| {})?;
    let (stream, _) = bigint_as_bytes(&BigInt::from(size), 8);
    writer.write_all(&stream).map_err(|_err| {})?;
    writer.seek(SeekFrom::Start(go_back_1)).map_err(|_err| {})?;
    //writer.flush().map_err(|_| {})
    Result::Ok(())
}


pub fn write_prime_section(writer: &mut BufWriter<File>, producer: &CVMProducer)-> Result<(), ()>{
    
    let prime = producer.get_prime().parse::<BigInt>().unwrap();
    let size_prime = if prime.bits() % 64 == 0 {
        prime.bits() / 8
    } else{
        (prime.bits() / 64 + 1) * 8
    };

    let start = initialize_section(writer, PRIME)?;
    
    let (length_stream, bytes_size) = bigint_as_bytes(&BigInt::from(size_prime), 4);
    let (field_stream, bytes_field) = bigint_as_bytes(&prime, size_prime);

    writer.write_all(&length_stream).map_err(|_err| {})?;
    writer.write_all(&field_stream).map_err(|_err| {})?;
    
    let size = bytes_field + bytes_size;

    //end_section(writer, start, size)?;

    Ok(())
}

pub fn write_memory_signals_section(writer: &mut BufWriter<File>, producer: &CVMProducer)-> Result<(), ()>{
    
    let num_signals = producer.get_total_number_of_signals();
    let start = initialize_section(writer, MEM_SIGNALS)?;
    let (signals_stream, size) = bigint_as_bytes(&BigInt::from(num_signals), 8);
    writer.write_all(&signals_stream).map_err(|_err| {})?;
    //end_section(writer, start, size)?;

    Ok(())
}

pub fn write_components_heap_section(writer: &mut BufWriter<File>, producer: &CVMProducer)-> Result<(), ()>{
    
    let num_components = producer.get_size_of_component_tree();
    let start = initialize_section(writer, HEAP_COMP)?;
    let (components_stream, size) = bigint_as_bytes(&BigInt::from(num_components), 8);
    writer.write_all(&components_stream).map_err(|_err| {})?;
    //end_section(writer, start, size)?;

    Ok(())
}

pub fn generate_variable_declaration(vtype: Option<usize>, dimensions: &Vec<usize>) -> Vec<Vec<u8>>{
    let mut result = Vec::new();

    let type_field = if vtype.is_some(){
        vtype.unwrap() + 1
    } else{
        0
    };
    let (type_stream, size_type) = bigint_as_bytes(&BigInt::from(type_field), 4);
    result.push(type_stream);
    let (dim_stream, size_dim) = bigint_as_bytes(&BigInt::from(dimensions.len()), 4);
    result.push(dim_stream);

    for dim in dimensions{
        let (dims_stream, size_dims) = bigint_as_bytes(&BigInt::from(*dim), 8);
        result.push(dims_stream);
    }
    result
}

pub fn write_type_section(writer: &mut BufWriter<File>, bus_info: &Vec<FieldData>)-> Result<(), ()>{
    
    let start = initialize_section(writer, TYPES)?;
    let (fields_stream, size_fields) = bigint_as_bytes(&BigInt::from(bus_info.len()), 4);
    writer.write_all(&fields_stream).map_err(|_err| {})?;
    for data in bus_info{
        let result = generate_variable_declaration(data.bus_id, &data.dimensions);
        let merged_result = merge_code(result);
        writer.write_all(&merged_result).map_err(|_err| {})?;
    }
    //end_section(writer, start, size)?;

    Ok(())
}


pub fn write_all_types(writer: &mut BufWriter<File>, producer: &CVMProducer)-> Result<(), ()>{
    
    let buses = producer.get_busid_field_info();
    for bus in buses{
        write_type_section(writer, bus)?;
    }
    Ok(())
}

pub fn write_main_template(writer: &mut BufWriter<File>, producer: &CVMProducer)-> Result<(), ()> {
    
    let id_main = producer.get_main_id();
    let start = initialize_section(writer, MAIN)?;
    let (signals_stream, size) = bigint_as_bytes(&BigInt::from(id_main), 4);
    writer.write_all(&signals_stream).map_err(|_err| {})?;
    //end_section(writer, start, size)?;

    Ok(())
}

pub fn write_witness(writer: &mut BufWriter<File>, producer: &CVMProducer)-> Result<(), ()> {
    
    let witness = producer.get_witness_to_signal_list();
    let start: u64 = initialize_section(writer, WITNESS)?;
    let (signals_stream, size) = bigint_as_bytes(&BigInt::from(witness.len()), 8);
    writer.write_all(&signals_stream).map_err(|_err| {})?;

    for signal in witness{
        let (signal_stream, size) = bigint_as_bytes(&BigInt::from(*signal), 8);
        writer.write_all(&signal_stream).map_err(|_err| {})?;
    }
    //end_section(writer, start, size)?;

    Ok(())
}

pub fn write_inputs(writer: &mut BufWriter<File>, producer: &CVMProducer)-> Result<(), ()> {
    let inputs = producer.get_main_input_list();
    let num_filtered_inputs = inputs
        .into_iter()                     
        .filter(|input| !input.name.contains("."))
        .count();
    let start = initialize_section(writer, INPUTS)?;
    let (inputs_stream, size) = bigint_as_bytes(&BigInt::from(num_filtered_inputs), 4);
    writer.write_all(&inputs_stream).map_err(|_err| {})?;

    for input in inputs{
        if !input.name.contains("."){
            //write the signal name followed by 0
            writer.write_all(&input.name.as_bytes()).map_err(|_err| {})?;
            let zero_byte: &[u8] = &[0];
            writer.write_all(zero_byte).map_err(|_err: std::io::Error| {})?;

            let result = generate_variable_declaration(input.bus_id, &input.dimensions);
            let merged_result = merge_code(result);
            writer.write_all(&merged_result).map_err(|_err| {})?;
        }

    }
    Ok(())
}

pub fn initialize_file(writer: &mut BufWriter<File>) -> Result<(), ()> {
    writer.write_all(MAGIC).map_err(|_err| {})?;
    //writer.flush().map_err(|_err| {})?;
    writer.write_all(VERSION).map_err(|_err| {})?;
    //writer.flush().map_err(|_err| {})?;
    //writer.write_all(&[num_sections, 0, 0, 0]).map_err(|_err| {})?;
    //writer.flush().map_err(|_err| {})?;
    Result::Ok(())
}


pub fn merge_code(instructions: Vec<Vec<u8>>) -> Vec<u8> {
    let mut code = Vec::new();
    for insts in instructions{
        for i in insts{
            code.push(i);
        }
    }
    code
}