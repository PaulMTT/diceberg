# Diceberg!

Accessing dici assets without using our trino cluster or core.

This hits our dici-management server for some information.

Data is fetched directly from s3 via iceberg through a glue catalog.

## Requirements

You will need environment variables listed below.

### AWS

- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `AWS_REGION`

There are typically set via invoking

```
aws-vault exec <profile> -- <command>
```

### DICI

- `DICI_WAREHOUSE` - The bucket path like `s3://tyler-iceberg-catalog-us-west-2-staging-alpha/`
- `DICI_MANAGEMENT_ADDRESS` - The address of the dici management server like
  `http://internal-dici-management-alb-staging-1989759444.us-west-2.elb.amazonaws.com`

The management address is only needed if querying via a core fxf, this also requires you to be on the correct VPN.

## Features

- Lookup registrations
- Lookup inventories
- Get schema of table
- Execute sql against table
- CLI that exposes everything through sub commands
- Lib with table/sql api, catalog/context access
- Table manifest size, table data size
- Table partitions
- Snapshot history, snapshot details
- JSON or IPC(arrow format) sql output format
- IPC print utility
- Execute sql against dataframes(IPC) input to stdin

## Demo

For now we have, in staging, the inventory:

```
{
  "id": {
    "domain": {
      "domain": "erp-pro-10-dici.test-socrata.com"
    },
    "icebergLocation": {
      "icebergLocation": "_ac642f8374a4a7c17e855f828c41cf48"
    },
    "schemaTable": {
      "schemaTable": "dbo_vendors"
    }
  },
  "fourByFour": {
    "fourByFour": "yfc6-7rgw"
  },
  "createdAt": "2025-05-29T21:34:11.165908Z",
  "updatedAt": "2025-05-29T21:34:11.171485Z"
}
```

First, set these envs for staging:

```shell
export DICI_MANAGEMENT_ADDRESS='http://internal-dici-management-alb-staging-1989759444.us-west-2.elb.amazonaws.com'
export DICI_WAREHOUSE='s3://tyler-iceberg-catalog-us-west-2-staging-alpha/'
```

Install the release version of the tool:

```shell
cargo install --path .
```

Make sure to add cargo bin to your path like

```shell
export PATH="$HOME/.cargo/bin:$PATH"
```

and then

Lookup registrations:
```shell
aws-vault exec staging -- dici info lookup registration all
aws-vault exec staging -- dici info lookup registration path erp_pro_10
aws-vault exec staging -- dici info lookup registration filtered erp domain erp-pro-10-dici.test-socrata.com

[
  {
    "id": {
      "path": "erp_pro_10/incode_chicoarearapdca"
    },
    "icebergLocation": {
      "icebergLocation": "_ac642f8374a4a7c17e855f828c41cf48"
    },
    "createdAt": "2025-04-23T11:00:13.656208Z",
    "updatedAt": "2025-05-29T21:10:03.553041Z",
    "metadata": {
      "domain": "erp-pro-10-dici.test-socrata.com"
    }
  }
]
```

Lookup inventories:
```shell
aws-vault exec staging -- dici info lookup inventory all
aws-vault exec staging -- dici info lookup inventory fxf yfc6-7rgw
aws-vault exec staging -- dici info lookup inventory iceberg _ac642f8374a4a7c17e855f828c41cf48

[
  {
    "id": {
      "domain": {
        "domain": "erp-pro-10-dici.test-socrata.com"
      },
      "icebergLocation": {
        "icebergLocation": "_ac642f8374a4a7c17e855f828c41cf48"
      },
      "schemaTable": {
        "schemaTable": "dbo_vendors"
      }
    },
    "fourByFour": {
      "fourByFour": "yfc6-7rgw"
    },
    "createdAt": "2025-05-29T21:34:11.165908Z",
    "updatedAt": "2025-05-29T21:34:11.171485Z"
  }
]
```

Schema:
```shell
aws-vault exec staging -- dici info table schema iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors
aws-vault exec staging -- dici info table schema core yfc6-7rgw

[
  {
    "id": 1,
    "name": "vendorname",
    "required": false,
    "type": "string"
  },
]
```

Table snapshot history:
```shell
aws-vault exec staging -- dici info table history all iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors
aws-vault exec staging -- dici info table history all core yfc6-7rgw

[
  {
    "snapshot-id": 6552266534160396918,
    "timestamp-ms": 1744974274978
  },
]
```

Table snapshot details:
```shell
aws-vault exec staging -- dici info table history snapshot core yfc6-7rgw 5276000349694124598
aws-vault exec staging -- dici info table history snapshot iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors 5276000349694124598

{
  "snapshot-id": 5276000349694124598,
  "sequence-number": 89,
  "timestamp-ms": 1749207971702,
  "manifest-list": "s3://tyler-iceberg-catalog-us-west-2-staging-alpha/_ac642f8374a4a7c17e855f828c41cf48/dbo_vendors/metadata/snap-5276000349694124598-1-c80c4860-ac9f-4adf-8898-0a8b6ee6f81d.avro",
  "summary": {
    "operation": "append",
    "added-data-files": "1",
    "total-data-files": "1",
    "added-files-size": "18827",
    "trino_query_id": "20250606_110610_00012_4mu88",
    "total-records": "463",
    "iceberg-version": "Apache Iceberg 1.8.1 (commit 9ce0fcf0af7becf25ad9fc996c3bad2afdcfd33d)",
    "trino_user": "queue-processor",
    "total-position-deletes": "0",
    "added-records": "463",
    "total-files-size": "18827",
    "total-equality-deletes": "0",
    "engine-version": "474",
    "changed-partition-count": "1",
    "engine-name": "trino",
    "total-delete-files": "0"
  },
  "schema-id": 0
}
```

Table manifest size in bytes:
```shell
aws-vault exec staging -- dici info table stats manifest-size core yfc6-7rgw
aws-vault exec staging -- dici info table stats manifest-size iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors

{
  "manifest_size_bytes": 347695
}
```

Table data size in bytes:
```shell
aws-vault exec staging -- dici info table stats data-size core yfc6-7rgw
aws-vault exec staging -- dici info table stats data-size iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors

{
  "data_size_bytes": 768871
}
```

Table partitions:
```shell
aws-vault exec staging -- dici info table partition core yfc6-7rgw
aws-vault exec staging -- dici info table partition iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors

{
  "spec-id": 0,
  "fields": []
}
```

Execute sql:
```shell
aws-vault exec staging -- dici sql iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors 'select count(*) from dbo_vendors'
aws-vault exec staging -- dici sql core yfc6-7rgw "select count(*) from 'yfc6-7rgw'"


[{"count(*)":463}]
```

Execute sql, outputting arrow IPC, and then reading and printing the dataframe:
```shell
aws-vault exec staging -- dici sql iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors 'select count(*) from dbo_vendors' --format ipc | dici util ipc print
aws-vault exec staging -- dici sql core yfc6-7rgw "select count(*) from 'yfc6-7rgw'" --format ipc | dici util ipc print


shape: (1, 1)
┌──────────┐
│ count(*) │
│ ---      │
│ i64      │
╞══════════╡
│ 463      │
└──────────┘
```

You can also execute sql against raw dataframes in ipc format, the table identifier will be `this`:
```shell
aws-vault exec staging -- dici sql core yfc6-7rgw "select * from 'yfc6-7rgw' limit 100" --format ipc | dici util ipc query "select * from this order by vendorname limit 1" --format ipc | dici util ipc print

shape: (1, 15)
┌────────────┬──────────┬──────────────────┬────────────────┬───┬────────────┬───────────────────┬────────────────────┬────────────────────┐
│ vendorname ┆ vendorid ┆ vendoraddress1   ┆ vendoraddress2 ┆ … ┆ fiscalyear ┆ vendorcontactname ┆ vendorcontactphone ┆ vendorcontactemail │
│ ---        ┆ ---      ┆ ---              ┆ ---            ┆   ┆ ---        ┆ ---               ┆ ---                ┆ ---                │
│ str        ┆ str      ┆ str              ┆ str            ┆   ┆ i64        ┆ str               ┆ str                ┆ str                │
╞════════════╪══════════╪══════════════════╪════════════════╪═══╪════════════╪═══════════════════╪════════════════════╪════════════════════╡
│ AARON HAAR ┆ 383000   ┆ 581 WHITE AVENUE ┆                ┆ … ┆ 2025       ┆ null              ┆ null               ┆ null               │
└────────────┴──────────┴──────────────────┴────────────────┴───┴────────────┴───────────────────┴────────────────────┴────────────────────┘
```