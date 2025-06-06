# Diceberg!

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

Or do a release build:

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
```

Lookup inventories:
```shell
aws-vault exec staging -- dici info lookup inventory all
aws-vault exec staging -- dici info lookup inventory fxf yfc6-7rgw
aws-vault exec staging -- dici info lookup inventory iceberg _ac642f8374a4a7c17e855f828c41cf48
```

Schema:
```shell
aws-vault exec staging -- dici info table schema iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors
aws-vault exec staging -- dici info table schema core yfc6-7rgw
```

Execute sql:
```shell
aws-vault exec staging -- dici sql iceberg _ac642f8374a4a7c17e855f828c41cf48 dbo_vendors 'select count(*) from dbo_vendors'
aws-vault exec staging -- dici sql core yfc6-7rgw "select count(*) from 'yfc6-7rgw'"
```