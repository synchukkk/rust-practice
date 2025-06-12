const CANVAS_WIDTH: usize = 60;
const CANVAS_HEIGHT: usize = 30;

fn main() {
    let mut canvas = String::new();
    
    for row in 0..CANVAS_HEIGHT {
        for col in 0..CANVAS_WIDTH {
            let is_frame_edge = is_on_frame_border(row, col);
            let is_main_diagonal = is_on_primary_diagonal(row, col);
            let is_anti_diagonal = is_on_secondary_diagonal(row, col);
            
            let should_draw_star = is_frame_edge || is_main_diagonal || is_anti_diagonal;
            
            if should_draw_star {
                canvas.push('*');
            } else {
                canvas.push(' ');
            }
        }
        canvas.push('\n');
    }
    
    print!("{}", canvas);
}

fn is_on_frame_border(row: usize, col: usize) -> bool {
    row == 0 || row == CANVAS_HEIGHT - 1 || col == 0 || col == CANVAS_WIDTH - 1
}

fn is_on_primary_diagonal(row: usize, col: usize) -> bool {
    col == row * CANVAS_WIDTH / CANVAS_HEIGHT
}

fn is_on_secondary_diagonal(row: usize, col: usize) -> bool {
    col == CANVAS_WIDTH - 1 - row * CANVAS_WIDTH / CANVAS_HEIGHT
}
