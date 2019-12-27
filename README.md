# hatch

`hatch` is a tool for accessing EC2 instances managed by AWS Systems Manager (via the Session Manager feature).

## Download

You can download a prebuilt binary [here](https://github.com/slapula/hatch/releases).

## Prerequisites

`hatch` assumes you have [awscli](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html) installed locally and properly configured.  It also assumes you have the [session-manager-plugin](https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager-working-with-install-plugin.html) installed as well.

## Help

```
USAGE:
    hatch [FLAGS] [OPTIONS] [INSTANCE]

FLAGS:
    -h, --help       Prints help information
    -l, --list       List instances available for remote sessions via AWS Systems Manager
    -V, --version    Prints version information

OPTIONS:
    -r, --region <STRING>    AWS Region (Default: 'us-east-1')

ARGS:
    <INSTANCE>    Name, ID, or IP address of target EC2 instance
```

### Getting Started

You can view a list of instances managed by AWS Systems Manager by running `hatch -l`:
```
+-------------------------------+---------------------+----------------+
| Name                          | Instance ID         | IP Address     |
+-------------------------------+---------------------+----------------+
| ssm-example-1                 | i-1293d51299c7aa4b4 | 10.236.223.65  |
| ssm-example-2                 | i-918adf735b3ccd5a8 | 10.47.100.197  |
| ssm-example-3                 | i-51aaaf17afc1a2532 | 10.134.93.15   |
| ssm-example-4                 | i-46afc19573dca72a3 | 10.215.192.54  |
| ssm-example-5                 | i-10bac3cd174f72ec7 | 10.145.16.189  |
+-------------------------------+---------------------+----------------+
```
You can use all three identifiers (Names, Instance IDs, and IP addresses) to access a given EC2 instance:
```
$ hatch ssm-example-1

Starting session with SessionId: examp-15262b9a4187dc69a
sh-4.2$ cat /etc/hostname 
ssm-example-1
sh-4.2$ exit
exit


Exiting session with sessionId: examp-15262b9a4187dc69a.

```

## NOTE

I'm new to Rust so if you would like to report a bug, submit a feature request, or just contribute please be my guest!