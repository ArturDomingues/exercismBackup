pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut i:u64 = n;
    let mut count = 0;
    while i > 1{
        if i%2==0{
            i=i/2;
        }else{
            i=3*i+1
        };
        count +=1;
    };
    Some(count) 
}
