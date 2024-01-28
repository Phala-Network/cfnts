// This file is part of cfnts.
// Copyright (c) 2019, Cloudflare. All rights reserved.
// See LICENSE for licensing information.

//! The client subcommand.

use anyhow::{Context, Result};

use log::debug;

use crate::ntp::client::{run_nts_ntp_client, NtpResult};
use crate::nts_ke::client::{run_nts_ke_client, ClientConfig};

async fn get_time_ip_ver(host: &str, port: Option<u16>, use_ipv6: bool) -> Result<NtpResult> {
    let config = ClientConfig {
        host: host.into(),
        port,
        use_ipv6,
    };
    let state = run_nts_ke_client(config)
        .await
        .context("failed to handshake")?;
    debug!("handshake fine");
    run_nts_ntp_client(state)
        .await
        .context("failed to get time")
}

/// get_time gets the time from given NTS server.
pub async fn get_time(host: &str, port: Option<u16>) -> Result<NtpResult> {
    let v4_result = get_time_ip_ver(host, port, false).await;
    if v4_result.is_ok() {
        return v4_result;
    }
    let v6_result = get_time_ip_ver(host, port, true).await;
    if v6_result.is_ok() {
        return v6_result;
    }
    v4_result
}

#[tokio::test]
async fn it_works() {
    let result = get_time("time.cloudflare.com", None).await.unwrap();
    assert!(result.time_diff < 10.);
}
