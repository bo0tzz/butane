//! Butane migrations embedded in Rust.
use std::result::Result;

use butane::migrations::MemMigrations;

/// Load the butane migrations embedded in Rust.
pub fn get_migrations() -> Result<MemMigrations, butane::Error> {
    let json = r#"{
  "migrations": {
    "20201229_144636751_init": {
      "name": "20201229_144636751_init",
      "db": {
        "tables": {
          "Blog": {
            "name": "Blog",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "Known": "BigInt"
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "name",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "Post": {
            "name": "Post",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "Known": "Int"
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "title",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "body",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "published",
                "sqltype": {
                  "Known": "Bool"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "blog",
                "sqltype": {
                  "Known": "BigInt"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "byline",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "Post_tags_Many": {
            "name": "Post_tags_Many",
            "columns": [
              {
                "name": "owner",
                "sqltype": {
                  "Known": "Int"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "has",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "Tag": {
            "name": "Tag",
            "columns": [
              {
                "name": "tag",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": true,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          }
        },
        "extra_types": {}
      },
      "from": null,
      "up": {
        "sqlite": "CREATE TABLE Tag (\ntag TEXT NOT NULL PRIMARY KEY\n);\nCREATE TABLE Post_tags_Many (\nowner INTEGER NOT NULL,\nhas TEXT NOT NULL\n);\nCREATE TABLE Post (\nid INTEGER NOT NULL PRIMARY KEY,\ntitle TEXT NOT NULL,\nbody TEXT NOT NULL,\npublished INTEGER NOT NULL,\nblog INTEGER NOT NULL,\nbyline TEXT \n);\nCREATE TABLE Blog (\nid INTEGER NOT NULL PRIMARY KEY,\nname TEXT NOT NULL\n);\nCREATE TABLE butane_migrations (\nname TEXT NOT NULL PRIMARY KEY\n);"
      },
      "down": {
        "sqlite": "DROP TABLE Blog;\nDROP TABLE Post_tags_Many;\nDROP TABLE Post;\nDROP TABLE Tag;"
      }
    },
    "20201229_171630604_likes": {
      "name": "20201229_171630604_likes",
      "db": {
        "tables": {
          "Blog": {
            "name": "Blog",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "Known": "BigInt"
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "name",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "Post": {
            "name": "Post",
            "columns": [
              {
                "name": "id",
                "sqltype": {
                  "Known": "Int"
                },
                "nullable": false,
                "pk": true,
                "auto": true,
                "unique": false,
                "default": null
              },
              {
                "name": "title",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "body",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "published",
                "sqltype": {
                  "Known": "Bool"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "blog",
                "sqltype": {
                  "Known": "BigInt"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "byline",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": true,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "likes",
                "sqltype": {
                  "Known": "Int"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "Post_tags_Many": {
            "name": "Post_tags_Many",
            "columns": [
              {
                "name": "owner",
                "sqltype": {
                  "Known": "Int"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              },
              {
                "name": "has",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": false,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          },
          "Tag": {
            "name": "Tag",
            "columns": [
              {
                "name": "tag",
                "sqltype": {
                  "Known": "Text"
                },
                "nullable": false,
                "pk": true,
                "auto": false,
                "unique": false,
                "default": null
              }
            ]
          }
        },
        "extra_types": {}
      },
      "from": "20201229_144636751_init",
      "up": {
        "sqlite": "ALTER TABLE Post ADD COLUMN likes INTEGER NOT NULL DEFAULT 0;"
      },
      "down": {
        "sqlite": "CREATE TABLE Post__butane_tmp (\nid INTEGER NOT NULL PRIMARY KEY,\ntitle TEXT NOT NULL,\nbody TEXT NOT NULL,\npublished INTEGER NOT NULL,\nblog INTEGER NOT NULL,\nbyline TEXT \n);\nINSERT INTO Post__butane_tmp SELECT id, title, body, published, blog, byline FROM Post;\nDROP TABLE Post;\nALTER TABLE Post__butane_tmp RENAME TO Post;"
      }
    }
  },
  "current": {
    "name": "current",
    "db": {
      "tables": {},
      "extra_types": {}
    },
    "from": null,
    "up": {},
    "down": {}
  },
  "latest": "20201229_171630604_likes"
}"#;
    MemMigrations::from_json(json)
}