use std::thread;

const N:u32 = 8;

fn get_n_connections(n:u32) -> u32{
    (n as f64).log2() as u32
}

fn test(perm:u32, bytes:[[u32; 2]; N as usize-2]) -> bool{
    let mut lines:[u32; N as usize] = [0; N as usize];

    for i in 1..lines.len(){
        lines[i-1] |= 2_u32.pow(i as u32);
        lines[i] |= 2_u32.pow(i as u32-1)
    }


    for i in 0..N-2{
        lines[N as usize-i as usize-1] |= perm & bytes[i as usize][0] >> bytes[i as usize][1];
    }

    for i in 0..N{
        println!("{:06b}", lines[i as usize]);
    }

    return true;
}

fn extra_stack_thread(){
    let n:u32 = N;
    let mut m:u32 = 0;
    for i in 0..n-1{
        m += i;
    }

    let n_connections:u32 = (((get_n_connections(n) as f32/2.0)+1.0) as u32)*(n-2)+1;

    println!("{} {} {} {}", n, m, get_n_connections(n), n_connections);

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

    let mut perms:[u32; 2_usize.pow(21)] = [0; 2_usize.pow(21)];

    for i in 0..perms.len(){
        perms[i] = i as u32;
    }

    for perm in perms{
        println!("{} {}", perm, test(perm, bytes));
    }

    test(0b_10101_01010, bytes);
    println!("{} {} {} {}", n, m, get_n_connections(n), n_connections);
}

fn main(){
    let child = thread::Builder::new()
        .stack_size(300000000)
        .spawn(extra_stack_thread)
        .unwrap();
    
    child.join().unwrap();
}