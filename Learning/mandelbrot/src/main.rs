use num::Complex;

///試著使用有限的迭代來確定"C"是否屬於Mandelbrot集合
///若"C"不是成員，則回傳"Some(i)"，其中"i"是讓"C"離開以圓點為中心、半徑為2的圓所需的迭代次數。若"C"似乎是成員(更精確地說，若迭代次數到達上限，卻無法證明"C"不是成員)，則回傳"None"。
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            returm Some(i);
        }
        z = z * z + c;
    }
    None
}
