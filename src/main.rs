mod args;
mod geo_utils;

use std::{io, process};
use std::io::BufRead;

use anyhow::Result;
use geojson::GeoJson;
use h3ron::{ToH3Indexes, Index, H3Cell};
use rayon::prelude::*;
use structopt::StructOpt;

use crate::args::Args;
use crate::geo_utils::eat_neighbors;

fn main() -> Result<()> {
    let args: Args = Args::from_args();

    let string_input = collect_input();

    let gj_geometry = string_input.parse::<GeoJson>()?;

    let gt_polygons = geo_utils::marshal_polygons(&gj_geometry)?;

    let h3_indices = gt_polygons
        .par_iter()
        .flat_map(|poly| poly.to_h3_indexes(args.resolution))
        .flatten()
        .collect::<Vec<H3Cell>>();

    let results = h3_indices
        .into_iter()
        .map(|h3| {
            let i = h3.h3index();
            println!("{}", i);
            i
        })
        .collect::<Vec<u64>>();
// 53
    if results.is_empty() {
        println!("Resolution too high")
    }

    Ok(())
}

fn collect_input() -> String {
    let stdin = io::stdin();
    let mut input = String::with_capacity(4096);

    for result in stdin.lock().lines() {
        if let Ok(line) = result {
            input.push_str(&line);
        } else {
            println!("Error: Broken pipe");
            process::exit(1)
        }
    }

    input
}