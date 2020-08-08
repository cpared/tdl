#![allow(warnings)]
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use std::convert::TryInto;
use wasm_bindgen::JsCast;
//con web_sys podemos interactuar con las clases que manejan el DOM
use web_sys::{CanvasRenderingContext2d};

const MAX_ITER: i64 = 10000;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    //#[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_i64(a: i64);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello mandelbrot!");
}

fn main(){
    println!("Hello World!");
}

#[wasm_bindgen]
pub fn draw(ctx:&CanvasRenderingContext2d,
    width: i64, height: i64, x1: i64, y1: i64, rss: f64, rse: f64, iss: f64, ise: f64){

    for x in 0..width{
        for y in 0..height{

            let xi = x1 - (x1 - x);
            let yi = y1 - (y1 - y);

            let mut mandelbrot_point = calculateSet(xi, yi, width, height, rss, rse, iss, ise);

            if(mandelbrot_point == MAX_ITER){
                mandelbrot_point = 0;
            }

            let str = format!("hsl(0,100%, {m}%)", m = mandelbrot_point);
            ctx.set_fill_style(&str.into());
            ctx.fill_rect(x as f64,y as f64, 1.0, 1.0);

        }
    }
}

fn calculateSet(x0: i64 , y0: i64, width: i64, height: i64, rss: f64, rse: f64, iss: f64, ise: f64) -> i64{

    let mut a = rss + ((x0 as f64 / width as f64) * (rse - rss));
    let mut b = iss + ((y0 as f64 / height as f64) * (ise - iss));

    let ca = a;
    let cb = b;

    let mut i = 0;

    for n in 0..MAX_ITER {
        let mut an = (a * a) - (b * b);
        let mut bn = 2.0 * a * b;

        a = an + ca;
        b = bn + cb;

        i = n;

        if ((a*a) + (b*b) > 4.0){
            break;
        }
    }

    return i;
}
