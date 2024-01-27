use atlas_communication::reconfiguration_node::NetworkInformationProvider;

use atlas_core::ordering_protocol::networking::serialize::{OrderingProtocolMessage, OrderProtocolVerificationHelper};

pub trait LogTransferVerificationHelper<RQ, OP, NI>: OrderProtocolVerificationHelper<RQ, OP, NI>
    where OP: OrderingProtocolMessage<RQ>, NI: NetworkInformationProvider {}
