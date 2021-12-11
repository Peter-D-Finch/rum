# rum
```text
Peter Finch
Professor Noah Daniel’s
CSC 411: RUM Design Document
3 December 2021
1) Program Architecture Explanation:
1.1 Machine State
The Universal Machine consists of the following parts:
1. 8 general purpose registers
2. Address space of memory segments
3. I/O device that takes ASCII as input and output
4. 32-Bit program counter
5. Segment (NOTE: Architecture described in section 3 of this document)
As a general abstraction, all of these parts are wrapped into a struct of the following form:
struct mch_state {
regs: [u32; 8],
prog_cntr: u32,
addr_space: HashMap<u32, segment>
}
I chose to use the following data abstractions of these parts, respectively:
1. For registers, just a fixed size array of u32 type and length 8
2. For address space, a hash map using a u32 as key and segment struct as value
3. For I/O device, a trait will be used to implement the input() and output() methods
4. For program counter, just a u32 value
5. For segment, a struct that stores the data as an array of u64 values and the length of
32-bit words.
1.2 Basic Behavior
A trait named um_functions is used to give the Universal Machine it’s most basic behaviors: the
execution cycle and the initial state. The trait is of the following form:
trait um_functions {
+ fn exec_cycle(),
+ fn new(init_seg: Vec<u32>) -> Self
}
The function exec_cycle() is triggered at every time step and retrieves an instruction word from
the ‘0’ segment at the index of the program counter, advances the program counter if there is
another instruction, and then executes the retrieved instruction. This function also handles the
decoding of machine instructions from a 32-bit word and calls the appropriate function in the
um_operations module with the appropriate registers.
The
1.3 I/O Device
The I/O Device is implemented as a trait that mch_state implements and contains the input()
and output methods. The trait has the following form:
trait IO_Device {
+ fn input(reg: &u32),
+ fn output(reg: &u32)
}
1.4 Operations Implementation
I chose to implement the Universal Machine operations as a module of functions each taking a
tuple of form (&u32, &u32, &u32) as an argument where each &u32 is a memory address to a
register in the Universal Machine. The module has the following form:
module um_operations {
+ fn cond_move(regs: (&u32, &u32, &u32))
+ fn seg_load(regs: (&u32, &u32, &u32))
+ fn add(regs: (&u32, &u32, &u32))
+ fn multiply(regs: (&u32, &u32, &u32))
+ fn bitwise_nand(regs: (&u32, &u32, &u32))
+ fn divide(regs: (&u32, &u32, &u32))
+ fn halt()
+ fn map_seg(regs: (&u32, &u32, &u32))
+ fn unmap_seg(regs: (&u32, &u32, &u32))
+ fn load_prog(regs: (&u32, &u32, &u32))
+ fn load_val(reg: &u32, val: u8)
}
1.5 Usage of Bitpack
No changes were made to the bitpack module since it’s last submission in the previous Arith
assignment. Bitpack is used here to handle the instruction coding within 32-bit words and
packing of two 32-bit words into one 64-bit word.
1.6 Main Method Coupling with UM
The only purpose of the main function is to process the command line argument and read in the
sequence of 32-bit words from the file and then call the constructor method new(init_seg:
Vec<u32>) -> Self
2) Testing Architecture:
2.1 Overview
Both traits within the architecture will have a testing module and within every module each of the
methods within the trait will be tested. In addition, the um_operations module will have a unit
test validating each of the functions within the module. A composite test of the program will be
done using a self written test program as well as the example test programs provided with the
assignment, as well as a test to make sure that resource exhaustion causes a checked runtime
error.
2.2 Trait Testing
The IO_Device trait will test the functionality of the input and output functions. It will ensure that
the machine correctly inputs and outputs as well as handling errors from a value being passed
outside the range 0-255. The IO_Device test module will have the form:
#[cfg(test)]
mod IO_Device_tests {
#[test]
fn input(reg: &u32);
#[test]
fn output(reg: &u32);
}
The um_functions test module will test the proper encoding and decoding and ensure that
exec_cycle calls the correct functions using the correct registers. The test module will be of the
form:
#[cfg(test)]
mod um_functions_tests {
#[test]
fn exec_cycle_test();
#[test]
fn new(init_seg: Vec<u32>);
}
2.3 Operations Testing
The bulk of the testing will be on the operations module and testing each of the functions. Since
the um_functions module contains fairly basic operations it’s easy to test the input and output of
each function to ensure that it correctly performs the calculations and utilizes it’s registers
correctly.
3) Segment Representation and Invariants
3.1 Representation Specifications
As stated in the assignment specifications document a segment must be:
1. An ordered collection (sequence)
2. The objects within the collection must be 32-bit words
3.2 Segment Representation
Since the collection is ordered and the memory segments are meant to be traversed through
(although with some jumping) I thought a good implementation would be packing the
instructions in pairs of two’s inside an unsigned 64 bit value. If the number of 32-bit words within
the segment happens to be odd, an extra halt instruction will be appended, since the program
will halt when there are no remaining instructions, anyway. When an instruction is called inside
the exec_cycle() function, both instructions will be unpacked and loaded at once and executed
sequentially before the next two instructions are loaded.
3.3 Segment Invariants
● Any memory segment of 32-bit words of length n will be represented as an array of
unsigned 64-bit values. If the number of 32-bit words is even, the length of u64 values
will be 𝑛/2, else the length will be 𝑛/2 + 1.
● The most significant 32 bytes of any u64 value at index n (within the sequence of u64
values) will represent the 32-bit word at index 2n (within the memory segment) and the
least significant 32 bytes of data represent the 32-bit word at index 2n + 1.
● If the number of 32-bit words in a segment is odd, the last word in the instruction will
always be an instruction word for the halt operation
```