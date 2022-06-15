extern crate pedersen;
extern crate curve25519_dalek;

use pedersen::sequence::*;
use pedersen::commitments::*;

fn main() {
    // generate input table
    let mut table: Vec<Sequence> = Vec::new();

    /////////////////////////////////////////////
    // Define the data vectors that will be used in the computation. Each vector
    // will be translated into a single 32 bytes dalek CompressedRistretto data
    //
    // Note that you must specify the vector element type (u8, u16, u32, u64, u128)
    //
    // For instance:
    //     commitment[0] = g[0]*data1[0] + g[1]*data1[1] + g[2]*data1[2] + g[3]^data1[3]
    //                   = g[0]*1 + g[1]*2 + g[2]*3 + g[3]*4
    //
    //     commitment[2] = g[0]*data3[0] + g[1]*data3[1] + ... + g[6]*data3[6]
    //                   = g[0]*40 + g[1]*32 + g[2]*21 + g[3]*10 + ... + g[6]^444
    //
    /////////////////////////////////////////////
    let data1: Vec<u16> = vec![1, 2, 3, 4];
    let data2: Vec<u32> = vec![4, 3, 2, 1];
    let data3: Vec<u64> = vec![40, 32, 21, 10, 20, 35, 444];

    /////////////////////////////////////////////
    // Fill the table with entries
    // 
    // We need to wrapper the vector array inside the table object.
    // This object holds a slice of the data vector and the
    // total amount of bytes of each element stored in the vector
    /////////////////////////////////////////////
    table.push(Sequence::Dense(DenseSequence {
        data_slice: &data1.as_byte_slice(),
        element_size: std::mem::size_of_val(&data1[0])
    }));

    table.push(Sequence::Dense(DenseSequence {
        data_slice: &data2.as_byte_slice(),
        element_size: std::mem::size_of_val(&data2[0])
    }));

    table.push(Sequence::Dense(DenseSequence {
        data_slice: &data3.as_byte_slice(),
        element_size: std::mem::size_of_val(&data3[0])
    }));

    /////////////////////////////////////////////
    // We need to define a commitment vector which 
    // will store all the commitment results
    /////////////////////////////////////////////
    let mut commitments = vec![CompressedRistretto::from_slice(&[0 as u8; 32]); table.len()];
    
    /////////////////////////////////////////////
    // Do the actual commitment computation
    /////////////////////////////////////////////
    compute_commitments_with_sequences(& mut commitments, &table);

    for i in 0..commitments.len() {
        println!("commitment {}: {:?}\n", i, commitments[i]);
    }
}