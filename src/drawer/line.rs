use drawer::figure::Figure;
use drawer::point::Point;

pub fn draw_thick_line(p1: &Point, p2: &Point, width: f64, opacity: f64, figure: &mut Figure)
{
    let get_inc = |from, to| if from <= to { 1 } else { -1 };

    let (dx, dy) = ((p2.x - p1.x).abs(), (p2.y - p1.y).abs());
    let (mut x0, mut y0) = (p1.x, p1.y);
    let should_swap_x_y = dx > dy;

    let (mn, mx) = swap_x_y_if_needed(&mut x0, &mut y0, should_swap_x_y);
    let (mn_last, mx_last) = swap_x_y_if_needed(p2.x, p2.y, should_swap_x_y);
    let (mn_delta, mx_delta) = swap_x_y_if_needed(dx, dy, should_swap_x_y);
    let (mn_inc, mx_inc) = swap_x_y_if_needed(get_inc(p1.x, p2.x), get_inc(p1.y, p2.y), should_swap_x_y);

    let mut error = 0;
    let mut p_error = 0;

    let update_error = |error: &mut i32| {
        let was_corrected = if *error + 2 * mn_delta > mx_delta {
            *error -= 2 * mx_delta;
            true
        } else {
            false
        };
        *error += 2 * mn_delta;
        was_corrected
    };

    let line_dist_numer_const = ((p2.x * p1.y) - (p2.y * p1.x)) as f64;
    let line_dist_denom = ((dy*dy + dx*dx) as f64).sqrt();
    let half_width = width / 2.0;
    let feather_from = (half_width - 0.5).max(0.0);
    let feather_to = (half_width + 0.5).max(1.0);
    let feather_dist = feather_to - feather_from;
    let opacity_mul = opacity * width.min(1.0);

    let mut draw_perpendiculars = |mn, mx, p_error| {
        let mut draw_one_perpendicular = |mul| {
            let mut p_mn = mx;
            let mut p_mx = mn;
            let mut error = mul * p_error;
            loop {
                let (perp_x, perp_y) = swap_x_y_if_needed(p_mx, p_mn, should_swap_x_y);

                let line_dist_numer_non_const = ((p2.y - p1.y) * perp_x - (p2.x - p1.x) * perp_y) as f64;
                let line_dist = (line_dist_numer_const + line_dist_numer_non_const).abs() / line_dist_denom;

                let opacity = if line_dist < feather_from {
                    opacity_mul
                } else if line_dist < feather_to {
                    (feather_to - line_dist) / feather_dist * opacity_mul
                } else {
                    break;
                };

                let cur_point = Point { x: perp_x, y: perp_y };
                if !cur_point.is_in_logical_tile() {
                    break;
                }

                if cur_point.is_in_visible_tile() {
                    figure.add(perp_x as usize, perp_y as usize, opacity);
                }

                if update_error(&mut error) {
                    p_mn -= mul * mx_inc;
                }
                p_mx += mul * mn_inc;
            }
        };

        draw_one_perpendicular(1);
        draw_one_perpendicular(-1);
    };

    loop {
        draw_perpendiculars(*mn, *mx, p_error);

        if *mn == mn_last && *mx == mx_last {
            break;
        }

        if update_error(&mut error) {
            *mn += mn_inc;
            if update_error(&mut p_error) {
                draw_perpendiculars(*mn, *mx, p_error);
            }
        }
        *mx += mx_inc;
    }
}

fn swap_x_y_if_needed<T>(a: T, b: T, should_swap: bool) -> (T, T) {
    if should_swap {
        (b, a)
    } else {
        (a, b)
    }
}
