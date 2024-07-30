use std::f64;
fn magnitude(a:[f64;3]) -> f64 {
    let mag:f64=(a[0]*a[0])+(a[1]*a[1])+(a[2]*a[2]).sqrt();
    return mag;
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(a:[f64;3],_mag:f64) ->[f64;3] {
    let nmag : [f64;3]=[(a[0]/magnitude(a)),(a[1]/magnitude(a)),(a[2]/magnitude(a))];
    return nmag;
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude([0.0, 1.0, 0.0]));
    let v = [1.0, 2.0, 9.0];
    let magv:f64=magnitude(v);
    println!("Magnitude of v: {}",magv);
    let nv=normalize(v, magv);
    // while i<3{
    //     print!(" {}",nv[i]);
    //     i+=1;
    // }
    for i in nv{
        print!(" {}",i);
    }
    println!();
    let nvmag=magnitude(nv);
    println!("Magnitude of v after normalisation: {}",nvmag);

}
