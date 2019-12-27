#[macro_use] extern crate clap;
#[macro_use] extern crate prettytable;
extern crate yaml_rust;
extern crate rusoto_core;
extern crate rusoto_ssm;

use std::process::{Command};
use clap::{App, AppSettings};
use prettytable::{Table, format};
use rusoto_core::{Region};
use rusoto_ssm::{Ssm, SsmClient, InstanceInformation, DescribeInstanceInformationRequest};

#[derive(Debug)]
struct InstanceData {
    name:  std::string::String,
    id:    std::string::String,
    ip:    std::string::String,
}

fn parse_region(s: &str) -> Result<Region, Region> {
    let v : &str = &s.to_lowercase();
    match v {
        "ap-northeast-1" | "apnortheast1" => Ok(Region::ApNortheast1),
        "ap-northeast-2" | "apnortheast2" => Ok(Region::ApNortheast2),
        "ap-south-1" | "apsouth1" => Ok(Region::ApSouth1),
        "ap-southeast-1" | "apsoutheast1" => Ok(Region::ApSoutheast1),
        "ap-southeast-2" | "apsoutheast2" => Ok(Region::ApSoutheast2),
        "ca-central-1" | "cacentral1" => Ok(Region::CaCentral1),
        "eu-central-1" | "eucentral1" => Ok(Region::EuCentral1),
        "eu-west-1" | "euwest1" => Ok(Region::EuWest1),
        "eu-west-2" | "euwest2" => Ok(Region::EuWest2),
        "eu-west-3" | "euwest3" => Ok(Region::EuWest3),
        "sa-east-1" | "saeast1" => Ok(Region::SaEast1),
        "us-east-1" | "useast1" => Ok(Region::UsEast1),
        "us-east-2" | "useast2" => Ok(Region::UsEast2),
        "us-west-1" | "uswest1" => Ok(Region::UsWest1),
        "us-west-2" | "uswest2" => Ok(Region::UsWest2),
        "us-gov-west-1" | "usgovwest1" => Ok(Region::UsGovWest1),
        "cn-north-1" | "cnnorth1" => Ok(Region::CnNorth1),
        "cn-northwest-1" | "cnnorthwest1" => Ok(Region::CnNorthwest1),
        _s => Err(Region::UsEast1),
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).setting(AppSettings::ArgRequiredElseHelp).get_matches();
    
    let region_val = matches.value_of("region").unwrap_or("us-east-1");
    let region_result = parse_region(region_val);
    let region = match region_result {
        Ok(v) => v,
        Err(e) => e,
    };

    let ssm_client = SsmClient::new(region.clone());

    let mut instances_req = DescribeInstanceInformationRequest {
        ..Default::default()
    };

    let mut instances_resp = ssm_client.describe_instance_information(instances_req.clone()).sync().ok().unwrap();
    let mut instances: Vec<InstanceInformation> = Vec::new();

    loop {
        for i in instances_resp.instance_information_list.clone().into_iter() {
            instances.extend(i);
        }

        if instances_resp.next_token.is_none() { break; }

        instances_req = DescribeInstanceInformationRequest {
            next_token: instances_resp.next_token,
            ..Default::default()
        };

        instances_resp = ssm_client.describe_instance_information(instances_req).sync().ok().unwrap();
    }

    if matches.is_present("list") {
        let mut instance_data: Vec<InstanceData> = Vec::new();

        for x in instances.iter() {
            instance_data.push(
                InstanceData {
                    name: x.computer_name.as_ref().unwrap().to_string(),
                    id: x.instance_id.as_ref().unwrap().to_string(),
                    ip: x.ip_address.as_ref().unwrap().to_string()
                }
            )
        };

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row!["Name", "Instance ID", "IP Address"]);
    
        for (y, _z) in instance_data.iter().enumerate() {
            table.add_row(row![    
                instance_data[y as usize].name,
                instance_data[y as usize].id,
                instance_data[y as usize].ip
            ]);
        };
        table.printstd();
    } else if matches.is_present("INSTANCE") {
        for x in instances.iter() {
            if [ x.computer_name.as_ref().unwrap().to_string(),
                 x.instance_id.as_ref().unwrap().to_string(),
                 x.ip_address.as_ref().unwrap().to_string()
            ].contains(&matches.value_of("INSTANCE").unwrap().to_string()) {
                let _session = Command::new("aws")
                    .arg("ssm")
                    .arg("start-session")
                    .arg("--target")
                    .arg(x.instance_id.as_ref().unwrap().to_string())
                    .status()
                    .expect("failed to open session");
            }
        };
    } else {
        println!("exit");
    }
}
