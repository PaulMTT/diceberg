# Requirements

You will need the following environment variables.

## AWS

- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `AWS_REGION`

These are usually set for you when executing

```
aws-vault exec <profile> -- <command>
```

## DICI

- `DICI_WAREHOUSE` - The bucket path like s3://tyler-iceberg-catalog-us-west-2-staging-alpha/

- `DICI_MANAGEMENT_ADDRESS` - The address of the dici management server like http://internal-dici-management-alb-staging-1989759444.us-west-2.elb.amazonaws.com

The management address is only needed if querying via a core fxf, this also **requires you to be on the correct VPN**.