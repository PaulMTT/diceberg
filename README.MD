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


```shell
aws-vault exec staging -- cargo run --bin dici sql core --fxf 'yfc6-7rgw' --query "select count(distinct(vendorname)) as distinct_count, count(vendorname) as count from 'yfc6-7rgw'"
aws-vault exec staging -- cargo run --bin dici sql iceberg --location '_ac642f8374a4a7c17e855f828c41cf48' --schema-table 'dbo_vendors' --query "select count(distinct(vendorname)) as distinct_count, count(vendorname) as count from dbo_vendors"

```

```shell
aws-vault exec staging -- cargo run --bin dici info schema core --fxf 'yfc6-7rgw'
aws-vault exec staging -- cargo run --bin dici info schema iceberg --location '_ac642f8374a4a7c17e855f828c41cf48' --schema-table 'dbo_vendors'

```