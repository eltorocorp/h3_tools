use anyhow::{Result, Error};
use geo_types::{Geometry, GeometryCollection, Polygon};
use geojson::{GeoJson, quick_collection};
use h3ron::{H3Cell, Index};
use rayon::prelude::IntoParallelIterator;
use itertools::Itertools;
use std::collections::HashMap;

pub fn marshal_polygons(gj: &GeoJson) -> Result<Vec<Polygon<f64>>> {
    let collection: GeometryCollection<f64> = quick_collection(gj)?;
    Ok(collection.0.into_iter().flat_map(|g| flatten_geometry(g)).flatten().collect())
}

pub fn flatten_geometry(g: Geometry<f64>) -> Result<Vec<Polygon<f64>>> {
    match g {
        Geometry::Point(_) => Err(Error::msg("Unsupported type")),          // todo get res 8-11 cell of point, polyfill that ?
        Geometry::Line(_l) => Err(Error::msg("Unsupported type")),          // todo same as above but bbox of line ?
        Geometry::LineString(_ls) => Err(Error::msg("Unsupported type")),   // todo same as about
        Geometry::Polygon(p) => Ok(vec![p]),
        Geometry::MultiPoint(_mp) => Err(Error::msg("Unsupported type")),   // todo bbox of all points, or vec<polygon> of res 8-11 cells of point ?
        Geometry::MultiLineString(mls) => Ok(mls
            .0
            .into_iter()
            .map(|ls| Polygon::new(ls, vec![]))
            .collect()),
        Geometry::MultiPolygon(mp) => Ok(mp.0),
        Geometry::GeometryCollection(gc) => {
            Ok(gc.0.into_iter().flat_map(flatten_geometry).flatten().collect())
        }
        Geometry::Rect(r) => Ok(vec![r.to_polygon()]),
        Geometry::Triangle(t) => Ok(vec![t.to_polygon()]),
    }
}

pub fn eat_neighbors(indices: Vec<H3Cell>) -> Vec<H3Cell> {
    // but what if we started with neighbors?
    let mut parental_advisory: HashMap<H3Cell, Vec<H3Cell>> = HashMap::new();
    for (index, parent) in indices
        .into_iter()
        .map(|i| (i, i.get_parent(i.resolution() - 1)))
        .filter(|(h3, p)| p.is_ok())
        .map(|(h3, p)| (h3, p.unwrap()))
    {
        parental_advisory
            .entry(parent)
            .and_modify(|mut children| children.push(index))
            .or_insert({
                let mut v = Vec::<H3Cell>::with_capacity(6);
                v.push(index);
                v
            });
    }

    let mut trimmed = Vec::<H3Cell>::with_capacity(parental_advisory.len() * 6);

    for (parent, mut children) in parental_advisory {
        if children.len() == 6 {
            trimmed.push(parent);
        } else {
            trimmed.extend(children.drain(..))
        }
    }

    trimmed
}

fn full_brood(parent: &H3Cell, potentials: &Vec<H3Cell>) -> bool {
    todo!()
}


#[cfg(test)]
mod tests {
    use geojson::GeoJson;
    use crate::geo_utils::marshal_polygons;

    #[test]
    fn test_into() {
        let geojson_str = r#"
    {
      "type": "GeometryCollection",
      "geometries": [
        {"type": "Point", "coordinates": [0,1]},
        {"type": "MultiPoint", "coordinates": [[-1,0],[1,0]]},
        {"type": "LineString", "coordinates": [[-1,-1],[1,-1]]},
        {"type": "MultiLineString", "coordinates": [
          [[-2,-2],[2,-2]],
          [[-3,-3],[3,-3]]
        ]},
        {"type": "Polygon", "coordinates": [
          [[-5,-5],[5,-5],[0,5],[-5,-5]],
          [[-4,-4],[4,-4],[0,4],[-4,-4]]
        ]},
        { "type": "MultiPolygon", "coordinates": [[
          [[-7,-7],[7,-7],[0,7],[-7,-7]],
          [[-6,-6],[6,-6],[0,6],[-6,-6]]
        ],[
          [[-9,-9],[9,-9],[0,9],[-9,-9]],
          [[-8,-8],[8,-8],[0,8],[-8,-8]]]
        ]},
        {"type": "GeometryCollection", "geometries": [
          {"type": "Polygon", "coordinates": [
            [[-5.5,-5.5],[5,-5],[0,5],[-5,-5]],
            [[-4,-4],[4,-4],[0,4],[-4.5,-4.5]]
          ]}
        ]}
      ]
    }
    "#;
        let geojson = geojson_str.parse::<GeoJson>().unwrap();
        let res = marshal_polygons(&geojson);

        assert!(res.is_ok())
    }
}