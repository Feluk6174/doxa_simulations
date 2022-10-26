fn main(){
    let m = 10;
    const N:u32 = 6;

    let byte1:u32 = 0b_0_00_000_1111;
    let byte2:u32 = 0b_0_00_111_0000;
    let byte3:u32 = 0b_0_11_000_0000;
    let byte4:u32 = 0b_1_00_000_0000;

    let mut bytes:[[u32; 2]; N as usize-2] = [[0; 2]; N as usize-2];
    let mut shift = 0;
    for i in 0..N-2{
        let mut shift_val = 0;
        for j in 0..N-1-i-1{
            shift_val += 2_u32.pow(j as u32);
        }
        bytes[i as usize][0] = shift_val << shift;
        bytes[i as usize][1] = shift;
        shift += N-2-i
    }

    for i in 0..bytes.len(){
        println!("{} {:21b} {}", i, bytes[i][0], bytes[i][1])
    }
}