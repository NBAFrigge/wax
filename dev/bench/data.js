window.BENCHMARK_DATA = {
  "lastUpdate": 1776290586876,
  "repoUrl": "https://github.com/NBAFrigge/wax",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "frigerioalessandro659@gmail.com",
            "name": "Frigge",
            "username": "NBAFrigge"
          },
          "committer": {
            "email": "frigerioalessandro659@gmail.com",
            "name": "Frigge",
            "username": "NBAFrigge"
          },
          "distinct": true,
          "id": "72f3d114a86fa3394a90382ba7d829ca689c908f",
          "message": "reduce CI sample size and drop 10k push variant",
          "timestamp": "2026-04-15T23:55:30+02:00",
          "tree_id": "e432e65948f5b84a410974f63b4b1dbea9d27f12",
          "url": "https://github.com/NBAFrigge/wax/commit/72f3d114a86fa3394a90382ba7d829ca689c908f"
        },
        "date": 1776290315752,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7833,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8914,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13435,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 78212,
            "range": "± 282",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 7460,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 10178,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 11066,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1183432,
            "range": "± 36147",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 850881,
            "range": "± 65735",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 863611,
            "range": "± 46194",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Frigge",
            "username": "NBAFrigge",
            "email": "frigerioalessandro659@gmail.com"
          },
          "committer": {
            "name": "Frigge",
            "username": "NBAFrigge",
            "email": "frigerioalessandro659@gmail.com"
          },
          "id": "72f3d114a86fa3394a90382ba7d829ca689c908f",
          "message": "reduce CI sample size and drop 10k push variant",
          "timestamp": "2026-04-15T21:55:30Z",
          "url": "https://github.com/NBAFrigge/wax/commit/72f3d114a86fa3394a90382ba7d829ca689c908f"
        },
        "date": 1776290586478,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 6997,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 7790,
            "range": "± 177",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 11164,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 58359,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 7052,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 9043,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 10206,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1408100,
            "range": "± 57457428",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 994651,
            "range": "± 747634",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1132189,
            "range": "± 1830115",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/10000",
            "value": 1236148,
            "range": "± 37331338",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}