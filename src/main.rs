mod utils;

fn main() {
    println!("Hello, world!");
}
/*
 - Init data base with K
 - S = Hash512(H)
 - gen1 = AesGenR1(S)
 - Fill RANDOMX_SCRATCHPAD_L3 random bytes using generator gen1
 - gen4 = AesGenR4(gen1.state) (final state)
 - frpc register = 0
 Cycle: {
 - Programm VM using 128 + 8 * RANDOMX_PROGRAM_SIZE random bytes using gen4
 - Execute VM
 - S = Hash512(RegisterFile)
 - Set gen4.state = S
 }
 - Perform Cycle a total of RANDOMX_PROGRAM_COUNT times, last iteration skips Hashing the register and state change
 - Scratchpad fingerprint -> A = AesHash1R(ScratchPad)
 - bytes [192 - 255] of RegisterFile = A
 - Result = Hash256(RegisterFile)

 The input of the Hash512 function in [- S = Hash512(RegisterFile)] is the following 256 bytes:
 +---------------------------------+
 |         registers r0-r7         | (64 bytes)
 +---------------------------------+
 |         registers f0-f3         | (64 bytes)
 +---------------------------------+
 |         registers e0-e3         | (64 bytes)
 +---------------------------------+
 |         registers a0-a3         | (64 bytes)
 +---------------------------------+

 The input of the Hash256 function in last step is the following 256 bytes:

 +---------------------------------+
 |         registers r0-r7         | (64 bytes)
 +---------------------------------+
 |         registers f0-f3         | (64 bytes)
 +---------------------------------+
 |         registers e0-e3         | (64 bytes)
 +---------------------------------+
 |      AesHash1R(Scratchpad)      | (64 bytes)
 +---------------------------------+

 */