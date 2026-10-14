use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn distance(x1:f64,y1:f64,x2:f64,y2:f64)->f64 {(x2-x1).hypot(y2-y1)}
#[wasm_bindgen]
pub fn polygon_area(coords:&[f64])->f64 {let n=coords.len()/2;if n<3{return 0.0}let mut s=0.0;for i in 0..n{let j=(i+1)%n;s+=coords[2*i]*coords[2*j+1]-coords[2*j]*coords[2*i+1];}s.abs()/2.0}
#[wasm_bindgen]
pub fn polygon_perimeter(coords:&[f64])->f64{let n=coords.len()/2;if n<2{return 0.0}let mut s=0.0;for i in 0..n{let j=(i+1)%n;s+=distance(coords[2*i],coords[2*i+1],coords[2*j],coords[2*j+1]);}s}
#[cfg(test)]mod tests{use super::*;#[test]fn rectangle(){let a=[0.,0.,4.,0.,4.,3.,0.,3.];assert_eq!(polygon_area(&a),12.);assert_eq!(polygon_perimeter(&a),14.);}}