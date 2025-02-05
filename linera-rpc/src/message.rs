// Copyright (c) Facebook, Inc. and its affiliates.
// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_base::identifiers::ChainId;
use linera_chain::data_types::{BlockProposal, LiteVote};
use linera_core::{
    data_types::{ChainInfoQuery, ChainInfoResponse, CrossChainRequest},
    node::NodeError,
};
use linera_version::VersionInfo;
use serde::{Deserialize, Serialize};

use crate::{HandleCertificateRequest, HandleLiteCertRequest};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(with_testing, derive(Eq, PartialEq))]
pub enum RpcMessage {
    // Inbound
    BlockProposal(Box<BlockProposal>),
    Certificate(Box<HandleCertificateRequest>),
    LiteCertificate(Box<HandleLiteCertRequest<'static>>),
    ChainInfoQuery(Box<ChainInfoQuery>),
    VersionInfoQuery,

    // Outbound
    Vote(Box<LiteVote>),
    ChainInfoResponse(Box<ChainInfoResponse>),
    Error(Box<NodeError>),
    VersionInfoResponse(Box<VersionInfo>),

    // Internal to a validator
    CrossChainRequest(Box<CrossChainRequest>),
}

impl RpcMessage {
    /// Obtains the [`ChainId`] of the chain targeted by this message, if there is one.
    ///
    /// Only inbound messages have target chains.
    pub fn target_chain_id(&self) -> Option<ChainId> {
        use RpcMessage::*;

        let chain_id = match self {
            BlockProposal(proposal) => proposal.content.block.chain_id,
            LiteCertificate(request) => request.certificate.value.chain_id,
            Certificate(request) => request.certificate.value().chain_id(),
            ChainInfoQuery(query) => query.chain_id,
            CrossChainRequest(request) => request.target_chain_id(),
            Vote(_)
            | Error(_)
            | ChainInfoResponse(_)
            | VersionInfoQuery
            | VersionInfoResponse(_) => {
                return None;
            }
        };

        Some(chain_id)
    }
}

impl TryFrom<RpcMessage> for ChainInfoResponse {
    type Error = NodeError;
    fn try_from(message: RpcMessage) -> Result<Self, Self::Error> {
        use RpcMessage::*;
        match message {
            ChainInfoResponse(response) => Ok(*response),
            Error(error) => Err(*error),
            _ => Err(NodeError::UnexpectedMessage),
        }
    }
}

impl TryFrom<RpcMessage> for VersionInfo {
    type Error = NodeError;
    fn try_from(message: RpcMessage) -> Result<Self, Self::Error> {
        use RpcMessage::*;
        match message {
            VersionInfoResponse(version_info) => Ok(*version_info),
            Error(error) => Err(*error),
            _ => Err(NodeError::UnexpectedMessage),
        }
    }
}

impl From<BlockProposal> for RpcMessage {
    fn from(block_proposal: BlockProposal) -> Self {
        RpcMessage::BlockProposal(Box::new(block_proposal))
    }
}

impl From<HandleLiteCertRequest<'static>> for RpcMessage {
    fn from(request: HandleLiteCertRequest<'static>) -> Self {
        RpcMessage::LiteCertificate(Box::new(request))
    }
}

impl From<HandleCertificateRequest> for RpcMessage {
    fn from(request: HandleCertificateRequest) -> Self {
        RpcMessage::Certificate(Box::new(request))
    }
}

impl From<ChainInfoQuery> for RpcMessage {
    fn from(chain_info_query: ChainInfoQuery) -> Self {
        RpcMessage::ChainInfoQuery(Box::new(chain_info_query))
    }
}

impl From<LiteVote> for RpcMessage {
    fn from(vote: LiteVote) -> Self {
        RpcMessage::Vote(Box::new(vote))
    }
}

impl From<ChainInfoResponse> for RpcMessage {
    fn from(chain_info_response: ChainInfoResponse) -> Self {
        RpcMessage::ChainInfoResponse(Box::new(chain_info_response))
    }
}

impl From<NodeError> for RpcMessage {
    fn from(error: NodeError) -> Self {
        RpcMessage::Error(Box::new(error))
    }
}

impl From<CrossChainRequest> for RpcMessage {
    fn from(cross_chain_request: CrossChainRequest) -> Self {
        RpcMessage::CrossChainRequest(Box::new(cross_chain_request))
    }
}

impl From<VersionInfo> for RpcMessage {
    fn from(version_info: VersionInfo) -> Self {
        RpcMessage::VersionInfoResponse(Box::new(version_info))
    }
}
