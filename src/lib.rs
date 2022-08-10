#![recursion_limit="256"]
#![feature(alloc)]
#![feature(drain_filter)]
#![feature(get_mut_unchecked)]
#![allow(warnings)]
extern crate wasm_bindgen;
extern crate hashbrown;
#[macro_use]
extern crate alloc;
extern crate core;
extern crate web_sys;
extern crate mech_core;
extern crate mech_utilities;
extern crate bincode;
#[macro_use]
extern crate lazy_static;
extern crate miniz_oxide;
extern crate base64;
//use mech_core::{Interner, Transaction};
//use mech_core::Value;

use base64::{encode, decode};
use miniz_oxide::inflate::decompress_to_vec;
use miniz_oxide::deflate::compress_to_vec;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use hashbrown::hash_set::HashSet;
use alloc::vec::Vec;
use core::fmt;
//use mech_syntax::formatter::Formatter;
use mech_core::*;
use mech_utilities::{SocketMessage, MiniBlock};
use web_sys::{ErrorEvent, MessageEvent, WebSocket, FileReader};
use std::sync::Arc;

mod shapes;
mod elements;

lazy_static! {
  static ref HTML_APP: u64 = hash_str("html/app");
  static ref DIV: u64 = hash_str("div");
  static ref A: u64 = hash_str("a");
  static ref IMG: u64 = hash_str("img");
  static ref SRC: u64 = hash_str("src");
  static ref CONTAINS: u64 = hash_str("contains");
  static ref ROOT: u64 = hash_str("root");
  static ref TYPE: u64 = hash_str("type");
  static ref KIND: u64 = hash_str("kind");
  static ref HREF: u64 = hash_str("href");
  static ref BUTTON: u64 = hash_str("button");
  static ref SLIDER: u64 = hash_str("slider");
  static ref MIN: u64 = hash_str("min");
  static ref MAX: u64 = hash_str("max");
  static ref VALUE: u64 = hash_str("value");
  static ref CANVAS: u64 = hash_str("canvas");
  static ref PARAMETERS: u64 = hash_str("parameters");
  static ref HEIGHT: u64 = hash_str("height");
  static ref WIDTH: u64 = hash_str("width");
  static ref SHAPE: u64 = hash_str("shape");
  static ref CIRCLE: u64 = hash_str("circle");
  static ref RECTANGLE: u64 = hash_str("rectangle");
  static ref LINE: u64 = hash_str("line");
  static ref PATH: u64 = hash_str("path");
  static ref START__POINT: u64 = hash_str("start-point");
  static ref LINE__WIDTH: u64 = hash_str("line-width");
  static ref START__ANGLE: u64 = hash_str("start-angle");
  static ref END__ANGLE: u64 = hash_str("end-angle");
  static ref QUADRATIC: u64 = hash_str("quadratic");
  static ref CONTROL__POINT: u64 = hash_str("control-point");
  static ref CONTROL__POINTS: u64 = hash_str("control-points");
  static ref END__POINT: u64 = hash_str("end-point");
  static ref X1: u64 = hash_str("x1");
  static ref X2: u64 = hash_str("x2");
  static ref Y1: u64 = hash_str("y1");
  static ref Y2: u64 = hash_str("y2");
  static ref RADIUS: u64 = hash_str("radius");
  static ref STROKE: u64 = hash_str("stroke");
  static ref FILL: u64 = hash_str("fill");
  static ref CENTER__X: u64 = hash_str("center-x");
  static ref CENTER__Y: u64 = hash_str("center-y");
  static ref IMAGE: u64 = hash_str("image");
  static ref X: u64 = hash_str("x");
  static ref Y: u64 = hash_str("y");
  static ref ROTATE: u64 = hash_str("rotate");
  static ref TRANSLATE: u64 = hash_str("translate");
  static ref SOURCE: u64 = hash_str("source");
  static ref TIME_TIMER: u64 = hash_str("time/timer");
  static ref PERIOD: u64 = hash_str("period");
  static ref TICKS: u64 = hash_str("ticks");
  static ref HTML_EVENT_POINTER__MOVE: u64 = hash_str("html/event/pointer-move");
  static ref HTML_EVENT_POINTER__DOWN: u64 = hash_str("html/event/pointer-down");
  static ref HTML_EVENT_POINTER__UP: u64 = hash_str("html/event/pointer-up");
  static ref HTML_EVENT_KEY__DOWN: u64 = hash_str("html/event/key-down");
  static ref HTML_EVENT_KEY__UP: u64 = hash_str("html/event/key-up");
  static ref TARGET: u64 = hash_str("target");
  static ref KEY: u64 = hash_str("key");
  static ref EVENT__ID: u64 = hash_str("event-id");
  static ref ARC: u64 = hash_str("arc");
  static ref ELLIPSE: u64 = hash_str("ellipse");
  static ref MAJOR__AXIS: u64 = hash_str("major-axis");
  static ref MINOR__AXIS: u64 = hash_str("minor-axis");
  static ref STARTING__ANGLE: u64 = hash_str("starting-angle");
  static ref ENDING__ANGLE: u64 = hash_str("ending-angle");
  static ref TEXT: u64 = hash_str("text");
  static ref FONT: u64 = hash_str("font");
  static ref SIZE: u64 = hash_str("size");
  static ref FACE: u64 = hash_str("face");
  static ref STYLE: u64 = hash_str("style");
  static ref WEIGHT: u64 = hash_str("weight");
  static ref BOLD: u64 = hash_str("bold");
  static ref NORMAL: u64 = hash_str("normal");
  static ref ITALIC: u64 = hash_str("italic");
  static ref FAMILY: u64 = hash_str("family");
  static ref DIRECTION: u64 = hash_str("direction");
  static ref ALIGNMENT: u64 = hash_str("alignment");
  static ref START: u64 = hash_str("start");
  static ref END: u64 = hash_str("end");
  static ref LEFT: u64 = hash_str("left");
  static ref RIGHT: u64 = hash_str("right");
  static ref CENTER: u64 = hash_str("center");
  static ref BEZIER: u64 = hash_str("bezier");
  static ref HTML_LOCATION: u64 = hash_str("html/location");
  static ref HASH: u64 = hash_str("hash");
  static ref HOST: u64 = hash_str("host");
  static ref HOST__NAME: u64 = hash_str("host-name");
  static ref ORIGIN: u64 = hash_str("origin");
  static ref PATH__NAME: u64 = hash_str("path-name");
  static ref PORT: u64 = hash_str("port");
  static ref PROTOCOL: u64 = hash_str("protocol");
  static ref SEARCH: u64 = hash_str("search");
  static ref SCALE: u64 = hash_str("scale");
}

pub fn render_value(&mut self, value: Value, document: &web_sys::Document, core: &mech_core::Core) -> Result<web_sys::Element, JsValue> {
  let mut div = document.create_element("div")?;
  match value {
    Value::String(chars) => {
      let contents_string = chars.to_string();
      div.set_inner_html(&contents_string);
    },
    Value::F32(x) => div.set_inner_html(&format!("{:.2?}", x)),
    Value::F64(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::U128(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::U64(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::U32(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::U16(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::U8(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::I128(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::I64(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::I32(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::I16(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::I8(x) => div.set_inner_html(&format!("{:?}", x)),
    Value::Reference(TableId::Global(table_id)) => {
      let table = core.get_table_by_id(table_id).unwrap();
      let rendered_ref = make_element(&table.borrow(), document, core)?;
      div.append_child(&rendered_ref)?;
    }
    x => log!("4745 {:?}",x),
  }
  Ok(div)
}

pub fn make_element(&mut self, table: &Table, document: &web_sys::Document, core: &mech_core::Core) -> Result<web_sys::Element, JsValue> {
  let mut container: web_sys::Element = document.create_element("div")?;
  let element_id = hash_str(&format!("div-{:?}", table.id));
  container.set_id(&format!("{:?}",element_id));
  container.set_attribute("table-id", &format!("{}", table.id))?;
  // First check to see if the table has a "type" column. If it doesn't, just render the table
  match table.col_map.get_index(&*KIND) {
    Ok(_) => {
      for row in 1..=table.rows {
        match table.get(&TableIndex::Index(row), &TableIndex::Alias(*KIND))  {
          Ok(Value::String(kind)) => {
            let raw_kind = kind.hash();
            // Render an HTML element
            if raw_kind == *DIV { render_div(table,&mut container, document, core)?; }
            else if raw_kind == *A { render_link(table,&mut container, document, core)?; }
            else if raw_kind == *IMG { render_img(table,&mut container,document, core)?; }
            else if raw_kind == *BUTTON { render_button(table, &mut container, document, core)?; }
            else if raw_kind == *SLIDER { render_slider(table, &mut container, document, core)?; }
            else if raw_kind == *CANVAS { render_canvas(table, &mut container, document, core)?; }
            else {
              log!("4744 {:?}", raw_kind);
            }
          }
          x => log!("4745 {:?}",x),
          Err(x) => log!("4746 {:?}",x),
        }
      }
    }
    // There's no Type column, so we are going to treat the table as a generic thing and just turn it into divs
    Err(_) => {
      // Make a div for each row
      for row in 1..=table.rows {
        let mut row_div = document.create_element("div")?;
        let element_id = hash_str(&format!("div-{:?}-{:?}", table.id, row));
        row_div.set_id(&format!("{:?}",element_id));
        // Make an internal div for each cell 
        for column in 1..=table.cols {
          // Get contents
          match table.get(&TableIndex::Index(row), &TableIndex::Index(column)) {
            Ok(contents) => {
              let mut cell_div = document.create_element("div")?;
              let element_id = hash_str(&format!("div-{:?}-{:?}-{:?}", table.id, row, column));
              let rendered = render_value(contents, document, core)?;
              rendered.set_id(&format!("{:?}",element_id));
              row_div.append_child(&rendered)?;
            }
            x => log!("4747 {:?}",x),
          }          
        }
        container.append_child(&row_div)?;
      }
    }
  }
  Ok(container)
}