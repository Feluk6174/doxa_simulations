import sys
try:
    n = int(sys.argv[1])
except IndexError:
    n = int(input("n = "))
m = sum([i for i in range(n-1)])

with open("graph_creation/graph_creation_base.rs", "r") as f:
    base_text = f.read()

text = f"fn write(new_perm:[[u8;{m}]; {2**(m)}])"
text += '{println!("{:?}", new_perm.last());}'

text += """
fn get_n_connections(n:u32) -> u32{
    (n as f64).log2() as u32
}

fn test(){\n"""

text += f"    let n:u32 = {n};"
text += """
    let mut m:u32 = 0;
    for i in 0..n-1{
        m += i;
    }

    let n_connections:u32 = (((get_n_connections(n) as f32/2.0)+1.0) as u32)*(n-2)+1;

    println!("{} {} {} {}", n, m, get_n_connections(n), n_connections);

"""

text += """
    let last_perm = vec!(vec!(0), vec!(1));
    let mut new_perm:[[u8; 2]; 4] = [[2; 2]; 4];
    for i in 0..last_perm.len(){
        for j in 0..last_perm[i].len(){
            new_perm[i*2][j] = last_perm[i][j];
            new_perm[i*2+1][j] = last_perm[i][j];
        }
        new_perm[i*2][last_perm[i].len()] = 0;
        new_perm[i*2+1][last_perm[i].len()] = 1;
    }
    
"""

for i in range(2, m+1):
    text += f"let last_perm = new_perm;\n    "
    text += f"let mut new_perm:[[u8;{i+1}]; {2**(i+1)}] = [[2;{i+1}]; {2**(i+1)}];\n"
    text += """    for i in 0..last_perm.len(){
        for j in 0..last_perm[i].len(){
            new_perm[i*2][j] = last_perm[i][j];
            new_perm[i*2+1][j] = last_perm[i][j];
        }
        new_perm[i*2][last_perm[i].len()] = 0;
        new_perm[i*2+1][last_perm[i].len()] = 1;
    }

    """

text += 'write(last_perm);\n}'

text += """
fn main(){
    std::thread::Builder::new().stack_size(333554432).spawn(test).unwrap().join().unwrap();
}
"""

with open("graph_greator.rs", "w") as f:
    f.write(text)


print(text)

print(n, m)