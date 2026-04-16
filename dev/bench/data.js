window.BENCHMARK_DATA = {
  "lastUpdate": 1776377264908,
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
      }
    ]
  }
}