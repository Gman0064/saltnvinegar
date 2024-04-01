// chip8.rs
// Implementation of chip8 machine, registers, and memory makeup 

use crate::util;

// Memory map constants
pub const PRGRM_START: usize = 512;
pub const MAX_MEM_SIZE: usize = 4096;
pub const MAX_PRG_SIZE: usize = 3584;

// Instruction constants
const INSTR_NOOP: u16 = 0x0000;
const INSTR_EXIT: u16 = 0x00FD;

#[derive(Debug)]
struct EmulationState {
    // Miscellanious registers
    program_counter: u16,
    stack_ptr: u8,
    
    // General purpose registers
    registers: [u8; 16],

    // Memory Map:
    // +---------------+= 0xFFF (4095) End of Chip-8 RAM
    // |               |
    // |               |
    // |               |
    // |               |
    // |               |
    // | 0x200 to 0xFFF|
    // |     Chip-8    |
    // | Program / Data|
    // |     Space     |
    // |               |
    // |               |
    // |               |
    // +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
    // |               |
    // |               |
    // |               |
    // +---------------+= 0x200 (512) Start of most Chip-8 programs
    // | 0x000 to 0x1FF|
    // | Reserved for  |
    // |  interpreter  |
    // +---------------+= 0x000 (0) Start of Chip-8 RAM
    memory: [u8; 4096]
}

pub fn start_emulation(data: [u8; MAX_PRG_SIZE]) {
    let mut state: EmulationState = EmulationState{
        program_counter: 0,
        stack_ptr: 0,
        registers: [0; 16],
        memory: [0; MAX_MEM_SIZE]
    };

    load_program(data, &mut state);
    exec_program(&mut state);
}

// Load the program into virtual memory
fn load_program(data: [u8; MAX_PRG_SIZE], state: &mut EmulationState) {
    println!("Copying program data to memory");
    for i in 0..MAX_PRG_SIZE {
        state.memory[i+PRGRM_START] = data[i];
    }
}

// Execute the program starting at the userspace address
fn exec_program(state: &mut EmulationState) {
    println!("Starting emulation");

    let mut head_byte: u8;
    let mut tail_byte: u8;
    let mut instruction_tuple: (u8, u8);

    for _i in PRGRM_START+usize::from(state.program_counter)..MAX_MEM_SIZE {
        // Grab the first byte of instruction
        head_byte = state.memory[PRGRM_START + usize::from(state.program_counter)];
        state.program_counter += 1;
        
        // Grab the second byte of instruction
        tail_byte = state.memory[PRGRM_START + usize::from(state.program_counter)];
        state.program_counter += 1;
        
        instruction_tuple = (head_byte, tail_byte);

        println!("0x{:02X?}{:02X?}", head_byte, tail_byte);

        if util::u8_tuple_to_u16(instruction_tuple) == INSTR_EXIT {
            println!("Read exit instruction, stopping emulation");
            return;
        }

        if util::u8_tuple_to_u16(instruction_tuple) == INSTR_NOOP {
            println!("Read no-op instruction, stopping emulation");
            return;
        }
    }

    
}