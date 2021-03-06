---
delivery_guarantee: "at_least_once"
description: "The Vector `aws_kinesis_streams` sink batches `log` events to Amazon Web Service's Kinesis Data Stream service via the `PutRecords` API endpoint."
event_types: ["log"]
issues_url: https://github.com/timberio/vector/issues?q=is%3Aopen+is%3Aissue+label%3A%22sink%3A+aws_kinesis_streams%22
operating_systems: ["Linux","MacOS","Windows"]
sidebar_label: "aws_kinesis_streams|[\"log\"]"
source_url: https://github.com/timberio/vector/tree/master/src/sinks/aws_kinesis_streams.rs
status: "beta"
title: "AWS Kinesis Data Streams Sink"
unsupported_operating_systems: []
---

The Vector `aws_kinesis_streams` sink [batches](#buffers--batches) [`log`][docs.data-model.log] events to [Amazon Web Service's Kinesis Data Stream service][urls.aws_kinesis_data_streams] via the [`PutRecords` API endpoint](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecords.html).

<!--
     THIS FILE IS AUTOGENERATED!

     To make changes please edit the template located at:

     website/docs/reference/sinks/aws_kinesis_streams.md.erb
-->

## Configuration

import Tabs from '@theme/Tabs';

<Tabs
  block={true}
  defaultValue="common"
  values={[
    { label: 'Common', value: 'common', },
    { label: 'Advanced', value: 'advanced', },
  ]
}>

import TabItem from '@theme/TabItem';

<TabItem value="common">

import CodeHeader from '@site/src/components/CodeHeader';

<CodeHeader fileName="vector.toml" learnMoreUrl="/docs/setup/configuration/"/ >

```toml
[sinks.my_sink_id]
  # REQUIRED - General
  type = "aws_kinesis_streams" # must be: "aws_kinesis_streams"
  inputs = ["my-source-id"] # example
  region = "us-east-1" # example, relevant when host = ""
  stream_name = "my-stream" # example

  # REQUIRED - requests
  encoding = "json" # example, enum

  # OPTIONAL - General
  partition_key_field = "user_id" # example, no default
```

</TabItem>
<TabItem value="advanced">

<CodeHeader fileName="vector.toml" learnMoreUrl="/docs/setup/configuration/" />

```toml
[sinks.my_sink_id]
  # REQUIRED - General
  type = "aws_kinesis_streams" # must be: "aws_kinesis_streams"
  inputs = ["my-source-id"] # example
  region = "us-east-1" # example, relevant when host = ""
  stream_name = "my-stream" # example

  # REQUIRED - requests
  encoding = "json" # example, enum

  # OPTIONAL - General
  assume_role = "arn:aws:iam::123456789098:role/my_role" # example, no default
  endpoint = "127.0.0.0:5000/path/to/service" # example, no default, relevant when region = ""
  partition_key_field = "user_id" # example, no default

  # OPTIONAL - Batch
  [sinks.my_sink_id.batch]
    max_events = 500 # default, events
    timeout_secs = 1 # default, seconds

  # OPTIONAL - Buffer
  [sinks.my_sink_id.buffer]
    # OPTIONAL
    type = "memory" # default, enum
    max_events = 500 # default, events, relevant when type = "memory"
    when_full = "block" # default, enum

    # REQUIRED
    max_size = 104900000 # example, bytes, relevant when type = "disk"

  # OPTIONAL - Request
  [sinks.my_sink_id.request]
    in_flight_limit = 5 # default, requests
    rate_limit_duration_secs = 1 # default, seconds
    rate_limit_num = 5 # default
    retry_attempts = -1 # default
    retry_initial_backoff_secs = 1 # default, seconds
    retry_max_duration_secs = 10 # default, seconds
    timeout_secs = 30 # default, seconds
```

</TabItem>

</Tabs>

## Options

import Fields from '@site/src/components/Fields';

import Field from '@site/src/components/Field';

<Fields filters={true}>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={["arn:aws:iam::123456789098:role/my_role"]}
  name={"assume_role"}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

### assume_role

The ARN of an [IAM role][urls.aws_iam_role] to assume at startup. See [AWS Authentication](#aws-authentication) for more info.


</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"batch"}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"table"}
  unit={null}
  >

### batch

Configures the sink batching behavior.

<Fields filters={false}>


<Field
  common={true}
  defaultValue={500}
  enumValues={null}
  examples={[500]}
  name={"max_events"}
  path={"batch"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"int"}
  unit={"events"}
  >

#### max_events

The maximum size of a batch, in events, before it is flushed. See [Buffers & Batches](#buffers--batches) for more info.


</Field>


<Field
  common={true}
  defaultValue={1}
  enumValues={null}
  examples={[1]}
  name={"timeout_secs"}
  path={"batch"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"int"}
  unit={"seconds"}
  >

#### timeout_secs

The maximum age of a batch before it is flushed. See [Buffers & Batches](#buffers--batches) for more info.


</Field>


</Fields>

</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"buffer"}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"table"}
  unit={null}
  >

### buffer

Configures the sink specific buffer behavior.

<Fields filters={false}>


<Field
  common={true}
  defaultValue={500}
  enumValues={null}
  examples={[500]}
  name={"max_events"}
  path={"buffer"}
  relevantWhen={{"type":"memory"}}
  required={true}
  templateable={false}
  type={"int"}
  unit={"events"}
  >

#### max_events

The maximum number of [events][docs.data-model] allowed in the buffer. See [Buffers & Batches](#buffers--batches) for more info.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[104900000]}
  name={"max_size"}
  path={"buffer"}
  relevantWhen={{"type":"disk"}}
  required={true}
  templateable={false}
  type={"int"}
  unit={"bytes"}
  >

#### max_size

The maximum size of the buffer on the disk.


</Field>


<Field
  common={true}
  defaultValue={"memory"}
  enumValues={{"memory":"Stores the sink's buffer in memory. This is more performant, but less durable. Data will be lost if Vector is restarted forcefully.","disk":"Stores the sink's buffer on disk. This is less performant, but durable. Data will not be lost between restarts."}}
  examples={["memory","disk"]}
  name={"type"}
  path={"buffer"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

#### type

The buffer's type and storage mechanism.


</Field>


<Field
  common={false}
  defaultValue={"block"}
  enumValues={{"block":"Applies back pressure when the buffer is full. This prevents data loss, but will cause data to pile up on the edge.","drop_newest":"Drops new data as it's received. This data is lost. This should be used when performance is the highest priority."}}
  examples={["block","drop_newest"]}
  name={"when_full"}
  path={"buffer"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

#### when_full

The behavior when the buffer becomes full.


</Field>


</Fields>

</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={{"json":"Each event is encoded into JSON and the payload is represented as a JSON array.","text":"Each event is encoded into text via the `message` key and the payload is new line delimited."}}
  examples={["json","text"]}
  name={"encoding"}
  path={null}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

### encoding

The encoding format used to serialize the events before outputting.


</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={["127.0.0.0:5000/path/to/service"]}
  name={"endpoint"}
  path={null}
  relevantWhen={{"region":""}}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

### endpoint

Custom endpoint for use with AWS-compatible services. Providing a value for this option will make [`region`](#region) moot.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["user_id"]}
  name={"partition_key_field"}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

### partition_key_field

The log field used as the Kinesis record's partition key value. See [Partitioning](#partitioning) for more info.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["us-east-1"]}
  name={"region"}
  path={null}
  relevantWhen={{"host":""}}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

### region

The [AWS region][urls.aws_regions] of the target service. If [`endpoint`](#endpoint) is provided it will override this value since the endpoint includes the region.


</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"request"}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"table"}
  unit={null}
  >

### request

Configures the sink request behavior.

<Fields filters={false}>


<Field
  common={false}
  defaultValue={5}
  enumValues={null}
  examples={[5]}
  name={"in_flight_limit"}
  path={"request"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"int"}
  unit={"requests"}
  >

#### in_flight_limit

The maximum number of in-flight requests allowed at any given time. See [Rate Limits](#rate-limits) for more info.


</Field>


<Field
  common={false}
  defaultValue={1}
  enumValues={null}
  examples={[1]}
  name={"rate_limit_duration_secs"}
  path={"request"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"int"}
  unit={"seconds"}
  >

#### rate_limit_duration_secs

The time window, in seconds, used for the [`rate_limit_num`](#rate_limit_num) option. See [Rate Limits](#rate-limits) for more info.


</Field>


<Field
  common={false}
  defaultValue={5}
  enumValues={null}
  examples={[5]}
  name={"rate_limit_num"}
  path={"request"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"int"}
  unit={null}
  >

#### rate_limit_num

The maximum number of requests allowed within the [`rate_limit_duration_secs`](#rate_limit_duration_secs) time window. See [Rate Limits](#rate-limits) for more info.


</Field>


<Field
  common={false}
  defaultValue={-1}
  enumValues={null}
  examples={[-1]}
  name={"retry_attempts"}
  path={"request"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"int"}
  unit={null}
  >

#### retry_attempts

The maximum number of retries to make for failed requests. See [Retry Policy](#retry-policy) for more info.


</Field>


<Field
  common={false}
  defaultValue={1}
  enumValues={null}
  examples={[1]}
  name={"retry_initial_backoff_secs"}
  path={"request"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"int"}
  unit={"seconds"}
  >

#### retry_initial_backoff_secs

The amount of time to wait before attempting the first retry for a failed request. Once, the first retry has failed the fibonacci sequence will be used to select future backoffs.


</Field>


<Field
  common={false}
  defaultValue={10}
  enumValues={null}
  examples={[10]}
  name={"retry_max_duration_secs"}
  path={"request"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"int"}
  unit={"seconds"}
  >

#### retry_max_duration_secs

The maximum amount of time, in seconds, to wait between retries.


</Field>


<Field
  common={false}
  defaultValue={30}
  enumValues={null}
  examples={[30]}
  name={"timeout_secs"}
  path={"request"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"int"}
  unit={"seconds"}
  >

#### timeout_secs

The maximum time a request can take before being aborted. It is highly recommended that you do not lower value below the service's internal timeout, as this could create orphaned requests, pile on retries, and result in duplicate data downstream. See [Buffers & Batches](#buffers--batches) for more info.


</Field>


</Fields>

</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["my-stream"]}
  name={"stream_name"}
  path={null}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

### stream_name

The [stream name][urls.aws_cw_logs_stream_name] of the target Kinesis Logs stream.


</Field>


</Fields>

## Env Vars

<Fields filters={true}>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={["AKIAIOSFODNN7EXAMPLE"]}
  name={"AWS_ACCESS_KEY_ID"}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

### AWS_ACCESS_KEY_ID

Used for AWS authentication when communicating with AWS services. See relevant [AWS components][pages.aws_components] for more info. See [AWS Authentication](#aws-authentication) for more info.


</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={["wJalrXUtnFEMI/K7MDENG/FD2F4GJ"]}
  name={"AWS_SECRET_ACCESS_KEY"}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

### AWS_SECRET_ACCESS_KEY

Used for AWS authentication when communicating with AWS services. See relevant [AWS components][pages.aws_components] for more info. See [AWS Authentication](#aws-authentication) for more info.


</Field>


</Fields>

## Output

The `aws_kinesis_streams` sink [batches](#buffers--batches) [`log`][docs.data-model.log] events to [Amazon Web Service's Kinesis Data Stream service][urls.aws_kinesis_data_streams] via the [`PutRecords` API endpoint](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecords.html).
Batches are flushed via the [`batch_size`](#batch_size) or
[`batch_timeout`](#batch_timeout) options. You can learn more in the [buffers &
batches](#buffers--batches) section.
For example:


```http
POST / HTTP/1.1
Host: kinesis.<region>.<domain>
Content-Length: <byte_size>
Content-Type: application/x-amz-json-1.1
Connection: Keep-Alive
X-Amz-Target: Kinesis_20131202.PutRecords
{
    "Records": [
        {
            "Data": "<json_encoded_log>",
            "PartitionKey": "<partition_key>"
        },
        {
            "Data": "<json_encoded_log>",
            "PartitionKey": "<partition_key>"
        },
        {
            "Data": "<json_encoded_log>",
            "PartitionKey": "<partition_key>"
        },
    ],
    "StreamName": "<stream_name>"
}
```

## How It Works

### AWS Authentication

Vector checks for AWS credentials in the following order:

1. Environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`.
2. The [`credential_process` command][urls.aws_credential_process] in the AWS config file. (usually located at `~/.aws/config`)
3. The [AWS credentials file][urls.aws_credentials_file]. (usually located at `~/.aws/credentials`)
4. The [IAM instance profile][urls.iam_instance_profile]. (will only work if running on an EC2 instance with an instance profile/role)

If credentials are not found the [healtcheck](#healthchecks) will fail and an
error will be [logged][docs.monitoring#logs].

#### Obtaining an access key

In general, we recommend using instance profiles/roles whenever possible. In
cases where this is not possible you can generate an AWS access key for any user
within your AWS account. AWS provides a [detailed guide][urls.aws_access_keys] on
how to do this.

#### Assuming Roles

Vector can assume an AWS IAM role via the [`assume_role`](#assume_role) option. This is an
optional setting that is helpful for a variety of use cases, such as cross
account access.
### Buffers & Batches

import SVG from 'react-inlinesvg';

<SVG src="/img/buffers-and-batches-serial.svg" />

The `aws_kinesis_streams` sink buffers & batches data as
shown in the diagram above. You'll notice that Vector treats these concepts
differently, instead of treating them as global concepts, Vector treats them
as sink specific concepts. This isolates sinks, ensuring services disruptions
are contained and [delivery guarantees][docs.guarantees] are honored.

*Batches* are flushed when 1 of 2 conditions are met:

1. The batch age meets or exceeds the configured [`timeout_secs`](#timeout_secs).
2. The batch size meets or exceeds the configured [`max_events`](#max_events).

*Buffers* are controlled via the [`buffer.*`](#buffer) options.

### Environment Variables

Environment variables are supported through all of Vector's configuration.
Simply add `${MY_ENV_VAR}` in your Vector configuration file and the variable
will be replaced before being evaluated.

You can learn more in the [Environment Variables][docs.configuration#environment-variables]
section.

### Health Checks

Health checks ensure that the downstream service is accessible and ready to
accept data. This check is performed upon sink initialization.
If the health check fails an error will be logged and Vector will proceed to
start.

#### Require Health Checks

If you'd like to exit immediately upon a health check failure, you can
pass the `--require-healthy` flag:

```bash
vector --config /etc/vector/vector.toml --require-healthy
```

#### Disable Health Checks

If you'd like to disable health checks for this sink you can set the
`healthcheck` option to `false`.

### Partitioning

By default, Vector issues random 16 byte values for each
[Kinesis record's partition key][urls.aws_kinesis_partition_key], evenly
distributing records across your Kinesis partitions. Depending on your use case
this might not be sufficient since random distribution does not preserve order.
To override this, you can supply the [`partition_key_field`](#partition_key_field) option. This option
represents a field on your event to use for the partition key value instead.
This is useful if you have a field already on your event, and it also pairs
nicely with the [`add_fields` transform][docs.transforms.add_fields].

#### Missing keys or blank values

Kenisis requires a value for the partition key and therefore if the key is
missing or the value is blank the event will be dropped and a
[`warning` level log event][docs.monitoring#logs] will be logged. As such,
the field specified in the [`partition_key_field`](#partition_key_field) option should always contain
a value.

#### Values that exceed 256 characters

If the value provided exceeds the maximum allowed length of 256 characters
Vector will slice the value and use the first 256 characters.

#### Non-string values

Vector will coerce the value into a string.

#### Provisioning & capacity planning

This is generally outside the scope of Vector but worth touching on. When you
supply your own partition key it opens up the possibility for "hot spots",
and you should be aware of your data distribution for the key you're providing.
Kinesis provides the ability to
[manually split shards][urls.aws_kinesis_split_shards] to accomodate this.
If they key you're using is dynamic and unpredictable we highly recommend
recondsidering your ordering policy to allow for even and random distribution.



### Rate Limits

Vector offers a few levers to control the rate and volume of requests to the
downstream service. Start with the [`rate_limit_duration_secs`](#rate_limit_duration_secs) and
`rate_limit_num` options to ensure Vector does not exceed the specified
number of requests in the specified window. You can further control the pace at
which this window is saturated with the [`in_flight_limit`](#in_flight_limit) option, which
will guarantee no more than the specified number of requests are in-flight at
any given time.

Please note, Vector's defaults are carefully chosen and it should be rare that
you need to adjust these. If you found a good reason to do so please share it
with the Vector team by [opening an issie][urls.new_aws_kinesis_streams_sink_issue].

### Retry Policy

Vector will retry failed requests (status == `429`, >= `500`, and != `501`).
Other responses will _not_ be retried. You can control the number of retry
attempts and backoff rate with the [`retry_attempts`](#retry_attempts) and
`retry_backoff_secs` options.


[docs.configuration#environment-variables]: /docs/setup/configuration/#environment-variables
[docs.data-model.log]: /docs/about/data-model/log/
[docs.data-model]: /docs/about/data-model/
[docs.guarantees]: /docs/about/guarantees/
[docs.monitoring#logs]: /docs/administration/monitoring/#logs
[docs.transforms.add_fields]: /docs/reference/transforms/add_fields/
[pages.aws_components]: /components?providers%5B%5D=aws/
[urls.aws_access_keys]: https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html
[urls.aws_credential_process]: https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sourcing-external.html
[urls.aws_credentials_file]: https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html
[urls.aws_cw_logs_stream_name]: https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/Working-with-log-groups-and-streams.html
[urls.aws_iam_role]: https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html
[urls.aws_kinesis_data_streams]: https://aws.amazon.com/kinesis/data-streams/
[urls.aws_kinesis_partition_key]: https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecordsRequestEntry.html#Streams-Type-PutRecordsRequestEntry-PartitionKey
[urls.aws_kinesis_split_shards]: https://docs.aws.amazon.com/streams/latest/dev/kinesis-using-sdk-java-resharding-split.html
[urls.aws_regions]: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.RegionsAndAvailabilityZones.html
[urls.iam_instance_profile]: https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_switch-role-ec2_instance-profiles.html
[urls.new_aws_kinesis_streams_sink_issue]: https://github.com/timberio/vector/issues/new?labels=sink%3A+aws_kinesis_streams
