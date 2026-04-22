window.BENCHMARK_DATA = {
  "lastUpdate": 1776870219753,
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
      },
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
          "id": "3389f08f973caa354eb2a74cd1e137a0fb198288",
          "message": "Config support added",
          "timestamp": "2026-04-16T01:39:13+02:00",
          "tree_id": "5ff4362f1dc7aa9192ea4b8dd13b9cc0defd9051",
          "url": "https://github.com/NBAFrigge/wax/commit/3389f08f973caa354eb2a74cd1e137a0fb198288"
        },
        "date": 1776296510172,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7581,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8389,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 12955,
            "range": "± 96",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 69454,
            "range": "± 996",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8387,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11455,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12397,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1516081,
            "range": "± 92382",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 1164207,
            "range": "± 1021713",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1097093,
            "range": "± 63359",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "068a70be1a3b6ef9c683b7d2a3e250c5de41db6c",
          "message": "readme.md updated",
          "timestamp": "2026-04-16T01:49:01+02:00",
          "tree_id": "ea5c90cc95933cf83e595a031c92a68784eba7f1",
          "url": "https://github.com/NBAFrigge/wax/commit/068a70be1a3b6ef9c683b7d2a3e250c5de41db6c"
        },
        "date": 1776297095254,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7538,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8523,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13449,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 84287,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8634,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11517,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12232,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1202491,
            "range": "± 142282",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 941270,
            "range": "± 183967",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 896986,
            "range": "± 66287",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "aba1e9dc17c95ed1373f948e62cb519a5c9b3f00",
          "message": "TTL on wax-store added",
          "timestamp": "2026-04-16T15:20:32+02:00",
          "tree_id": "9b13bbcb7c7644558dda6b67b1a0cad0f39b728c",
          "url": "https://github.com/NBAFrigge/wax/commit/aba1e9dc17c95ed1373f948e62cb519a5c9b3f00"
        },
        "date": 1776345822396,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7470,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8260,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13249,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 69357,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8331,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11423,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12154,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1269633,
            "range": "± 52279",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 1016001,
            "range": "± 558199",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 2932830,
            "range": "± 4631193",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "aba1e9dc17c95ed1373f948e62cb519a5c9b3f00",
          "message": "TTL on wax-store added",
          "timestamp": "2026-04-16T15:20:32+02:00",
          "tree_id": "9b13bbcb7c7644558dda6b67b1a0cad0f39b728c",
          "url": "https://github.com/NBAFrigge/wax/commit/aba1e9dc17c95ed1373f948e62cb519a5c9b3f00"
        },
        "date": 1776346108602,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 8492,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 9268,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 14448,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 80673,
            "range": "± 176",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 9432,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 13086,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 14019,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 906717,
            "range": "± 27506",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 721521,
            "range": "± 306109",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 747631,
            "range": "± 56881",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "4c95e2e1fb2cf26056332c3cd19482ebf477423a",
          "message": "isntat_paste option added on --instat-paste",
          "timestamp": "2026-04-16T15:34:31+02:00",
          "tree_id": "ab91f8cb7e845dc939769e964b6ba2d30cfc3e7b",
          "url": "https://github.com/NBAFrigge/wax/commit/4c95e2e1fb2cf26056332c3cd19482ebf477423a"
        },
        "date": 1776346633026,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7334,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8226,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 12949,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 72295,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8348,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11399,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12414,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1254758,
            "range": "± 27117",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 977219,
            "range": "± 146741",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 974564,
            "range": "± 53540",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "e1e7e171c06a96e1d49fa5e4dc22d123cef2050f",
          "message": "version changed 0.1.2 -> 0.1.3",
          "timestamp": "2026-04-16T15:36:54+02:00",
          "tree_id": "241f4265b0b1d7f6909aefa845ad91b2e8588ade",
          "url": "https://github.com/NBAFrigge/wax/commit/e1e7e171c06a96e1d49fa5e4dc22d123cef2050f"
        },
        "date": 1776346781790,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7359,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8222,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13121,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 76011,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8469,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11568,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12523,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1130054,
            "range": "± 14484",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 848897,
            "range": "± 358492",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 842419,
            "range": "± 37121",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "4c95e2e1fb2cf26056332c3cd19482ebf477423a",
          "message": "isntat_paste option added on --instat-paste",
          "timestamp": "2026-04-16T15:34:31+02:00",
          "tree_id": "ab91f8cb7e845dc939769e964b6ba2d30cfc3e7b",
          "url": "https://github.com/NBAFrigge/wax/commit/4c95e2e1fb2cf26056332c3cd19482ebf477423a"
        },
        "date": 1776346891922,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 4558,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 5371,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 9145,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 109105,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 9699,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 12418,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 13273,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 911963,
            "range": "± 15700",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 714284,
            "range": "± 1052962",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 745805,
            "range": "± 26447",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "83bd122557f1bf9eb2a941e5aeedec7a0bcfd8ec",
          "message": "version changed 0.1.2 -> 0.1.3",
          "timestamp": "2026-04-16T15:40:51+02:00",
          "tree_id": "241f4265b0b1d7f6909aefa845ad91b2e8588ade",
          "url": "https://github.com/NBAFrigge/wax/commit/83bd122557f1bf9eb2a941e5aeedec7a0bcfd8ec"
        },
        "date": 1776347016131,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7296,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8099,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 12974,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 66791,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8745,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 10803,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12290,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1414814,
            "range": "± 138339",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 1080814,
            "range": "± 164604",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1113263,
            "range": "± 69117",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "d25207480b084262df8b499b88b9906687892cf2",
          "message": "instant paste fixed",
          "timestamp": "2026-04-16T19:18:52+02:00",
          "tree_id": "f414b9eb1182b76a5487b9241a66b9ee8d208ab3",
          "url": "https://github.com/NBAFrigge/wax/commit/d25207480b084262df8b499b88b9906687892cf2"
        },
        "date": 1776360096292,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7325,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8132,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13001,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 74330,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8719,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11635,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12312,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1165921,
            "range": "± 21634",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 849513,
            "range": "± 262569",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 870785,
            "range": "± 46733",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "17f3240839e4f13d00519d3d7de600c702d2a9ac",
          "message": "fix dedup checking only last history entry",
          "timestamp": "2026-04-16T20:31:27+02:00",
          "tree_id": "1d2a189e5bac3d734fb8fb4c3b8d3652c79b5089",
          "url": "https://github.com/NBAFrigge/wax/commit/17f3240839e4f13d00519d3d7de600c702d2a9ac"
        },
        "date": 1776364455041,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7837,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8663,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13449,
            "range": "± 728",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 80641,
            "range": "± 367",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8578,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11702,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12055,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1273600,
            "range": "± 68755",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 940877,
            "range": "± 168352",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 984283,
            "range": "± 91979",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "8861d6f7cea2b60664f3dc61977047cb38d1aff2",
          "message": "minor refactor",
          "timestamp": "2026-04-16T20:39:05+02:00",
          "tree_id": "d784c9623540db78e9a498d5eb23b21acb58ef71",
          "url": "https://github.com/NBAFrigge/wax/commit/8861d6f7cea2b60664f3dc61977047cb38d1aff2"
        },
        "date": 1776364908310,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 8501,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 9254,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 14125,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 78809,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 9229,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 12084,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 13409,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 898710,
            "range": "± 11747",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 811537,
            "range": "± 506341",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 827526,
            "range": "± 103165",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "ce161243a73415d300d000a47ce97874b92a0077",
          "message": "blacklist on regex pattern added",
          "timestamp": "2026-04-16T22:40:58+02:00",
          "tree_id": "3afd3123e1a333e0ef663aca437f2b57318ff357",
          "url": "https://github.com/NBAFrigge/wax/commit/ce161243a73415d300d000a47ce97874b92a0077"
        },
        "date": 1776372230369,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 6539,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 7173,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 10898,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 59819,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 7262,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 9791,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 10290,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 6951939,
            "range": "± 9595724",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 1139438,
            "range": "± 266279",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1041198,
            "range": "± 330561",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "ce161243a73415d300d000a47ce97874b92a0077",
          "message": "blacklist on regex pattern added",
          "timestamp": "2026-04-16T22:40:58+02:00",
          "tree_id": "3afd3123e1a333e0ef663aca437f2b57318ff357",
          "url": "https://github.com/NBAFrigge/wax/commit/ce161243a73415d300d000a47ce97874b92a0077"
        },
        "date": 1776372874910,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 8490,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 9242,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13909,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 77822,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 9312,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 12707,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 13670,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 878593,
            "range": "± 9394",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 694026,
            "range": "± 525441",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 751048,
            "range": "± 27968",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "48525589a287bcdc210a213ebb1fa5333a4e0738",
          "message": "add regex blacklist, primary selection and config options",
          "timestamp": "2026-04-16T23:58:07+02:00",
          "tree_id": "9a3f6e12ddd6d286b47a0bdfb6ebfdb966d28823",
          "url": "https://github.com/NBAFrigge/wax/commit/48525589a287bcdc210a213ebb1fa5333a4e0738"
        },
        "date": 1776376854319,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7364,
            "range": "± 329",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8225,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13185,
            "range": "± 91",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 67898,
            "range": "± 526",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8454,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11339,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12425,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1243754,
            "range": "± 49022",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 923480,
            "range": "± 60595",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 957700,
            "range": "± 3124960",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "4f4b01adb6fb1d455c6fdf56d72952087f3a6ff8",
          "message": "primary selection toggle fixed",
          "timestamp": "2026-04-17T00:05:02+02:00",
          "tree_id": "c7a079eeffeff9d021c95db442a318d0fac90e3a",
          "url": "https://github.com/NBAFrigge/wax/commit/4f4b01adb6fb1d455c6fdf56d72952087f3a6ff8"
        },
        "date": 1776377264487,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7302,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8179,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 12745,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 77628,
            "range": "± 413",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8336,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11442,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 11957,
            "range": "± 509",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1137731,
            "range": "± 26319",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 853362,
            "range": "± 37070",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 947807,
            "range": "± 2789696",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "ac28271238d7944611542f2383994a916b03d35b",
          "message": "readme updated",
          "timestamp": "2026-04-17T00:10:07+02:00",
          "tree_id": "2d60dcba3c576e6e9ceb6cda86b74bd6c3b2975e",
          "url": "https://github.com/NBAFrigge/wax/commit/ac28271238d7944611542f2383994a916b03d35b"
        },
        "date": 1776377570643,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7464,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8364,
            "range": "± 480",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 12743,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 69098,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8474,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 10928,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12312,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1163257,
            "range": "± 16909",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 867924,
            "range": "± 139604",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 963130,
            "range": "± 2596087",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "46405cb99e50e4bb3b88daa9587d389add7a7877",
          "message": "fuzzy search flag added + history bug fixed",
          "timestamp": "2026-04-19T15:13:53+02:00",
          "tree_id": "2107b304a9db81c39622622d8052c1710706e1da",
          "url": "https://github.com/NBAFrigge/wax/commit/46405cb99e50e4bb3b88daa9587d389add7a7877"
        },
        "date": 1776604599146,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7480,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8433,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13320,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 71251,
            "range": "± 432",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8590,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 11392,
            "range": "± 210",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 12620,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1994311,
            "range": "± 244924",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 1602785,
            "range": "± 179994",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1757452,
            "range": "± 160376",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "43a311d334d834b5ca40d3a0e03b0b7484a3123b",
          "message": "pin function added",
          "timestamp": "2026-04-19T17:31:50+02:00",
          "tree_id": "33415d14ac281f2299cfc66932c5925f8066c9df",
          "url": "https://github.com/NBAFrigge/wax/commit/43a311d334d834b5ca40d3a0e03b0b7484a3123b"
        },
        "date": 1776612874474,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 8415,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 9255,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13828,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 75406,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 10274,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 13539,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 14795,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 906824,
            "range": "± 18443",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 890984,
            "range": "± 255086",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1091696,
            "range": "± 148925",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "d268ec54c925422f416702825656332666ce7439",
          "message": "delete on pinned image bug fixed",
          "timestamp": "2026-04-19T17:40:44+02:00",
          "tree_id": "8e917993ea53de6556a1d30f1231120304167dcb",
          "url": "https://github.com/NBAFrigge/wax/commit/d268ec54c925422f416702825656332666ce7439"
        },
        "date": 1776613402734,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7586,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8367,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 13259,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 73161,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 9257,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 12072,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 13105,
            "range": "± 460",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1275609,
            "range": "± 91582",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 924712,
            "range": "± 49553",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1231647,
            "range": "± 361091",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "a5e46971bd386c23697052b817c07aa6b253e6bc",
          "message": "Trim_oldest bug on pinned clips fixed",
          "timestamp": "2026-04-22T16:46:39+02:00",
          "tree_id": "5f4c887bcca54776d1137e0b80492d5f3c50860f",
          "url": "https://github.com/NBAFrigge/wax/commit/a5e46971bd386c23697052b817c07aa6b253e6bc"
        },
        "date": 1776869423914,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 8372,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 9295,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 14084,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 75260,
            "range": "± 370",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 9556,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 13202,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 14488,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 933588,
            "range": "± 17057",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 1148475,
            "range": "± 491349",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1133798,
            "range": "± 53437",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "1ad507076b65bd64e41c9d18ea6d2ea987c96d22",
          "message": "performance improvements in store operations",
          "timestamp": "2026-04-22T16:54:49+02:00",
          "tree_id": "fc72eb0ebe59b304595d7a1ac7c24eb7112959f4",
          "url": "https://github.com/NBAFrigge/wax/commit/1ad507076b65bd64e41c9d18ea6d2ea987c96d22"
        },
        "date": 1776869864253,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7444,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8241,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 12971,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 77626,
            "range": "± 283",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 8933,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 12281,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 13246,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1137337,
            "range": "± 13957",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 878583,
            "range": "± 157946",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1170099,
            "range": "± 46567",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "ea247268fa523fd2c8e2da20d96ddb642e8b8bdc",
          "message": "multibyte chars handling added",
          "timestamp": "2026-04-22T17:00:54+02:00",
          "tree_id": "63045b42035b40dc99f1eaf3e8266bcc08917edc",
          "url": "https://github.com/NBAFrigge/wax/commit/ea247268fa523fd2c8e2da20d96ddb642e8b8bdc"
        },
        "date": 1776870219464,
        "tool": "cargo",
        "benches": [
          {
            "name": "read_cache/entries/50",
            "value": 7440,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/500",
            "value": 8343,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/5000",
            "value": 15435,
            "range": "± 1025",
            "unit": "ns/iter"
          },
          {
            "name": "read_cache/entries/50000",
            "value": 78877,
            "range": "± 1968",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/50",
            "value": 9232,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/500",
            "value": 12245,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "get/db_entries/5000",
            "value": 13279,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/0",
            "value": 1340595,
            "range": "± 37343",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/100",
            "value": 1067813,
            "range": "± 82974",
            "unit": "ns/iter"
          },
          {
            "name": "push_text/db_entries/1000",
            "value": 1261311,
            "range": "± 84267",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}