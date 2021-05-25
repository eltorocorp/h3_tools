A tool to apply h3's polyfill to GeoJSON.

Caveats: This tool uses [geo-json's](https://github.com/georust/geojson) `quick_collection` function. see https://docs.rs/geojson/0.21.0/geojson/index.html#conversion-to-geo-objects

Example:

```bash
$ echo '{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "properties": {},
      "geometry": {
        "type": "Polygon",
        "coordinates": [
          [
            [
              -77.0384931564331,
              38.91561302513129
            ],
            [
              -77.0384931564331,
              38.9125910546825
            ],
            [
              -77.03454494476318,
              38.912607751005346
            ],
            [
              -77.03454494476318,
              38.91559632951548
            ],
            [
              -77.0384931564331,
              38.91561302513129
            ]
          ]
        ]
      }
    }
  ]
}' | geo_polyfill -r 10

622247184025092095
622247186430853119
622247186431180799
622247184025124863
622247186430722047
622247186430885887
622247186430754815
622247186430918655
```
Caveats: This tool uses the `quick_collection` function. see https://docs.rs/geojson/0.21.0/geojson/index.html#conversion-to-geo-objects