use super::types::*;
use crate::hir::very_concrete_program::Param;
use crate::intermediate_representation::InstructionList;
use crate::translating_traits::*;
use code_producers::c_elements::*;
use code_producers::wasm_elements::*;
use code_producers::cvm_elements::*;

//use std::io::Write;

pub type FunctionCode = Box<FunctionCodeInfo>;
#[derive(Default)]
pub struct FunctionCodeInfo {
    pub header: String,
    pub name: String,
    pub params: Vec<Param>,
    pub returns: Vec<Dimension>,
    pub body: InstructionList,
    pub max_number_of_vars: usize,
    pub max_number_of_ops_in_expression: usize,
    pub is_array_result: bool,
}

impl ToString for FunctionCodeInfo {
    fn to_string(&self) -> String {
        let mut body = "".to_string();
        for i in &self.body {
            body = format!("{}{}\n", body, i.to_string());
        }
        format!("FUNCTION({})(\n{})", self.header, body)
    }
}

impl WriteWasm for FunctionCodeInfo {
    fn produce_wasm(&self, producer: &WASMProducer) -> Vec<String> {
        use code_producers::wasm_elements::wasm_code_generator::*;
        //to be revised
        let mut instructions = vec![];
        let funcdef = format!("(func ${} (type $_t_i32i32ri32)", self.header);
        instructions.push(funcdef);
        instructions.push(format!("(param {} i32)", producer.get_result_address_tag()));
        instructions.push(format!("(param {} i32)", producer.get_result_size_tag()));
	instructions.push("(result i32)".to_string()); //state 0 = OK; > 0 error
        instructions.push(format!("(local {} i32)", producer.get_cstack_tag()));
        instructions.push(format!("(local {} i32)", producer.get_lvar_tag()));
        instructions.push(format!("(local {} i32)", producer.get_expaux_tag()));
        instructions.push(format!("(local {} i32)", producer.get_temp_tag()));
        instructions.push(format!("(local {} i32)", producer.get_aux_0_tag()));
        instructions.push(format!("(local {} i32)", producer.get_aux_1_tag()));
        instructions.push(format!("(local {} i32)", producer.get_aux_2_tag()));
        instructions.push(format!("(local {} i32)", producer.get_counter_tag()));
        instructions.push(format!("(local {} i32)", producer.get_store_aux_1_tag()));
        instructions.push(format!("(local {} i32)", producer.get_store_aux_2_tag()));
        instructions.push(format!("(local {} i32)", producer.get_copy_counter_tag()));
        instructions.push(format!("(local {} i32)", producer.get_call_lvar_tag()));
        instructions.push(format!(" (local {} i32)", producer.get_merror_tag()));
        let local_info_size_u32 = producer.get_local_info_size_u32();
        //set lvar (start of auxiliar memory for vars)
        instructions.push(set_constant("0"));
        instructions.push(load32(None)); // current stack size
        let var_start = local_info_size_u32 * 4; // starts after local info
        if local_info_size_u32 != 0 {
            instructions.push(set_constant(&var_start.to_string()));
            instructions.push(add32());
        }
        instructions.push(set_local(producer.get_lvar_tag()));
        //set expaux (start of auxiliar memory for expressions)
        instructions.push(get_local(producer.get_lvar_tag()));
        let var_stack_size = self.max_number_of_vars * 4 * (producer.get_size_32_bits_in_memory()); // starts after vars
        instructions.push(set_constant(&var_stack_size.to_string()));
        instructions.push(add32());
        instructions.push(set_local(producer.get_expaux_tag()));
        //reserve stack and sets cstack (starts of local var memory)
        let needed_stack_bytes = var_start
            + var_stack_size
            + self.max_number_of_ops_in_expression * 4 * (producer.get_size_32_bits_in_memory());
        let mut reserve_stack_fr_code = reserve_stack_fr(producer, needed_stack_bytes);
        instructions.append(&mut reserve_stack_fr_code); //gives value to $cstack
        if producer.needs_comments() {
            instructions.push(";; start of the function code".to_string());
	}
        //generate code

        for t in &self.body {
            let mut instructions_body = t.produce_wasm(producer);
            instructions.append(&mut instructions_body);
        }
        instructions.push(set_constant("0"));	
        instructions.push(")".to_string());
        instructions
    }
}

impl WriteC for FunctionCodeInfo {
    fn produce_c(&self, producer: &CProducer, _parallel: Option<bool>) -> (Vec<String>, String) {
        use c_code_generator::*;
        
        let mut instructions = Vec::new();
        
        
        let header = format!("void {}", self.header);
        let params = vec![
            declare_circom_calc_wit(),
            if producer.prime_str != "goldilocks" {
                declare_lvar_pointer()
            } else {
                declare_64bit_lvar_array()
            },
            declare_component_father(),
            if producer.prime_str != "goldilocks" {
                declare_dest_pointer()
            } else {
                declare_64bit_dest_reference()
            },
            declare_dest_size(),
        ];
        let mut body = vec![];
        if producer.prime_str != "goldilocks" {
            body.push(format!("{};", declare_circuit_constants()));
            body.push(format!("{};", declare_expaux(self.max_number_of_ops_in_expression)));
        } else {
            body.push(format!("{};", declare_64bit_expaux(self.max_number_of_ops_in_expression)));
        }            
        body.push(format!("{};", declare_my_template_name_function(&self.name)));
        body.push(format!("u64 {} = {};", my_id(), component_father()));
        for t in &self.body {
            let (mut instructions_body, _) = t.produce_c(producer, Some(false));
            body.append(&mut instructions_body);
        }
        let callable = build_callable(header, params, body);
        instructions.push(callable);
  
        (instructions, "".to_string())
    }
}

impl WriteCVM for FunctionCodeInfo {
    fn produce_cvm(&self, producer: &mut CVMProducer) -> (Vec<String>, String) {
        use code_producers::cvm_elements::cvm_code_generator::*;
        // create function code
        let mut instructions = vec![];


        let mut inputs = "".to_string();
        //let mut inputs = format!("{} {}", "i64".to_string(), FUNCTION_DESTINATION_SIZE); 

        if self.is_array_result {
             inputs = format!("{} i64 0 i64 0", inputs);
        }
        
        for param in &self.params{
            inputs = format!("{} {}", inputs, declare_variable(None, &param.length));
        }

//        instructions.push(format!("%%result  {} {}", self.is_array_result, self.returns.len()));
        let outputs = if !self.is_array_result || self.returns.len() == 0 { "ff".to_string() } else { "".to_string() };

        instructions.push(format!("%%function {} [{}] [{}]",
            self.header, 
            outputs,
            inputs,
        ));
        let size = self.max_number_of_vars;
        //instructions.push(format!("local.memory {}",size));
        //instructions.push(format!("{} = {}", RETURN_POSITION, size));
        let return_call_position = producer.fresh_var();
        producer.set_current_var_to_return_from_call(return_call_position.clone());
        if self.is_array_result {            
            instructions.push(format!("{} = i64.{}", return_call_position, size+2));
            let return_position = producer.fresh_var();
            producer.set_current_function_return_position_var(return_position.clone());
            instructions.push(format!("{} = i64.load i64.0", return_position));
            let return_size = producer.fresh_var();
            producer.set_current_function_return_size_var(return_size.clone());
            instructions.push(format!("{} = i64.load i64.1", return_size));
        } else {
            instructions.push(format!("{} = i64.{}", return_call_position, size));
        }
        for t in &self.body {
            let (mut instructions_body,_) = t.produce_cvm(producer);
            instructions.append(&mut instructions_body);
        }
        instructions.push("".to_string());
        (instructions, "".to_string())
    }
}

impl FunctionCodeInfo {
    pub fn wrap(self) -> FunctionCode {
        FunctionCode::new(self)
    }
    pub fn is_linked(&self, name: &str, params: &Vec<Param>) -> bool {
        self.name.eq(name) && self.params.eq(params)
    }
}
