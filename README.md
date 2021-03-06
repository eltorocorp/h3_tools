# h3_tools

Originally just a tool to apply [H3's](https://eng.uber.com/h3/) polyfill to GeoJSON, but will mostly likely become a collection of tools to work with geotypes and H3. This'll most likely be expanded to include other types such as WKT or various WKBs.

## Contributing
this was a personal tool, updates will only happen once I get annoyed enough with a type to add compatibility. If you get the same way about another type, feel free to drop a PR.

## Caveats
This tool uses [geo-json's](https://github.com/georust/geojson) `quick_collection` function. see [this documentation](https://docs.rs/geojson/0.21.0/geojson/index.html#conversion-to-geo-objects) for more information.

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