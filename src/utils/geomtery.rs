use std::f32::consts::PI;

pub fn triangle_fan(n: u32) -> (Vec<f32>, Vec<u32>) {
    let mut verts = vec![0.0, 0.0, 0.5, 0.0];
    let mut idx = vec![];

    let mut angle: f32;
    for m in 1..n {
        angle = 2.0 * PI * (m as f32) / (n as f32);

        verts.push(angle.cos() * 0.5);
        verts.push(angle.sin() * 0.5);

        idx.push(0);
        idx.push(m);
        idx.push(m + 1);
    }
    idx.push(0);
    idx.push(n);
    idx.push(1);

    (verts, idx)
}
