pub fn calculate_letterbox_viewport(
    window_size: (u32, u32),
    logical_size: (u32, u32),
) -> (i32, i32, i32, i32) {
    let (win_w, win_h) = (window_size.0 as f32, window_size.1 as f32);
    let (log_w, log_h) = (logical_size.0 as f32, logical_size.1 as f32);

    let window_aspect = win_w / win_h;
    let logical_aspect = log_w / log_h;

    if window_aspect >= logical_aspect {
        // window is wider → letterbox horizontally (pillarbox)
        let scaled_width = (win_h * logical_aspect).round() as i32;
        let x_offset = ((win_w as i32) - scaled_width) / 2;
        (x_offset, 0, scaled_width, win_h as i32)
    } else {
        // window is taller → letterbox vertically
        let scaled_height = (win_w / logical_aspect).round() as i32;
        let y_offset = ((win_h as i32) - scaled_height) / 2;
        (0, y_offset, win_w as i32, scaled_height)
    }
}
