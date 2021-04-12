//! This is the testing module for arbitrary logic functionality. This is where instead of managing transfers directly the bridge simply passes an
//! arbitrary call to an arbitrary sub contract along with a specific amount of funds, allowing for execution of whatever command is required

use crate::{get_test_token_name, utils::ValidatorKeys};
use crate::{COSMOS_NODE_GRPC, TOTAL_TIMEOUT};
use actix::Arbiter;
use clarity::Address as EthAddress;
use contact::client::Contact;
use gravity_proto::gravity::query_client::QueryClient as GravityQueryClient;
use orchestrator::main_loop::orchestrator_main_loop;
use tokio::time::delay_for;
use tonic::transport::Channel;
use web30::client::Web3;

pub async fn arbitrary_logic_test(
    web30: &Web3,
    grpc_client: GravityQueryClient<Channel>,
    contact: &Contact,
    keys: Vec<ValidatorKeys>,
    gravity_address: EthAddress,
) {
    // start orchestrators
    #[allow(clippy::explicit_counter_loop)]
    for k in keys.iter() {
        info!("Spawning Orchestrator");
        let grpc_client = GravityQueryClient::connect(COSMOS_NODE_GRPC).await.unwrap();
        // we have only one actual futures executor thread (see the actix runtime tag on our main function)
        // but that will execute all the orchestrators in our test in parallel
        Arbiter::spawn(orchestrator_main_loop(
            k.validator_key,
            k.eth_key,
            web30.clone(),
            contact.clone(),
            grpc_client,
            gravity_address,
            get_test_token_name(),
        ));
    }

    delay_for(TOTAL_TIMEOUT).await;
}
