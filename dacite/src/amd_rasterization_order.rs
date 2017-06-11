// Copyright (c) 2017, Dennis Hamester <dennis.hamester@startmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

//! See extension [`VK_AMD_rasterization_order`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_rasterization_order)

use vks;

/// See [`VkRasterizationOrderAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRasterizationOrderAMD)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RasterizationOrderAmd {
    Strict,
    Relaxed,
    Unknown(vks::VkRasterizationOrderAMD),
}

impl From<RasterizationOrderAmd> for vks::VkRasterizationOrderAMD {
    fn from(order: RasterizationOrderAmd) -> Self {
        match order {
            RasterizationOrderAmd::Strict => vks::VK_RASTERIZATION_ORDER_STRICT_AMD,
            RasterizationOrderAmd::Relaxed => vks::VK_RASTERIZATION_ORDER_RELAXED_AMD,
            RasterizationOrderAmd::Unknown(order) => order,
        }
    }
}

gen_chain_struct! {
    name: PipelineRasterizationStateRasterizationOrderChainAmd [PipelineRasterizationStateRasterizationOrderChainAmdWrapper],
    query: PipelineRasterizationStateRasterizationOrderChainQueryAmd [PipelineRasterizationStateRasterizationOrderChainQueryAmdWrapper],
    vks: VkPipelineRasterizationStateRasterizationOrderAMD,
    input: true,
    output: false,
}

/// See [`VkPipelineRasterizationStateRasterizationOrderAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateRasterizationOrderAMD)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineRasterizationStateRasterizationOrderAmd {
    pub rasterization_order: RasterizationOrderAmd,
    pub chain: Option<PipelineRasterizationStateRasterizationOrderChainAmd>,
}

#[derive(Debug)]
pub(crate) struct VkPipelineRasterizationStateRasterizationOrderAMDWrapper {
    pub vks_struct: vks::VkPipelineRasterizationStateRasterizationOrderAMD,
    chain: Option<PipelineRasterizationStateRasterizationOrderChainAmdWrapper>,
}

impl VkPipelineRasterizationStateRasterizationOrderAMDWrapper {
    pub fn new(order: &PipelineRasterizationStateRasterizationOrderAmd, with_chain: bool) -> Self {
        let (pnext, chain) = PipelineRasterizationStateRasterizationOrderChainAmdWrapper::new_optional(&order.chain, with_chain);

        VkPipelineRasterizationStateRasterizationOrderAMDWrapper {
            vks_struct: vks::VkPipelineRasterizationStateRasterizationOrderAMD {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
                pNext: pnext,
                rasterizationOrder: order.rasterization_order.into(),
            },
            chain: chain,
        }
    }
}
