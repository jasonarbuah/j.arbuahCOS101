fn main() {
   let q1:f64 = 2.0;
   let q2:f64 = 1.0;
   let q3:f64 = 3.0;
   let q4:f64 = 3.0;
   let q5:f64 = 1.0;
   let p1:f64 = 450000.0;
   let p2:f64 = 1500000.0;
   let p3:f64 = 750000.0;
   let p4:f64 = 2850000.0;
   let p5:f64 = 250000.0;

   // sum
   let s = (q1 * p1) + (q2 * p2) + (q3 * p3) + (q4 * p4) + (q5 * p5);
   println!("sum is {}", s);
   let tq = q1 + q2 + q3 + q4 +q5;
   println!("total quantity is {}", tq);
   let a = s/tq;
   println!("average is {}", a);
   }
