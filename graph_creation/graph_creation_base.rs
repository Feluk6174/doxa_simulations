fn get_n_connections(n:u32) -> u32{
    (n as f64).log2() as u32
}

fn main(){
    let n:u32 = 8;
    let mut m:u32 = 0;
    for _i in 0..n-1{
        m += 1;
    }

    let n_connections:u32 = (((get_n_connections(n) as f32/2.0)+1.0) as u32)*(n-2);

    println!("{} {} {} {}", n, m, get_n_connections(n), n_connections);

    let last_perm:[[u8; 1]; 2] = [[0], [1]];
    let mut new_perm:[[u8; 2]; 4] = [[2; 2]; 4];
    for i in 0..last_perm.len(){
        for j in 0..last_perm[i].len(){
            new_perm[i*2][j] = last_perm[i][j];
            new_perm[i*2+1][j] = last_perm[i][j];
        }
        new_perm[i*2][last_perm[i].len()] = 0;
        new_perm[i*2+1][last_perm[i].len()] = 1;
    }
    
    let last_perm:[[u8; 2]; 4] = new_perm;
    let mut new_perm:[[u8; 3]; 8] = [[2;3]; 8];
    for i in 0..last_perm.len(){
        for j in 0..last_perm[i].len(){
            new_perm[i*2][j] = last_perm[i][j];
            new_perm[i*2+1][j] = last_perm[i][j];
        }
        new_perm[i*2][last_perm[i].len()] = 0;
        new_perm[i*2+1][last_perm[i].len()] = 1;
    }

    println!("{:?} {:?}", last_perm, new_perm);
}