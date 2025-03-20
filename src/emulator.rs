use crate::types::*;

/**
 * Emulator struct.
 *
 * Holds all runtime data for the struct.
 * Contains the following members:
 * - `V`: CHIP-8's 16 8-bit registers, V0-VF. Note that VF doubles
 *   as a flag for some operations, so it should generally be avoided.
 * - `I`: The address register, used for memory operations.
 */
pub struct Emulator {
    V:      [u16; 16],
    I:      u16,
    Timers: [u8; 2],
    PC:     u16,
    SP:     u8,
    Stack:  [u16; 16],
    RAM:    [u8; 4096],
}

impl Emulator {
    // Create a new Emulator struct
    pub fn new() -> Self {
        Self {
            V: [0; 16],
            I: 0,
            Timers: [0; 2],
            PC: 0,
            SP: 0,
            Stack: [0; 16],
            RAM: [0; 4096],
        }
    }
    
    // TODO: Load a CHIP-8 program
    pub fn load_program(&mut self) -> () { () }
    
    // TODO: Run a loaded CHIP-8 program
    pub fn run(&mut self) -> () { () }

    // TODO: Instruction set
}
