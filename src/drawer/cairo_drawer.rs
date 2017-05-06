use errors::*;

use cs;
use geodata::reader::OsmEntities;
use libc;
use mapcss::color::Color;
use mapcss::styler::{LineCap, LineJoin, Styler};
use std::slice;
use tile::{coords_to_float_xy, Tile, TILE_SIZE};

unsafe extern "C" fn write_func(closure: *mut libc::c_void, data: *mut u8, len: libc::c_uint) -> cs::enums::Status {
    let png_bytes: &mut Vec<u8> = &mut *(closure as *mut Vec<u8>);
    png_bytes.extend(slice::from_raw_parts(data, len as usize));
    cs::enums::Status::Success
}

pub fn draw_tile<'a>(entities: &OsmEntities<'a>, tile: &Tile, styler: &Styler) -> Result<Vec<u8>> {
    let mut data = Vec::new();

    unsafe {
        let s = cs::cairo_image_surface_create(cs::enums::Format::Rgb24, TILE_SIZE as i32, TILE_SIZE as i32);

        let cr = cs::cairo_create(s);

        let get_delta = |c| -((TILE_SIZE as f64) * (c as f64));
        cs::cairo_translate(cr, get_delta(tile.x), get_delta(tile.y));

        let to_double_color = |u8_color| (u8_color as f64) / 255.0_f64;
        let set_color = |c: &Color, a: f64| {
            cs::cairo_set_source_rgba(cr, to_double_color(c.r), to_double_color(c.g), to_double_color(c.b), a);
        };

        if let Some(ref color) = styler.canvas_fill_color {
            set_color(color, 1.0);
            cs::cairo_paint(cr);
        }

        let all_way_styles = styler.style_ways(entities.ways.iter(), tile.zoom);

        for &(w, ref style) in &all_way_styles {
            if w.node_count() == 0 {
                continue;
            }

            if style.color.is_none() && style.fill_color.is_none() {
                continue;
            }

            if let Some(ref line_join) = style.line_join {
                match *line_join {
                    LineJoin::Round => cs::cairo_set_line_join(cr, cs::enums::LineJoin::Round),
                    LineJoin::Miter => cs::cairo_set_line_join(cr, cs::enums::LineJoin::Miter),
                    LineJoin::Bevel => cs::cairo_set_line_join(cr, cs::enums::LineJoin::Bevel),
                }
            }

            if let Some(ref line_cap) = style.line_cap {
                match *line_cap {
                    LineCap::Butt => cs::cairo_set_line_cap(cr, cs::enums::LineCap::Butt),
                    LineCap::Round => cs::cairo_set_line_cap(cr, cs::enums::LineCap::Round),
                    LineCap::Square => cs::cairo_set_line_cap(cr, cs::enums::LineCap::Square),
                }
            }

             if let Some(ref dashes) = style.dashes {
                cs::cairo_set_dash(cr, dashes.as_ptr(), dashes.len() as i32, 0.0);
             }

            let draw_path = || {
                cs::cairo_new_path(cr);

                cs::cairo_set_line_width(cr, style.width.unwrap_or(1.0f64));

                let (x, y) = coords_to_float_xy(&w.get_node(0), tile.zoom);
                cs::cairo_move_to(cr, x, y);
                for i in 1..w.node_count() {
                    let (x, y) = coords_to_float_xy(&w.get_node(i), tile.zoom);
                    cs::cairo_line_to(cr, x, y);
                }
            };

            if let Some(ref c) = style.color {
                draw_path();
                set_color(c, style.opacity.unwrap_or(1.0f64));
                cs::cairo_stroke(cr);
            }

            if w.is_closed() {
                if let Some(ref c) = style.fill_color {
                    draw_path();
                    set_color(c, style.fill_opacity.unwrap_or(1.0f64));
                    cs::cairo_fill(cr);
                }
            }
        }

        cs::cairo_destroy(cr);

        cs::cairo_surface_write_to_png_stream(s, Some(write_func), &mut data as *mut Vec<u8> as *mut libc::c_void);
        cs::cairo_surface_destroy(s);
    }

    Ok(data)
}