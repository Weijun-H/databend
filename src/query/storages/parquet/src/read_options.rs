// Copyright 2023 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Copy, Clone, Debug, Default)]
pub struct ReadOptions {
    /// Prune row groups before reading. Require Chunk level statistics.
    /// Filter row groups don't need to read.
    prune_row_groups: bool,
    /// Prune pages before reading. Require Page level statistics.
    /// Filter rows don't need to read.
    prune_pages: bool,
    /// If use prewhere filter.
    do_prewhere: bool,
    /// If push down bitmap generated by prewhere reader to remain reader.
    /// If true, when remain reader deserializing,
    /// it will skip part of decompression and decoding according to the bitmap.
    ///
    /// Notice:
    ///
    /// - `push_down_bitmap` and  `prune_pages` are exclusive. (`push_down_bitmap && prune_pages == false`)
    /// - If `do_prewhere` is disabled, `push_down_bitmap` is useless.
    push_down_bitmap: bool,
}

impl ReadOptions {
    #[inline]
    pub fn new() -> Self {
        ReadOptions::default()
    }

    #[inline]
    pub fn with_prune_row_groups(mut self) -> Self {
        self.prune_row_groups = true;
        self
    }

    #[inline]
    pub fn with_prune_pages(mut self) -> Self {
        self.prune_pages = true;
        self.push_down_bitmap = false;
        self
    }

    #[inline]
    pub fn with_push_down_bitmap(mut self) -> Self {
        self.push_down_bitmap = true;
        self.prune_pages = false;
        self
    }

    #[inline]
    pub fn with_do_prewhere(mut self) -> Self {
        self.do_prewhere = true;
        self
    }

    #[inline]
    pub fn prune_row_groups(&self) -> bool {
        self.prune_row_groups
    }

    #[inline]
    pub fn prune_pages(&self) -> bool {
        self.prune_pages
    }

    #[inline]
    pub fn push_down_bitmap(&self) -> bool {
        self.push_down_bitmap
    }

    #[inline]
    pub fn do_prewhere(&self) -> bool {
        self.do_prewhere
    }
}